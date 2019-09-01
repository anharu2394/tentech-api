use crate::db;
use crate::error::TentechError;
use crate::models::user::TokenData;
use crate::schema::users;
use crate::validation::FieldValidator;
use lazy_static::lazy_static;
use percent_encoding::percent_decode_str;
use regex::Regex;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}
lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new(r"\A[a-z0-9_]{1,15}\z").unwrap();
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(regex = "USERNAME_REGEX", length(min = 1, max = 15))]
    username: Option<String>,
    #[validate(length(min = 1, max = 50))]
    nickname: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    user: UpdateUserData,
}
#[derive(Deserialize, Validate, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUserData {
    #[validate(regex = "USERNAME_REGEX", length(min = 1, max = 15))]
    username: String,
    #[validate(length(min = 1, max = 50))]
    nickname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = "8"))]
    password: String,
    #[validate(url)]
    avatar: Option<String>,
    bio: String,
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

#[post("/users/<id>", format = "json", data = "<update_user>")]
pub fn update_users(
    update_user: Json<UpdateUser>,
    conn: db::Conn,
    id: i32,
) -> Result<JsonValue, TentechError> {
    let update_user = update_user.into_inner().user;
    update_user
        .validate()
        .map_err(|e| TentechError::ValidationFailed(e))?;

    db::users::update(&conn, &id, &update_user)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
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
    Ok(json!({ "token": token, "user": target }))
}

#[post("/users/resend")]
pub fn resend(token: TokenData, conn: db::Conn) -> Result<JsonValue, TentechError> {
    token
        .user
        .prepare_activate()
        .map_err(|_| TentechError::CannotSendEmail)?;
    Ok(json!({}))
}

#[get("/users/validate")]
pub fn validate(token: TokenData, conn: db::Conn) -> Result<JsonValue, TentechError> {
    db::users::find(&conn, &token.user.id)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|u| json!({ "user": u }))
}

#[get("/users/<username>")]
pub fn get(username: String, conn: db::Conn) -> Result<JsonValue, TentechError> {
    db::users::find_by_username(&conn, &username)
        .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))
        .map(|u| json!({ "user": u }))
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
