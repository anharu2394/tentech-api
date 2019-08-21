use crate::error::TentechError;
use crate::models::tag::ProductTag;
use crate::models::tag::Tag;
use crate::schema::tags;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Iterator;
use std::time::SystemTime;
use std::vec::Vec;
use uuid::Uuid;

#[derive(Insertable, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub kind: String,
    pub uuid: Uuid,
}

pub fn init(conn: &PgConnection) -> Result<usize, Error> {
    let langs = BufReader::new(File::open("data/languages.txt").unwrap())
        .lines()
        .map(|l| NewTag {
            name: l.unwrap(),
            kind: "lang".to_string(),
            uuid: Uuid::new_v4(),
        });
    let fws = BufReader::new(File::open("data/frameworks.txt").unwrap())
        .lines()
        .map(|l| NewTag {
            name: l.unwrap(),
            kind: "fw".to_string(),
            uuid: Uuid::new_v4(),
        });
    let tools = BufReader::new(File::open("data/tools.txt").unwrap())
        .lines()
        .map(|l| NewTag {
            name: l.unwrap(),
            kind: "tool".to_string(),
            uuid: Uuid::new_v4(),
        });
    let new_tags: Vec<_> = langs.chain(fws).chain(tools).collect();
    let new_names: Vec<String> = new_tags.iter().map(|t| t.name.to_string()).collect();
    let exist_names = tags::table.select(tags::name).load::<String>(conn)?;
    if new_names == exist_names {
        return Ok(0);
    }
    diesel::insert_into(tags::table)
        .values(new_tags)
        .execute(conn)
}
