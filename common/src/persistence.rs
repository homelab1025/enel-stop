pub const SORTED_INCIDENTS_KEY: &str = "incidents:sorted";
pub fn generate_redis_key(incident_id: &str) -> String {
    format!("incidents:{}", incident_id)
}