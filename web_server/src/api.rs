use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use common::Record;
use common::persistence::SORTED_INCIDENTS_KEY;
use log::{debug, error};
use redis::aio::ConnectionLike;
use redis::{RedisError, RedisResult, cmd};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::r#gen::serde_json;
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(Clone)]
pub struct AppState<T>
where
    T: ConnectionLike + Send + Sync,
{
    pub ping_msg: String,
    pub redis_conn: Arc<Mutex<T>>,
}

#[derive(OpenApi)]
#[openapi(
    paths(ping, count_incidents, get_all_incidents),
    components(schemas(RecordCount, Ping, Incident)),
    servers(
        (url="https://enel.lab.wicked/api", description="homelab"),
        (url="http://localhost:8080/api", description="localhost")
    ),
    info(title = "Test API", license(name = "hey", identifier = "CC-BY-ND-4.0"))
)]
pub struct ApiDoc;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct RecordCount {
    count: u64,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Ping {
    ping: String,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Incident {
    id: String,
    county: String,
    location: String,
    datetime: String,
}

#[utoipa::path(
    get,
    path = "/incidents/count",
    responses(
            (status=200, description = "Count the number of records in the DB.", body=RecordCount),
            (status=500, description = "Error counting the number of records in the DB."),
    )
)]
pub async fn count_incidents<T>(state: State<AppState<T>>) -> Result<Json<RecordCount>, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let mut conn_guard = state.redis_conn.lock().await;
    let conn = &mut *conn_guard;
    let counter: Result<u64, RedisError> = redis::cmd("ZCARD").arg(SORTED_INCIDENTS_KEY).query_async(conn).await;

    match counter {
        Ok(key_count) => Ok(Json(RecordCount { count: key_count })),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize, IntoParams)]
pub struct IncidentsFiltering {
    county: Option<String>,
    // datetime: Option<String>,
}

#[utoipa::path(
    get,
    path = "/incidents/all",
    params(
        IncidentsFiltering
    ),
    responses(
        (status=200, description = "All incidents.", body=Vec<Incident>),
        (status=500, description = "Error getting all incidents.")
    )
)]
pub async fn get_all_incidents<T>(
    state: State<AppState<T>>,
    filtering: Query<IncidentsFiltering>,
) -> Result<Json<Vec<Incident>>, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let mut conn_guard = state.redis_conn.lock().await;
    let conn = &mut *conn_guard;
    let rev_ordered_incidents = get_rev_ordered_incidents(conn).await;

    match rev_ordered_incidents {
        Ok(incidents_keys) => {
            debug!("Found {} incidents", incidents_keys.len());

            let mut all_incidents: Vec<Incident> = vec![];

            for incident_key in incidents_keys {
                match cmd("GET").arg(incident_key).query_async::<String>(conn).await {
                    Ok(result) => {
                        let record_des_result = serde_json::from_str::<Record>(&result);
                        match record_des_result {
                            Ok(record) => {
                                let show = filtering
                                    .county
                                    .clone()
                                    .map_or_else(|| true, |county| county == record.judet);

                                if show {
                                    all_incidents.push(Incident {
                                        id: record.id,
                                        county: record.judet,
                                        location: record.localitate,
                                        datetime: record.date.to_string(),
                                    })
                                }
                            }
                            Err(error) => {
                                error!("Could not deserialize record: {:?}", error);
                            }
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }

            Ok(Json(all_incidents))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

async fn get_rev_ordered_incidents<T>(conn: &mut T) -> RedisResult<Vec<String>>
where
    T: ConnectionLike + Send + Sync,
{
    let ordered_incidents: RedisResult<Vec<String>> = redis::cmd("ZRANGE")
        .arg(SORTED_INCIDENTS_KEY)
        .arg("0")
        .arg("-1")
        .arg("REV")
        .query_async(conn)
        .await;

    ordered_incidents
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
            (status=200, description = "Respond with a pong.", body=Ping),
            (status=500, description = "Server is not ready to serve."),
    )
)]
pub async fn ping<T>(state: State<AppState<T>>) -> Result<Json<Ping>, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let a = state.ping_msg.deref();
    let response = format!("Hello {}!", a);
    Ok(Json(Ping { ping: response }))
}
