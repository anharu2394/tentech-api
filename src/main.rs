use tentech_api;

fn main() {
    match tentech_api::db::tags::init(&tentech_api::establish_connection()) {
        Err(e) => println!("{:?}", e),
        _ => {}
    }
    tentech_api::rocket().launch();
}
