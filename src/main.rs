mod client;
mod entity;

use crate::{
    client::{SurrealClient, SurrealInfo},
    entity::Character,
};
use dotenvy::dotenv;
use r#macro::SurrealCreate;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let client = SurrealClient::new(
        env::var("DB_URL")?,
        env::var("DB_USER")?,
        Some(env::var("DB_PASS")?),
        env::var("DB_NAME")?,
    );

    let info = client.send::<SurrealInfo, &str>("INFO FOR DB;").await?;
    println!("{:?}", info[0]);

    let account = client
        .send::<Vec<Character>, String>(Character::new("Yildac Nobleroot".to_string(), 16).create())
        .await?;
    println!("{:?}", account[0]);

    Ok(())
}
