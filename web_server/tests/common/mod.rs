use chrono::NaiveDate;
use common::Record;
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;
use std::ops::Deref;
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres;
use tokio::time::sleep;
use web_server::scraper::persistence::new_store_record;
use web_server::AppState;

pub const FILTERING_COUNTY: &str = "test_judet";
pub const FILTERING_DAY: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 1).unwrap();

pub struct TestInfrastructure {
    pub _postgres_container: ContainerAsync<postgres::Postgres>,
    pub postgres_port: u16,
    pub postgres_host: String,
}

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

        sleep(Duration::from_secs(5)).await;

        Self {
            _postgres_container: pg,
            postgres_port: pg_port,
            postgres_host: pg_host.to_string(),
        }
    }
}

static LOG_SETUP_ONCE: OnceLock<bool> = OnceLock::new();
static GENERATE_DB_DDL_ONCE: OnceLock<String> = OnceLock::new();

pub async fn create_app_state(infra: &TestInfrastructure) -> AppState {
    setup_logging();

    GENERATE_DB_DDL_ONCE.get_or_init(|| match generate_ddl() {
        Ok(value) => value,
        Err(err) => return err,
    });

    let pg_pool = setup_postgres(infra, GENERATE_DB_DDL_ONCE.get().unwrap()).await;

    AppState {
        ping_msg: "The state of ping.".to_string(),
        categories: vec![],
        metrics: Default::default(),
        pg_pool: pg_pool.clone(),
    }
}

pub fn setup_logging() {
    LOG_SETUP_ONCE.get_or_init(|| {
        let re = SimpleLogger::new().env().with_level(LevelFilter::Info).init();

        match re {
            Ok(_) => info!("Logging initialized."),
            Err(error) => println!("Failed to initialize logging: {}", error),
        }

        true
    });
}

pub fn generate_ddl() -> Result<String, String> {
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
        return Err(String::from(""));
    }

    Ok(String::from_utf8(output.stdout).unwrap())
}

async fn setup_postgres(infra: &TestInfrastructure, ddl: &str) -> Arc<Pool<Postgres>> {
    let pg_conn_string = format!(
        "postgres://postgres:postgres@{}:{}/enel",
        &infra.postgres_host, &infra.postgres_port
    );
    info!("Connecting to postgres: {}", &pg_conn_string);
    let pg_pool = Arc::new(PgPoolOptions::new().connect(&pg_conn_string).await.unwrap());

    let _res = sqlx::raw_sql(ddl).execute(pg_pool.clone().deref()).await.unwrap();
    populate_postgres(pg_pool.clone(), get_records().as_ref()).await;

    pg_pool.clone()
}

async fn populate_postgres(pool: Arc<Pool<Postgres>>, records: &[Record]) {
    for record in records {
        let _res = new_store_record(record, pool.clone()).await;
    }
}

fn get_records() -> Vec<Record> {
    let records = vec![
        Record {
            id: "test_id".to_string(),
            title: "test_title".to_string(),
            description: "test_description".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2023, 10, 2).unwrap(),
            county: FILTERING_COUNTY.to_string(),
            location: "test_localitate".to_string(),
        },
        Record {
            id: "test_id2".to_string(),
            title: "test_title2".to_string(),
            description: "test_description2".to_string(),
            date: FILTERING_DAY,
            county: FILTERING_COUNTY.to_string(),
            location: "test_localitate".to_string(),
        },
        Record {
            id: "test_id3".to_string(),
            title: "test_title3".to_string(),
            description: "test_description3".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2023, 12, 3).unwrap(),
            county: "test_judet2".to_string(),
            location: "test_localitate2".to_string(),
        },
        Record {
            id: "test_id4".to_string(),
            title: "test_title3".to_string(),
            description: "test_description3".to_string(),
            date: FILTERING_DAY,
            county: "test_judet2".to_string(),
            location: "test_localitate2".to_string(),
        },
        Record {
            id: "test_id5".to_string(),
            title: "test_title3".to_string(),
            description: "test_description3".to_string(),
            date: FILTERING_DAY,
            county: "test_judet2".to_string(),
            location: "test_localitate2".to_string(),
        },
    ];
    records
}
