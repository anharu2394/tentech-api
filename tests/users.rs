mod common;

use common::*;
use rocket::http::{ContentType, Status};

fn setup() {
    delete_all_users();
}

#[test]
fn post_users() {
    setup();
    let client = test_client();
    let mut res = client
        .post("/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"username": "create_test", "nickname": "create_test", "email": "create_test@test.com", "password": PASSWORD}}))
        .dispatch();
    print_response(&mut res);
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn post_same_username() {
    before();
    create_user("same_user");
    let client = test_client();
    let mut res = client
        .post("/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": user_json("same_user")}))
        .dispatch();
    assert_eq!(res.status(), Status::Conflict);
    assert_eq!(res.body_string(),Some("{\"message\":\"duplicate key value violates unique constraint \\\"users_username_key\\\"\",\"type\":\"DatabaseFailed\"}".to_string()));
}

#[test]
fn post_same_email() {
    before();
    create_user("same_email");
    let client = test_client();
    let mut res = client
        .post("/users")
        .header(ContentType::JSON)
        .body(json_string!({"user": {"username": "different", "nickname": "same_email_name", "email":"same_email_test@test.com", "password": PASSWORD}}))
        .dispatch();
    assert_eq!(res.status(), Status::Conflict);
    assert_eq!(res.body_string(),Some("{\"message\":\"duplicate key value violates unique constraint \\\"users_email_key\\\"\",\"type\":\"DatabaseFailed\"}".to_string()));

}

#[test]
fn login() {
    before();
    create_user("login");
    let client = test_client();
    let mut res = client
        .post("/users/login")
        .header(ContentType::JSON)
        .body(json_string!({"email": "login_test@test.com", "password": PASSWORD}))
        .dispatch();
    print_response(&mut res);
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn get_user() {
    before();
    create_user("get");
    let client = test_client();
    let mut res = client
        .get("/users/get_test")
        .dispatch();
    print_response(&mut res);
    assert_eq!(res.status(), Status::Ok);
}