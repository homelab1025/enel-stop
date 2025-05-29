use common::configuration;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::env;
use migration::call_migration;
use migration::migrations::MigrationProcess;
use migration::migrations::sorted_set::SortedSetMigration;

fn main() {
    SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();
    let config = env::args()
        .nth(1)
        .and_then(|file_path| configuration::get_configuration(&file_path).ok())
        .expect("Failed to load configuration");

    let redis_string = config.redis_server.expect("Redis server must be configured.");
    let client = redis::Client::open(redis_string).expect(
        "Redis client could not be created. Check connection string or remove it if you don't want to store results.",
    );

    let mut redis_conn = client.get_connection().expect("Could not connect to redis.");

    let mut sorted_set_migration = SortedSetMigration::default();
    let mut migrations: Vec<&mut dyn MigrationProcess> = vec![&mut sorted_set_migration];
    migrations.sort_by_key(|f| f.get_start_version());
    call_migration(&mut migrations, &mut redis_conn);
}

