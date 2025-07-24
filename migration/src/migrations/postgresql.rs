use crate::migrations::MigrationProcess;
use common::Record;
use log::{error, info};
use postgres::Client;
use redis::{cmd, ConnectionLike, RedisError};

/// CURRENT: data is stored in redis
/// NEXT: data is stored in postgresql
pub struct PostgresqlMigration {
    failed_migrations: Vec<String>,
    skipped: Vec<String>,
    // TODO: this should be a reference and the ref should live as long as this struct lives
    pg_client: Client,
}

impl PostgresqlMigration {
    pub fn new(pg_pool: Client) -> Self {
        PostgresqlMigration {
            pg_client: pg_pool,
            skipped: vec![],
            failed_migrations: vec![],
        }
    }
}

const INSERT_QUERY: &str = "INSERT INTO incidents(external_id, day, county, location, description) \
 VALUES ($1, $2, $3, $4, $5) \
 ON CONFLICT (external_id) DO \
 UPDATE SET day = $2, county = $3, location = $4, description = $5";

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

                    let insert_res = self.pg_client.execute(
                        INSERT_QUERY,
                        &[
                            &external_id,
                            &incident.date,
                            &incident.county.to_string(),
                            &incident.location.to_string(),
                            &incident.description.to_string(),
                        ],
                    );

                    info!("INSERT result: {:?}", insert_res);
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

#[cfg(test)]
mod tests {
    use crate::migrations::postgresql::PostgresqlMigration;
    use crate::migrations::MigrationProcess;
    use chrono::NaiveDate;
    use common::Record;
    use log::LevelFilter;
    use postgres::{Client, NoTls};
    use redis::Value;
    use redis_test::{MockCmd, MockRedisConnection};
    use simple_logger::SimpleLogger;
    use std::thread;
    use std::time::Duration;
    use testcontainers::runners::SyncRunner;
    use testcontainers_modules::postgres::Postgres;

    const INIT_SCRIPT: &str = "
    CREATE TABLE incidents
    (
    id          VARCHAR(255) PRIMARY KEY,
    day         DATE,
    county      VARCHAR(255) NOT NULL,
    location    VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL
    );
CREATE INDEX incident_day ON incidents (day);
CREATE INDEX incident_county ON incidents (county);
CREATE SEQUENCE incidents_id
    INCREMENT BY 1
    MINVALUE 1
    MAXVALUE 9223372036854775807
    START 1
	CACHE 1
	NO CYCLE;
ALTER TABLE incidents DROP CONSTRAINT IF EXISTS incidents_pkey;
ALTER TABLE incidents RENAME COLUMN id TO external_id;
ALTER TABLE incidents ADD COLUMN id BIGINT DEFAULT nextval('incidents_id');
ALTER TABLE incidents ALTER COLUMN id SET NOT NULL;
ALTER TABLE incidents ADD PRIMARY KEY (id);
CREATE UNIQUE INDEX unique_external_id ON incidents(external_id);";

    #[test]
    fn test_migration() {
        SimpleLogger::new().env().with_level(LevelFilter::Info).init().unwrap();

        let record = Record {
            id: "123".to_string(),
            title: "test_title".to_string(),
            description: "test_description".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            county: "test_judet".to_string(),
            location: "test_localitate".to_string(),
        };

        // mock connection so to return a set of records
        let record_ser = serde_json::to_string(&record).unwrap();
        let redis_key = format!("incidents:{}", record.id);
        let mut conn = MockRedisConnection::new(vec![MockCmd::new(
            redis::cmd("GET").arg(redis_key.clone()),
            Ok(Value::SimpleString(record_ser)),
        )]);

        let pg_server = Postgres::default()
            .with_init_sql(INIT_SCRIPT.to_string().into_bytes())
            .with_user("postgres")
            .with_password("postgres")
            .with_db_name("enel")
            .start()
            .unwrap();

        thread::sleep(Duration::from_secs(3));

        let pg_host = pg_server.get_host().unwrap();
        let pg_port = pg_server.get_host_port_ipv4(5432).unwrap();

        let connection_info = format!(
            "host={} user=postgres password=postgres dbname=enel port={}",
            pg_host, pg_port
        );
        let pg_client = Client::connect(connection_info.as_str(), NoTls).unwrap();

        let mut migration = PostgresqlMigration::new(pg_client);
        migration.migrate_key(redis_key.as_str(), &mut conn);

        let mut pg_client2 = Client::connect(connection_info.as_str(), NoTls).unwrap();
        let row = pg_client2
            .query_one("SELECT * FROM incidents WHERE external_id = $1", &[&record.id])
            .unwrap();

        assert_eq!(0, migration.failed_migrations.len());
        assert_eq!(0, migration.skipped.len());

        assert_eq!(record.id, row.get::<_, String>("external_id"));
        assert_eq!(record.description, row.get::<_, String>("description"));
        assert_eq!(record.date, row.get::<_, NaiveDate>("day"));
        assert_eq!(record.county, row.get::<_, String>("county"));
        assert_eq!(record.location, row.get::<_, String>("location"));
    }
}
