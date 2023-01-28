mod client;
mod entity;

use crate::{
    client::{SurrealClient, SurrealInfo},
    entity::Account,
};
use dotenvy::dotenv;
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
        .send::<Vec<Account>, String>(Account::new("ACME Inc".to_string()).create())
        .await?;
    println!("{:?}", account[0]);

    Ok(())
}
