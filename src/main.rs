use serde::Deserialize;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:8000/sql")
        .basic_auth("dan", Some("surreal"))
        .header("Accept", "application/json")
        .header("NS", "surreal")
        .header("DB", "surreal")
        .body("INFO FOR DB;")
        .send()
        .await?
        .json::<Vec<SurrealResponse>>()
        .await?;

    println!("{:?}", res[0]);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct SurrealResponse {
    time: String,
    status: String,
    result: serde_json::Value,
}
