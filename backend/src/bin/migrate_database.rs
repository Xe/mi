#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate tracing;

use color_eyre::eyre::{eyre, Result};
use diesel::prelude::*;
use mi::APPLICATION_NAME;
use std::env;

diesel_migrations::embed_migrations!("./migrations");

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL").unwrap_or("./mi.db".to_string());
    SqliteConnection::establish(&database_url)
        .map_err(|why| eyre!("can't connect to {}: {}", database_url, why))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("{} migrator starting up", APPLICATION_NAME);

    info!("running migrations");
    let connection = establish_connection()?;
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).map_err(|why| {
        error!("migration error: {}", why);
        why
    })?;
    info!("migrations succeeded");

    Ok(())
}
