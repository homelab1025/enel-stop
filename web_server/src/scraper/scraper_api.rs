use crate::metrics::AppMetrics;
use crate::scraper::persistence::new_store_record;
use crate::scraper::rss_reader::parse_rss;
use crate::web_api::Ping;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::{debug, error, info};
use redis::aio::ConnectionLike;

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
    match incidents.await {
        Ok(incidents) => {
            debug!("Incidents: {:?}", incidents);

            let mut stored_incidents = 0;
            for incident in incidents.iter() {
                new_store_record(incident, state.pg_pool.clone())
                    .await
                    .map(|_t| {
                        stored_incidents += 1;
                    })
                    .expect("Counting stored incidents.");
            }

            let labels = vec![];
            state
                .metrics
                .read()
                .await
                .get_gauge(AppMetrics::RssIncidentsCount)
                .inspect(|gauge| {
                    gauge.get_or_create(&labels).set(stored_incidents);
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
