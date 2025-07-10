use crate::metrics::AppMetrics::{RequestProcessingTime, RssIncidentsCount};
use crate::AppState;
use axum::extract::{MatchedPath, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use log::debug;
use prometheus_client::encoding::text::encode_registry;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use redis::aio::ConnectionLike;
use std::collections::HashMap;
use tokio::time::Instant;

#[derive(Eq, Hash, PartialEq)]
pub enum AppMetrics {
    RequestProcessingTime,
    RssIncidentsCount,
}

type HistogramWithLabels = Family<Vec<(String, String)>, Histogram, fn() -> Histogram>;
type GaugeWithLabels = Family<Vec<(String, String)>, Gauge, fn() -> Gauge>;
enum MetricHandle {
    Histogram(HistogramWithLabels),
    Gauge(GaugeWithLabels),
}

pub struct Metrics {
    metrics: HashMap<AppMetrics, MetricHandle>,
    pub registry: Registry,
}

const HISTOGRAM_BUCKETS: [f64; 11] = [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let mut metrics_registry = Registry::with_prefix_and_labels("enel", [].into_iter());
        let mut metrics = HashMap::<AppMetrics, MetricHandle>::new();

        let request_processing_time =
            Family::<Vec<(String, String)>, Histogram>::new_with_constructor(|| Histogram::new(HISTOGRAM_BUCKETS));
        metrics.insert(
            RequestProcessingTime,
            MetricHandle::Histogram(request_processing_time.clone()),
        );
        metrics_registry.register(
            "Request processing time",
            "Total time it took to process the HTTP request.",
            request_processing_time,
        );

        let rss_incidents_count = Family::<Vec<(String, String)>, Gauge>::new_with_constructor(|| Gauge::default());
        metrics.insert(RssIncidentsCount, MetricHandle::Gauge(rss_incidents_count.clone()));
        metrics_registry.register(
            "number of incidents in the rss file",
            "This is the number of incidents in the RSS file. It does not consider what we already have persisted.",
            rss_incidents_count.clone(),
        );

        Metrics {
            metrics,
            registry: metrics_registry,
        }
    }

    pub fn get_gauge(&self, metric_type: AppMetrics) -> Option<&GaugeWithLabels> {
        match self.metrics.get(&metric_type) {
            Some(MetricHandle::Gauge(m)) => Some(m),
            _ => None,
        }
    }

    pub fn get_histogram(&self, metric_type: AppMetrics) -> Option<&HistogramWithLabels> {
        match self.metrics.get(&metric_type) {
            Some(MetricHandle::Histogram(m)) => Some(m),
            _ => None,
        }
    }
}

pub async fn monitor_endpoint<T>(state: State<AppState<T>>, request: Request, next: Next) -> Response
where
    T: ConnectionLike + Send + Sync,
{
    let start_time = Instant::now();
    debug!("Processing request: {:?}", request);
    let mut labels: Vec<(String, String)> = vec![("method".to_string(), request.method().to_string())];
    let matched_part = request.extensions().get::<MatchedPath>();

    match matched_part {
        Some(path) => {
            labels.push(("path".into(), path.as_str().to_string()));
        }
        None => {
            labels.push(("method".into(), request.uri().path().to_string()));
        }
    }

    let response = next.run(request).await;

    debug!("Got response: {:?}", response);
    state
        .metrics
        .read()
        .await
        .get_histogram(AppMetrics::RequestProcessingTime)
        .inspect(|histogram| {
            histogram
                .get_or_create(&labels)
                .observe(start_time.elapsed().as_secs_f64());
        });
    response
}

pub async fn serve_metrics<T>(state: State<AppState<T>>) -> Result<String, (StatusCode, String)>
where
    T: ConnectionLike + Send + Sync,
{
    let mut buffer = String::new();
    let metrics_registry = &state.metrics.read().await.registry;
    let r = encode_registry(&mut buffer, metrics_registry);

    if r.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, buffer));
    }

    Ok(buffer)
}
