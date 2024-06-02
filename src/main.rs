use clickhouse::{Client, Row};
use serde::Deserialize;

#[derive(Row, Deserialize, Debug)]
struct User {
    name: String,
}

#[tokio::main]
async fn main() {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("test")
        .with_user("default")
        .with_password("default");
    // The settings are there to demonstrate that increasing timeout settings doesn't change anything
    let query = client.query("SELECT * FROM users SETTINGS http_receive_timeout=3600, receive_timeout=3600, http_send_timeout=3600, send_timeout=3600");
    let mut cursor = query.fetch::<User>().unwrap();
    while let Some(row) = cursor.next().await.unwrap() {
        println!("{:#?}", row);
        // Stop reading from the socket for 60 seconds, which will trigger `ExceptionWhileProcessing` after 30 seconds
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
