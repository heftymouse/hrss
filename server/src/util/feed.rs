use actix_web::web::Buf;
use feed_rs::parser;

use crate::models::FeedItem;

pub async fn read_feed(url: &str) -> Result<Vec<FeedItem>, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?.bytes().await?.reader();
    let feed = parser::parse(res).unwrap();
    
    println!("{:?}", feed.entries[0]);

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn feed_test() {
        let url = "https://heftymouse.me/blog/rss.xml";

        println!("{:?}", crate::util::feed::read_feed(url).await);
    }
}