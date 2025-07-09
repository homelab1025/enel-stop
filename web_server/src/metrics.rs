use prometheus_client::metrics::gauge::Gauge;
use prometheus_client::registry::Registry;
use std::collections::HashMap;

#[derive(Default)]
pub struct Metrics {
    pub gauge_map: HashMap<String, Gauge>,
}

impl Metrics {
    pub fn new(metrics_registry: &mut Registry) -> Self {
        let mut gauge_map = HashMap::<String, Gauge>::new();

        let total_time = Gauge::default();
        gauge_map.insert(TOTAL_TIME_NAME.to_string(), total_time.clone());
        metrics_registry.register(
            TOTAL_TIME_NAME,
            "Total time it took to process the RSS file.",
            total_time,
        );

        // gauge_map.insert(
        //     TOTAL_TIME_NAME.to_string(),
        //     TelemetryMetric {
        //         description: TOTAL_TIME_NAME.to_string(),
        //         name: TOTAL_TIME_NAME.to_string(),
        //         value: Box::<Gauge<i64, AtomicI64>>::new(Gauge::default()),
        //     },
        // );
        //
        // gauge_map.insert(
        //     FAILED_SCRAPINGS_NAME.to_string(),
        //     TelemetryMetric {
        //         description: FAILED_SCRAPINGS_NAME.to_string(),
        //         name: FAILED_SCRAPINGS_NAME.to_string(),
        //         value: Box::<Gauge<i64, AtomicI64>>::new(Gauge::default()),
        //     },
        // );

        Metrics { gauge_map }
    }

    pub fn get_gauge(self: &Self, name: &str) -> Option<&Gauge> {
        self.gauge_map.get(name)
    }
}

enum GAUGE_METRICS {
    TOTAL_TIME_NAME,
    INCIDENTS_COUNT_NAME,
}

const TOTAL_TIME_NAME: &str = "scrape_duration";
const FAILED_SCRAPINGS_NAME: &str = "scrape_failed";
pub const INCIDENTS_COUNT_NAME: &str = "incidents_count";
