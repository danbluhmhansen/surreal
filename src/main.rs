use dotenvy::dotenv;
use reqwest::{Body, Client, RequestBuilder};
use serde::Deserialize;
use std::{
    env::{self, VarError},
    error::Error,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let res = surreal_get::<SurrealInfo>().await?;

    println!("{:?}", res[0]);

    Ok(())
}

async fn surreal_get<T>() -> Result<Vec<SurrealResponse<T>>, Box<dyn Error>>
where
    T: for<'a> Deserialize<'a>,
{
    Ok(Client::new()
        .get(env::var("DB_URL")?)
        .surreal()?
        .send()
        .await?
        .json::<Vec<SurrealResponse<T>>>()
        .await?)
}

async fn surreal_post<T, B: Into<Body>>(body: B) -> Result<Vec<SurrealResponse<T>>, Box<dyn Error>>
where
    T: for<'a> Deserialize<'a>,
{
    Ok(Client::new()
        .post(env::var("DB_URL")?)
        .surreal()?
        .body(body)
        .send()
        .await?
        .json::<Vec<SurrealResponse<T>>>()
        .await?)
}

trait SurrealRequest {
    fn surreal(self) -> Result<Self, VarError>
    where
        Self: Sized;
}

impl SurrealRequest for RequestBuilder {
    fn surreal(self) -> Result<Self, VarError>
    where
        Self: Sized,
    {
        Ok(self
            .basic_auth(env::var("DB_USER")?, Some(env::var("DB_PASS")?))
            .header("Accept", "application/json")
            .header("NS", env::var("DB_NAME")?)
            .header("DB", env::var("DB_NAME")?))
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
