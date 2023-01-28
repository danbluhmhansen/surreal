use dotenvy::dotenv;
use reqwest::{Body, Client};
use serde::Deserialize;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let res = SurrealClient::new(
        env::var("DB_URL")?,
        env::var("DB_USER")?,
        Some(env::var("DB_PASS")?),
        env::var("DB_NAME")?,
    )
    .send::<SurrealInfo, &str>("INFO FOR DB;")
    .await?;

    println!("{:?}", res[0]);

    Ok(())
}

#[derive(Debug)]
struct SurrealClient {
    url: String,
    user: String,
    pass: Option<String>,
    name: String,
    client: Client,
}

impl SurrealClient {
    fn new(url: String, user: String, pass: Option<String>, name: String) -> Self {
        SurrealClient {
            url,
            user,
            pass,
            name,
            client: Client::new(),
        }
    }

    async fn send<T, B: Into<Body>>(
        &self,
        body: B,
    ) -> Result<Vec<SurrealResponse<T>>, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        Ok(self
            .client
            .post(&self.url)
            .body(body)
            .basic_auth(&self.user, self.pass.as_ref())
            .header("Accept", "application/json")
            .header("NS", &self.name)
            .header("DB", &self.name)
            .send()
            .await?
            .json::<Vec<SurrealResponse<T>>>()
            .await?)
    }
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
