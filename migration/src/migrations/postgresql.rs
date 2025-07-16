use crate::migrations::MigrationProcess;
use chrono::Datelike;
use common::Record;
use common::configuration::ServiceConfiguration;
use log::{error, info};
use redis::{ConnectionLike, RedisError, cmd};
use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

/// CURRENT: data is stored in redis
/// NEXT: data is stored in postgresql
#[derive(Debug)]
pub struct PostgresqlMigration {
    failed_migrations: Vec<String>,
    skipped: Vec<String>,
    pool: Pool<Postgres>,
}

impl PostgresqlMigration {
    pub fn new(service_config: &ServiceConfiguration) -> Self {
        let db_user = &service_config.db_user.clone().unwrap();
        let db_password = &service_config.db_password.clone().unwrap();
        let db_host = &service_config.db_host.clone().unwrap();
        let connection_string = format!("postgres://{}:{}@{}", db_user, db_password, db_host);

        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async {
                let pool = PgPoolOptions::new().connect(connection_string.as_str()).await.unwrap();

                PostgresqlMigration {
                    pool,
                    skipped: vec![],
                    failed_migrations: vec![],
                }
            })
    }
}

const INSERT_QUERY: &str = "INSERT INTO incidents(external_id, datetime, day, county, location, description) \
 VALUES ($1, $2, $3, $4, $5, $6) \
 ON CONFLICT (external_id) DO \
 UPDATE SET datetime = $2, day = $3, county = $4, location = $5, description = $6";

impl MigrationProcess for PostgresqlMigration {
    fn migrate_key(&mut self, key: &str, redis_conn: &mut dyn ConnectionLike) {
        info!("KEY {}", key);

        let key_components: Vec<&str> = key.split(':').collect();
        if key_components.len() == 2 && key_components[0] == "incidents" {
            let external_id = key_components[1];
            let ser_incident: Result<String, RedisError> = cmd("GET").arg(key).query(redis_conn);
            match ser_incident {
                Ok(str_val) => {
                    let incident: Record = serde_json::from_str(&str_val).unwrap();

                    tokio::runtime::Builder::new_current_thread()
                        .build()
                        .unwrap()
                        .block_on(async {
                            let res = sqlx::query(INSERT_QUERY)
                                .bind(external_id)
                                .bind(incident.date.to_string())
                                // TODO: not actually what we want
                                .bind(incident.date.day().to_string())
                                .bind(incident.judet)
                                .bind(incident.localitate)
                                .bind(incident.description)
                                .execute(&self.pool)
                                .await;

                            info!("Moved key {} to postgresql", key);
                        });
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

    fn print_results(&self) {
        info!("FINISHED RENAME FOR {}", self.get_start_version());
        info!("Skipped RENAME FOR {:?}", self.skipped);
        info!("Failed RENAME FOR {:?}", self.failed_migrations);
    }
}
