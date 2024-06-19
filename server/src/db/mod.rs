use sqlx::{sqlite::SqliteRow, Connection, Row, Sqlite, SqliteConnection};

use crate::models::Feed;

pub async fn get_feeds() -> Result<Vec<Feed>, sqlx::Error>  {
    let mut conn = SqliteConnection::connect("sqlite::memory").await?;

    sqlx::query_as::<Sqlite, Feed>("SELECT * FROM feeds")
    .fetch_all(&mut conn)
    .await
}

pub async fn add_feed(feed: Feed) -> Result<(), sqlx::Error> {
    let mut conn = SqliteConnection::connect("sqlite::memory").await?;

    sqlx::query("INSERT INTO feeds (name, description, link, last_updated) VALUES (?, ?, ?, ?)")
    .bind(feed.name)
    .bind(feed.description)
    .bind(feed.link)
    .bind(feed.last_updated)
    .execute(&mut conn)
    .await?;

    Ok(())
}


