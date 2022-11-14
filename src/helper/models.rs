use serde::{Deserialize, Serialize};

use crate::db::models::AnimeDB;
use crate::db::models::ListsDB;

//  anime details
#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedAnime {
    pub id: u32,
    pub relation: i16,
}

#[derive(Debug)]
pub struct AnimeDetails {
    pub id: i32,
    pub title: String,
    pub picture: Option<String>,
    pub airing_date: Option<chrono::NaiveDate>,
    pub mean: Option<i16>,
    pub airing_status: Option<i16>,
    pub genres: Option<Vec<Option<i16>>>,
    pub num_episodes: Option<i16>,
    pub rating: Option<i16>,
    pub related: Option<Vec<RelatedAnime>>,
}

impl AnimeDetails {
    pub fn from_db(db: AnimeDB) -> Self {
        AnimeDetails {
            id: db.id,
            title: db.title,
            picture: db.picture,
            airing_date: db.airing_date,
            mean: db.mean,
            airing_status: db.airing_status,
            genres: db.genres,
            num_episodes: db.num_episodes,
            rating: db.rating,
            related: into_related(db.related),
        }
    }
}

fn into_related(value: Option<serde_json::Value>) -> Option<Vec<RelatedAnime>> {
    match value {
        Some(value) => Some(<Vec<RelatedAnime> as Deserialize>::deserialize(value).unwrap()),
        None => None,
    }
}

//  user list
#[derive(Debug, Deserialize, Serialize)]
pub struct ListEntry {
    pub id: u32,
    pub status: u8,
    pub score: u16,
    pub episodes_watched: u16,
}

pub struct List {
    pub user_hash: String,
    pub list: Vec<ListEntry>,
    pub updated_at: chrono::NaiveDateTime,
}

impl List {
    pub fn from_db(db: ListsDB) -> Self {
        List {
            user_hash: db.user_hash,
            list: to_list(db.list),
            updated_at: db.updated_at,
        }
    }
}

fn to_list(value: serde_json::Value) -> Vec<ListEntry> {
    <Vec<ListEntry> as Deserialize>::deserialize(value).unwrap()
}
