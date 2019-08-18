use crate::db::{self, users::UserCreationError};
use crate::email::SendError;
use crate::error::TentechError;
use crate::models::user::TokenData;
use crate::validation::FieldValidator;
use percent_encoding::percent_decode_str;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde_json;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = "1"))]
    username: Option<String>,
    #[validate(length(min = "1"))]
    nickname: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = "8"))]
    password: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<JsonValue, TentechError> {
    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let nickname = extractor.extract("nickname", new_user.nickname);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor
        .check()
        .map_err(|e| TentechError::ValidationFailed(e.errors))?;

    // In create method, convert a password into a hash value. no worries.
    db::users::create(&conn, &username, &nickname, &email, &password)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .and_then(|user| {
            user.prepare_activate()
                .map_err(|_| TentechError::CannotSendEmail)
        })
        .map(|user| json!({ "user": user }))
}

#[get("/users/activate?<token>")]
pub fn activate(token: String, conn: db::Conn) -> Result<JsonValue, TentechError> {
    let url_decoded_token = percent_decode_str(&token)
        .decode_utf8()
        .map_err(|_| TentechError::CannotDecryptToken)?
        .to_string();
    let token_data =
        TokenData::decode(url_decoded_token).map_err(|_| TentechError::CannotDecryptToken)?;
    let target = db::users::find(&conn, &token_data.user.id)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    if target.activated {
        return Err(TentechError::AlreadyActivated);
    }
    db::users::activate(&conn, &token_data.user)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;

    if !token_data.check_expired() {
        return Err(TentechError::TokenExpired);
    }

    Ok(json!(token_data))
}

#[post("/users/login", format = "json", data = "<login_user>")]
pub fn login(login_user: Json<LoginUser>, conn: db::Conn) -> Result<JsonValue, TentechError> {
    let login_user = login_user.into_inner();
    let target = db::users::login(&conn, &login_user.email, &login_user.password)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?;
    let token = target.generate_token();
    Ok(json!({ "token": token }))
}

#[get("/users/<id>")]
pub fn get(id: i32, token: TokenData) -> String {
    "Ok".to_string()
}
#[cfg(test)]
mod test {
    use crate::db;
    use crate::rocket;
    use crate::test_establish_connection;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    fn setup() {
        db::users::delete_all(&test_establish_connection());
    }
    #[test]
    fn post_users() {
        setup();
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body("{\"user\": {\"username\": \"anharu2394\", \"nickname\": \"anharu\", \"email\": \"email@test.com\", \"password\": \"passpassword\"}}")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
