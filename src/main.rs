mod marv;

use self::marv::models::*;
use diesel::Connection;
use diesel::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger;
use log;
use marv::config;
use marv::config::MarvSetup;
use marv::network;
use marv::plugins;
use prometheus_exporter::{self};
use std::env;
use std::io::{self, prelude::*};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn check_messages() {
    use self::marv::schema::messages::dsl::*;

    let connection = &mut establish_connection();
    let results = messages
        .filter(published.eq(true))
        .limit(5)
        .select(Message::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

fn initialize() -> MarvSetup {
    dotenv().ok();
    env_logger::init();
    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap()).unwrap();
    establish_connection();
    check_messages();

    let setup = config::read_configuration().unwrap();
    let hostname = setup.config.hostname.clone();
    log::info!("Initializing marvbot - {}", hostname);

    return setup;
}

fn main() -> io::Result<()> {
    let setup = initialize();
    let mut plugins = plugins::default(&setup);

    network::stream(setup, |writer, protocol| {
        plugins::dispatch(&mut plugins, &protocol, |response: String| {
            writer.write_all(response.as_bytes()).unwrap()
        });
    });

    Ok(())
}
