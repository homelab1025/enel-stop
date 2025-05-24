use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use redis::aio::ConnectionLike;
use redis::RedisError;
use serde::Serialize;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::{OpenApi, ToSchema};

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
    paths(ping, count_incidents),
    components(schemas(RecordCount, Ping)),
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

#[utoipa::path(
        get,
        path = "/incidents/count",
        responses(
            (status=200, description = "Count the number of records in the DB.", body=[RecordCount]),
            (status=500, description = "Error counting the number of records in the DB."),
        )
    )]
pub async fn count_incidents<T>(state: State<AppState<T>>) -> Result<Json<RecordCount>, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let mut conn_guard = state.redis_conn.lock().await;
    let conn = &mut *conn_guard;
    let counter: Result<u64, RedisError> = redis::cmd("DBSIZE").query_async(conn).await;

    match counter {
        Ok(key_count) => Ok(Json(RecordCount { count: key_count })),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
            (status=200, description = "Respond with a pong.", body=[Ping]),
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
