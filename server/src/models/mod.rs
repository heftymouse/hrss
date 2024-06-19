use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Feed {
    pub name: String,
    pub description: String,
    pub link: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct FeedItem {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "link")]
    pub link: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<u64>,
    #[serde(rename = "content")]
    pub content: Option<String>,
}