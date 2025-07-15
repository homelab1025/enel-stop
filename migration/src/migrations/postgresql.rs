use crate::migrations::MigrationProcess;
use common::configuration::ServiceConfiguration;
use log::{error, info};
use redis::{ConnectionLike, RedisError, cmd};
use sqlx::postgres::PgPoolOptions;

/// CURRENT: data is stored in redis
/// NEXT: data is stored in postgresql
#[derive(Default, Debug)]
pub struct PosgresqlMigration {
    failed_migrations: Vec<String>,
    skipped: Vec<String>,
}

impl MigrationProcess for PosgresqlMigration {
    fn migrate(&mut self, _conn: &mut dyn ConnectionLike, service_config: &ServiceConfiguration) {
        let db_user = &service_config.db_user.clone().unwrap();
        let db_password = &service_config.db_password.clone().unwrap();
        let db_host = &service_config.db_host.clone().unwrap();
        let connection_string = format!("postgres://{}:{}@{}", db_user, db_password, db_host);

        let _pool = PgPoolOptions::new().connect(connection_string.as_str());
    }
    fn migrate_key(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        info!("KEY {}", key);

        let key_components: Vec<&str> = key.split(':').collect();
        if key_components.len() == 2 && key_components[0] == "incidents" {
            let external_id = key_components[1];
            let result: Result<String, RedisError> = cmd("GET").arg(key).query(redis_conn);
            match result {
                Ok(valid) => {
                    info!("Moved key {} to postgresql", key);
                }
                Err(error) => {
                    self.failed_migrations.push(String::from(key));
                    error!("There was an error migrating: {}", error)
                }
            }
        } else {
            info!("Skipping key {}", key);
            self.skipped.push(String::from(key));
        }
    }

    fn get_start_version(&self) -> u64 {
        3
    }

    fn get_description(&self) -> String {
        String::from("Migrate to Postgresql")
    }

    fn print_results(&mut self) {
        info!("FINISHED RENAME FOR {}", self.get_start_version());
        info!("Skipped RENAME FOR {:?}", self.skipped);
        info!("Failed RENAME FOR {:?}", self.failed_migrations);
    }
}
