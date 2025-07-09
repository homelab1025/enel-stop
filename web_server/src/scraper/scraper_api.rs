use crate::AppState;
use crate::metrics::INCIDENTS_COUNT_NAME;
use crate::web_api::Ping;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use log::{debug, error, info};
use redis::aio::ConnectionLike;
use crate::scraper::redis_store::store_record;
use crate::scraper::rss_reader::parse_rss;

// #[utoipa::path(
//     get,
//     path = "/ping",
//     responses(
//             (status=200, description = "Respond with a pong.", body=Ping),
//             (status=500, description = "Server is not ready to serve."),
//     )
// )]
pub async fn submit_rss<T>(State(state): State<AppState<T>>, body: String) -> Result<Json<Ping>, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let incidents = parse_rss(&body, &state.categories);

    let mut conn_guard = state.redis_conn.lock().await;
    let redis_connection = &mut *conn_guard;

    match incidents.await {
        Ok(incidents) => {
            debug!("Incidents: {:?}", incidents);

            let mut stored_incidents = 0;
            for incident in incidents.iter() {
                store_record(incident, redis_connection)
                    .await
                    .map(|_t| {
                        stored_incidents += 1;
                    })
                    .expect("Counting stored incidents.");
            }

            state.metrics.read().await
                .get_gauge(INCIDENTS_COUNT_NAME)
                .inspect(|gauge| {
                gauge.set(stored_incidents);
            });

            info!(
                "Stored {} incidents out of {} received.",
                stored_incidents,
                incidents.len()
            );
        }
        Err(err) => {
            // state.metrics.failures_counter.inc();
            error!("{}", err);
        }
    }

    Ok(Json(Ping {
        ping: "response".to_string(),
    }))
}
