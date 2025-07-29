use common::Record;
use log::{error, info, LevelFilter};
use redis::aio::MultiplexedConnection;
use redis::Client;
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres;
use testcontainers_modules::redis::Redis;
use tokio::sync::Mutex;
use tokio::time::sleep;
use web_server::scraper::redis_store::store_record;
use web_server::AppState;

pub const FILTERING_COUNTY: &str = "test_judet";

pub struct TestInfrastructure {
    pub _redis_container: ContainerAsync<Redis>,
    pub _postgres_container: ContainerAsync<postgres::Postgres>,
    pub postgres_port: u16,
    pub redis_port: u16,
    pub redis_host: String,
    pub postgres_host: String,
}

pub const REDIS_TAG: &str = "7.4.2";
impl TestInfrastructure {
    pub async fn new() -> TestInfrastructure {
        let pg = postgres::Postgres::default()
            .with_user("postgres")
            .with_password("postgres")
            .with_db_name("enel")
            .start()
            .await
            .unwrap();
        let pg_port = pg.get_host_port_ipv4(5432).await.unwrap();
        let pg_host = pg.get_host().await.unwrap();

        let redis = testcontainers_modules::redis::Redis::default()
            .with_tag(REDIS_TAG)
            .start()
            .await
            .unwrap();
        let redis_port = redis.get_host_port_ipv4(6379).await.unwrap();
        let redis_host = redis.get_host().await.unwrap();

        sleep(Duration::from_secs(5)).await;

        Self {
            _postgres_container: pg,
            _redis_container: redis,
            postgres_port: pg_port,
            redis_port: redis_port,
            postgres_host: pg_host.to_string(),
            redis_host: redis_host.to_string(),
        }
    }
}

static LOG_SETUP_ONCE: OnceLock<bool> = OnceLock::new();
static GENERATE_DB_DDL_ONCE: OnceLock<String> = OnceLock::new();

pub async fn create_app_state(infra: &TestInfrastructure) -> AppState<MultiplexedConnection> {
    LOG_SETUP_ONCE.get_or_init(|| {
        let re = SimpleLogger::new().env().with_level(LevelFilter::Info).init();

        match re {
            Ok(_) => {
                info!("Logging initialized.")
            }
            Err(error) => {
                println!("Failed to initialize logging: {}", error)
            }
        }

        true
    });

    GENERATE_DB_DDL_ONCE.get_or_init(|| {
        let mut db_path = env::current_dir().unwrap();
        // TODO: make this more generic as it's custom made for integration tests in web_server workspace
        db_path.pop();
        db_path.push("db");

        println!("DB path: {:?}", db_path);

        let _ = Command::new("rm")
            .current_dir(&db_path)
            .stdout(Stdio::piped())
            .args(["databasechangelog.csv"])
            .output()
            .unwrap();

        let output = Command::new("liquibase")
            .current_dir(&db_path)
            .stdout(Stdio::piped())
            .args(["--url=offline:postgresq", "updateSQL"])
            .output()
            .unwrap();

        if !output.status.success() {
            error!(
                "Failed to run liquibase update sql: {}",
                String::from_utf8(output.stderr).unwrap()
            );
            return String::from("");
        }

        String::from_utf8(output.stdout).unwrap()
    });

    let async_redis_conn = setup_redis(&infra).await;
    let pg_pool = setup_postgres(&infra, GENERATE_DB_DDL_ONCE.get().unwrap()).await;

    AppState {
        ping_msg: "The state of ping.".to_string(),
        redis_conn: Arc::new(Mutex::new(async_redis_conn)),
        categories: vec![],
        metrics: Default::default(),
        pg_pool: Arc::new(pg_pool),
    }
}

async fn setup_postgres(infra: &&TestInfrastructure, ddl: &str) -> Pool<Postgres> {
    let pg_conn_string = format!(
        "postgres://postgres:postgres@{}:{}/enel",
        &infra.postgres_host, &infra.postgres_port
    );
    info!("Connecting to postgres: {}", &pg_conn_string);
    let pg_pool = PgPoolOptions::new().connect(&pg_conn_string).await.unwrap();
    let _res = sqlx::raw_sql(ddl).execute(&pg_pool).await.unwrap();

    pg_pool
}

async fn setup_redis(infra: &&TestInfrastructure) -> MultiplexedConnection {
    let conn_string = format!("redis://{}:{}/", &infra.redis_host, &infra.redis_port);
    info!("Connecting to REDIS: {}", &conn_string);
    let redis_client = Client::open(conn_string).expect("Connecting to the redis container");

    let async_redis_conn = redis_client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Async connection to Redis");
    populate_redis(&mut async_redis_conn.clone()).await;
    async_redis_conn
}

async fn populate_redis(conn: &mut MultiplexedConnection) {
    let incident1_county1 = Record {
        id: "test_id".to_string(),
        title: "test_title".to_string(),
        description: "test_description".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        county: FILTERING_COUNTY.to_string(),
        location: "test_localitate".to_string(),
    };
    let incident2_county1 = Record {
        id: "test_id2".to_string(),
        title: "test_title2".to_string(),
        description: "test_description2".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(),
        county: FILTERING_COUNTY.to_string(),
        location: "test_localitate".to_string(),
    };
    let incident3_county2 = Record {
        id: "test_id3".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };
    let incident4_county2 = Record {
        id: "test_id4".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };
    let incident5_county2 = Record {
        id: "test_id5".to_string(),
        title: "test_title3".to_string(),
        description: "test_description3".to_string(),
        date: chrono::NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
        county: "test_judet2".to_string(),
        location: "test_localitate2".to_string(),
    };

    let _res = store_record(&incident1_county1, conn).await;
    let _res = store_record(&incident2_county1, conn).await;
    let _res = store_record(&incident3_county2, conn).await;
    let _res = store_record(&incident4_county2, conn).await;
    let _res = store_record(&incident5_county2, conn).await;
}
