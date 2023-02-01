use std::error::Error;

use reqwest::{Body, Client};
use serde::Deserialize;

#[derive(Debug)]
pub struct SurrealClient {
    url: String,
    user: String,
    pass: Option<String>,
    name: String,
    client: Client,
}

impl SurrealClient {
    pub fn new(url: String, user: String, pass: Option<String>, name: String) -> Self {
        SurrealClient {
            url,
            user,
            pass,
            name,
            client: Client::new(),
        }
    }

    pub async fn send<T, B: Into<Body>>(
        &self,
        body: B,
    ) -> Result<Vec<SurrealResponse<T>>, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        let result = self
            .client
            .post(&self.url)
            .body(body)
            .basic_auth(&self.user, self.pass.as_ref())
            .header("Accept", "application/json")
            .header("NS", &self.name)
            .header("DB", &self.name)
            .send()
            .await?
            .text()
            .await?;
        println!("{}", result);
        Ok(serde_json::from_str::<Vec<SurrealResponse<T>>>(&result)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct SurrealResponse<T> {
    time: String,
    status: String,
    result: T,
}

#[derive(Debug, Deserialize)]
pub struct SurrealInfo {
    dl: serde_json::Value,
    dt: serde_json::Value,
    sc: serde_json::Value,
    tb: serde_json::Value,
}
