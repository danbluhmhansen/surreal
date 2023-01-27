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
        .json::<Vec<SurrealResponse<SurrealInfo>>>()
        .await?;

    println!("{:?}", res[0]);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct SurrealResponse<T> {
    time: String,
    status: String,
    result: T,
}

#[derive(Debug, Deserialize)]
struct SurrealInfo {
    dl: serde_json::Value,
    dt: serde_json::Value,
    sc: serde_json::Value,
    tb: serde_json::Value,
}
