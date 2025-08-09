use crate::AppState;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use chrono::NaiveDate;
use log::error;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, QueryBuilder, Row};
use std::ops::Deref;
use utoipa::{IntoParams, OpenApi, ToSchema};

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
    pub total_count: i64,
    #[schema(value_type = String, format = Date)]
    pub start_date: NaiveDate,
    #[schema(value_type = String, format = Date)]
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Ping {
    pub(crate) ping: String,
}

#[derive(Debug, Serialize, Clone, ToSchema, FromRow)]
pub struct Incident {
    pub external_id: String,
    pub county: String,
    pub location: String,
    #[schema(value_type = String, format = Date)]
    pub day: NaiveDate,
    pub description: String,
    pub id: i64,
}

#[utoipa::path(
    get,
    path = "/incidents/count",
    responses(
            (status=200, description = "Count the number of records in the DB.", body=RecordCount),
            (status=500, description = "Error counting the number of records in the DB."),
    )
)]
pub async fn count_incidents(state: State<AppState>) -> Result<Json<RecordCount>, (StatusCode, String)> {
    let row =
        sqlx::query("SELECT COUNT(*) as total_count, MIN(day) as start_date, MAX(day) as end_date FROM incidents")
            .fetch_one(state.pg_pool.deref())
            .await;

    match row {
        Ok(row) => Ok(Json(RecordCount {
            total_count: row.get("total_count"),
            start_date: row.get("start_date"),
            end_date: row.get("end_date"),
        })),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

#[derive(Deserialize, IntoParams, Default)]
pub struct IncidentsFiltering {
    pub county: Option<String>,
    pub offset: Option<u64>,
    pub count: Option<u64>,
    pub day: Option<String>,
    // datetime: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct GetIncidentsResponse {
    pub incidents: Vec<Incident>,
    pub total_count: u64,
}

#[utoipa::path(
    get,
    path = "/incidents/all",
    params(
        IncidentsFiltering
    ),
    responses(
        (status=200, description = "All incidents.", body=GetIncidentsResponse),
        (status=500, description = "Error getting all incidents.")
    )
)]
pub async fn get_all_incidents(
    state: State<AppState>,
    filtering: Query<IncidentsFiltering>,
) -> Result<Json<GetIncidentsResponse>, (StatusCode, String)> {
    let offset = filtering.offset;
    let count = filtering.count.unwrap_or(50);

    let mut query_builder = QueryBuilder::new("SELECT * FROM incidents");

    if filtering.county.is_some() || filtering.day.is_some() {
        query_builder.push(" WHERE ");

        let mut separated = query_builder.separated(" AND ");

        if let Some(county) = &filtering.county {
            separated.push("county = ").push_bind_unseparated(county);
        }

        if let Some(day) = &filtering.day {
            separated
                .push("day = ")
                .push_bind_unseparated(NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap());
        }
    }

    query_builder.push(" ORDER BY day DESC");

    query_builder.push(" LIMIT ").push(count);

    if let Some(offset) = offset {
        query_builder.push(" OFFSET ").push_bind(offset as i64);
    }

    let incidents_query_result: Result<Vec<Incident>, Error> =
        query_builder.build_query_as().fetch_all(state.pg_pool.deref()).await;

    match incidents_query_result {
        Ok(incidents) => {
            let incidents_count = incidents.len() as u64;
            Ok(Json(GetIncidentsResponse {
                incidents,
                total_count: incidents_count,
            }))
        }
        Err(err) => {
            error!("{}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Internal Server Error")))
        }
    }
}

#[utoipa::path(
    get,
    path = "/ping",
    responses(
            (status=200, description = "Respond with a pong.", body=Ping),
            (status=500, description = "Server is not ready to serve."),
    )
)]
pub async fn ping(state: State<AppState>) -> Result<Json<Ping>, (StatusCode, String)> {
    let a = state.ping_msg.deref();
    let response = format!("Hello {}!", a);
    Ok(Json(Ping { ping: response }))
}
