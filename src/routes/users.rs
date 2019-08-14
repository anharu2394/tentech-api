use crate::db::{self, users::UserCreationError};
use crate::email::SendError;
use crate::errors::{Errors, FieldValidator};
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
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

#[post("/users", format = "json", data = "<new_user>")]
pub fn post_users(new_user: Json<NewUser>, conn: db::Conn) -> Result<JsonValue, Errors> {
    let new_user = new_user.into_inner().user;

    let mut extractor = FieldValidator::validate(&new_user);
    let username = extractor.extract("username", new_user.username);
    let nickname = extractor.extract("nickname", new_user.nickname);
    let email = extractor.extract("email", new_user.email);
    let password = extractor.extract("password", new_user.password);

    extractor.check()?;

    // In create method, convert a password into a hash value. no worries.
    db::users::create(&conn, &username, &nickname, &email, &password)
        .and_then(|user| {
            user.prepare_activate()
                .map_err(|_| UserCreationError::DuplicatedEmail)
        })
        .map(|user| json!({ "user": user }))
        .map_err(|error| {
            let field = match error {
                UserCreationError::DuplicatedEmail => "email",
                UserCreationError::DuplicatedUsername => "username",
            };
            Errors::new(&[(field, "has already been taken")])
        })
}

#[get("/users/activate?<token>")]
pub fn activate(token: String) -> JsonValue {
    json!({ "ok": token })
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
