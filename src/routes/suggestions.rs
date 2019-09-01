use crate::db;
use crate::error::TentechError;
use crate::models::product::Product;
use crate::models::suggestion::Suggestion;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct SuggestionForm {
    pub long: i32,
    pub lang: String,
    pub kind: String,
}

#[post("/suggestion", format = "json", data = "<data>")]
pub fn suggestion(conn: db::Conn, data: Json<SuggestionForm>) -> Result<JsonValue, TentechError> {
    let data = data.into_inner();
    let mut suggestion = Suggestion {
        title: "Ruby".to_string(),
        body: "Ruby on RailsでTodoアプリを作るのはどうですか？".to_string(),
        learning_url: Vec::new(),
        working_url: Vec::new(),
        products: Vec::new(),
    };

    let mut file = File::open("data/suggestion.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let menu: Value = serde_json::from_str(&contents).unwrap();
    if data.lang != "others" && data.lang != "no" {
        suggestion.title = format!(
            "{}で{}を作るのはどうですか？",
            data.lang.to_string(),
            menu["beginner"]["WebApp"][data.lang.to_string()]["app"]
                .as_str()
                .unwrap()
        );
        suggestion.body = format!(
            "{}というフレームワークを使うのがおすすめです。これらのURLを参考にすれば、簡単に{}を作ることができます。",
            menu["beginner"]["WebApp"][data.lang.to_string()]["fw"]
                .as_str()
                .unwrap(),
            menu["beginner"]["WebApp"][data.lang.to_string()]["app"]
                .as_str()
                .unwrap(),
        );
        suggestion.learning_url = menu["beginner"]["WebApp"][data.lang.to_string()]["learning_url"]
            .as_array()
            .unwrap()
            .clone()
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        suggestion.working_url = menu["beginner"]["WebApp"][data.lang.to_string()]["working_url"]
            .as_array()
            .unwrap()
            .clone()
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        suggestion.products = db::products::find_by_tag_name(&conn, &data.lang.to_string())
            .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?
            .iter()
            .map(|p| {
                let user = db::users::find(&conn, &p.user_id).unwrap();
                let tag_ids = db::tags::get_by_product_id(&conn, &p.id).unwrap();
                let reactions = db::reactions::get_by_product_id(&conn, &p.id).unwrap();
                let mut json_tag = json!(p).as_object_mut().unwrap().clone();
                json_tag.insert("tag_ids".to_string(), json!(tag_ids).into());
                json_tag.insert("reactions".to_string(), json!(reactions).into());
                json_tag.insert("user".to_string(), json!(user).into());
                json_tag
            })
            .collect();
    } else {
        let lang = "Python".to_string();
        suggestion.title = format!(
            "{}で{}を作るのはどうですか？",
            lang.to_string(),
            menu["beginner"]["WebApp"][lang.to_string()]["app"]
                .as_str()
                .unwrap()
        );
        suggestion.body = format!(
            "{}というフレームワークを使うのがおすすめです。これらのURLを参考にすれば、簡単に{}を作ることができます。",
            menu["beginner"]["WebApp"][lang.to_string()]["fw"]
                .as_str()
                .unwrap(),
            menu["beginner"]["WebApp"][lang.to_string()]["app"]
                .as_str()
                .unwrap(),
        );
        suggestion.learning_url = menu["beginner"]["WebApp"][lang.to_string()]["learning_url"]
            .as_array()
            .unwrap()
            .clone()
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        suggestion.working_url = menu["beginner"]["WebApp"][lang.to_string()]["working_url"]
            .as_array()
            .unwrap()
            .clone()
            .iter()
            .map(|i| i.as_str().unwrap().to_string())
            .collect();
        suggestion.products = db::products::find_by_tag_name(&conn, &lang.to_string())
            .map_err(|e| TentechError::DatabaseFailed(format!("{}", e)))?
            .iter()
            .map(|p| {
                let user = db::users::find(&conn, &p.user_id).unwrap();
                let tag_ids = db::tags::get_by_product_id(&conn, &p.id).unwrap();
                let reactions = db::reactions::get_by_product_id(&conn, &p.id).unwrap();
                let mut json_tag = json!(p).as_object_mut().unwrap().clone();
                json_tag.insert("tag_ids".to_string(), json!(tag_ids).into());
                json_tag.insert("reactions".to_string(), json!(reactions).into());
                json_tag.insert("user".to_string(), json!(user).into());
                json_tag
            })
            .collect();
    }
    Ok(json!({ "suggestion": suggestion }))
}
