use std::env;

use cf_migration::Migrator;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  let url = format!(
    "postgres://{}:{}@{}:{}/{}",
    env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME not set"),
    env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD not set"),
    env::var("DATABASE_HOST").expect("DATABASE_HOST not set"),
    env::var("DATABASE_PORT").expect("DATABASE_PORT not set"),
    env::var("DATABASE_NAME").expect("DATABASE_NAME not set"),
  );

  // run_cli only reads DATABASE_URL
  unsafe { env::set_var("DATABASE_URL", url) };

  cli::run_cli(Migrator).await;
}
