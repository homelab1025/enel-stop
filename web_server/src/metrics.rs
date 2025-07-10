use crate::metrics::AppMetrics::{RssIncidentsCount, RssProcessingTime};
use crate::AppState;
use axum::extract::State;
use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::metrics::histogram::Histogram;
use prometheus_client::registry::Registry;
use redis::aio::ConnectionLike;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum AppMetrics {
    RssProcessingTime,
    RssIncidentsCount,
}

enum MetricHandle {
    Histogram(Histogram),
    Gauge(Gauge),
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

        let rss_processing_time = Histogram::new(HISTOGRAM_BUCKETS);
        metrics.insert(RssProcessingTime, MetricHandle::Histogram(rss_processing_time.clone()));
        metrics_registry.register(
            "rss processing time",
            "Total time it took to process the RSS file.",
            rss_processing_time,
        );

        let rss_incidents_count: Gauge = Gauge::default();
        metrics.insert(RssIncidentsCount, MetricHandle::Gauge(rss_incidents_count.clone()));
        metrics_registry.register(
            "number of incidents in the rss file",
            "This is the number of incidents in the RSS file. It does not consider what we already have persisted.",
            rss_incidents_count,
        );

        Metrics {
            metrics,
            registry: metrics_registry,
        }
    }

    pub fn get_gauge(&self, metric_type: AppMetrics) -> Option<&Gauge> {
        match self.metrics.get(&metric_type) {
            Some(MetricHandle::Gauge(m)) => Some(m),
            _ => None,
        }
    }

    pub fn get_histogram(&self, metric_type: AppMetrics) -> Option<&Histogram> {
        match self.metrics.get(&metric_type) {
            Some(MetricHandle::Histogram(m)) => Some(m),
            _ => None,
        }
    }
}

pub async fn serve_metrics<T>(state: State<AppState<T>>)
where
    T: ConnectionLike + Send + Sync,
{
}
