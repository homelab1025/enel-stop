use common::configuration;
use log::{error, info, LevelFilter};
use migration::call_migration;
use migration::migrations::rename_prefix::RenamePrefixMigration;
use migration::migrations::sorted_set::SortedSetMigration;
use migration::migrations::MigrationProcess;
use simple_logger::SimpleLogger;
use std::env;

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();
    let config = env::args().nth(1).and_then(|file_path| {
        info!("Using config file {}", &file_path);
        match configuration::get_configuration(&file_path) {
            Ok(configuration) => Some(configuration),
            Err(err) => {
                error!("Could not build configuration: {}", err);
                None
            }
        }
    });

    if let Some(config) = config {
        info!("Using redis server: {:?}", &config.redis_server.clone());
        let redis_string = config.redis_server.expect("Redis server must be configured.");
        let client = redis::Client::open(redis_string);
        match client {
            Ok(client) => match client.get_connection() {
                Ok(mut redis_conn) => {
                    let mut sorted_set_migration = SortedSetMigration::default();
                    let mut rename_migration = RenamePrefixMigration::default();
                    let mut migrations: Vec<&mut dyn MigrationProcess> =
                        vec![&mut sorted_set_migration, &mut rename_migration];
                    migrations.sort_by_key(|f| f.get_start_version());
                    info!("Migrations: {:?}",migrations.iter().map(|m| m.get_description()).collect::<Vec<_>>());
                    call_migration(&mut migrations, &mut redis_conn);
                }
                Err(err) => {
                    error!("Could not connect to redis server: {}", err);
                }
            },
            Err(err) => {
                error!(
                    "Redis client could not be created. Check connection string or remove it if you don't want to store results. Error: {}",
                    err
                );
            }
        }
    }
}
