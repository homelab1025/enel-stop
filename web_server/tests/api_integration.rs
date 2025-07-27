mod common;

mod tests {
    use crate::common::{create_app_state, TestInfrastructure, FILTERING_COUNTY};
    use axum::extract::{Query, State};
    use web_server::web_api::{GetIncidentsResponse, Incident, IncidentsFiltering, RecordCount};

    #[tokio::test]
    async fn test_api_count() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        let resp = web_server::web_api::count_incidents(State(state)).await;
        assert!(resp.is_ok());

        let json: RecordCount = resp.expect("Should be OK").0;
        assert_eq!(5, json.total_count);
    }

    #[tokio::test]
    async fn test_get_all_incidents() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        let filtering = IncidentsFiltering { ..Default::default() };

        let resp = web_server::web_api::get_all_incidents(State(state), Query(filtering)).await;
        assert!(resp.is_ok());

        let json: GetIncidentsResponse = resp.expect("Should be OK").0;
        assert_eq!(5, json.incidents.len());
    }

    #[tokio::test]
    async fn test_get_all_incidents_filter_county() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        let filtering = IncidentsFiltering {
            county: Some(FILTERING_COUNTY.to_string()),
            ..Default::default()
        };

        let resp = web_server::web_api::get_all_incidents(State(state), Query(filtering)).await;
        assert!(resp.is_ok());

        let json: GetIncidentsResponse = resp.expect("Should be OK").0;
        assert_eq!(2, json.incidents.len());
    }

    #[tokio::test]
    async fn test_ping() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        let resp = web_server::web_api::ping(State(state)).await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_incidents_with_offset() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        // Get all incidents first to determine their order
        let all_filtering = Default::default();

        let all_resp = web_server::web_api::get_all_incidents(State(state.clone()), Query(all_filtering)).await;
        assert!(all_resp.is_ok());

        let all_incidents = all_resp.expect("Should be OK").0.incidents;
        assert_eq!(5, all_incidents.len());

        // Now get incidents with offset 2
        let offset_filtering = IncidentsFiltering {
            offset: Some(2),
            ..Default::default()
        };

        let offset_resp = web_server::web_api::get_all_incidents(State(state), Query(offset_filtering)).await;
        assert!(offset_resp.is_ok());

        let offset_incidents: GetIncidentsResponse = offset_resp.expect("Should be OK").0;
        assert_eq!(3, offset_incidents.incidents.len()); // Should return 3 incidents (out of 5)

        // Check that the 3 incidents are the last 3 from the all_incidents array
        for (i, incident) in offset_incidents.incidents.iter().enumerate() {
            assert_eq!(format!("{:?}", incident), format!("{:?}", all_incidents[i + 2]));
        }
    }

    #[tokio::test]
    async fn test_get_all_incidents_with_count() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        // Get all incidents first to determine their order
        let all_filtering = Default::default();

        let all_resp = web_server::web_api::get_all_incidents(State(state.clone()), Query(all_filtering)).await;
        assert!(all_resp.is_ok());

        let all_incidents = all_resp.expect("Should be OK").0.incidents;
        assert_eq!(5, all_incidents.len());

        // Get incidents with count 2
        let count_filtering = IncidentsFiltering {
            count: Some(2),
            ..Default::default()
        };

        let count_resp = web_server::web_api::get_all_incidents(State(state), Query(count_filtering)).await;
        assert!(count_resp.is_ok());

        let count_incidents: Vec<Incident> = count_resp.expect("Should be OK").0.incidents;
        assert_eq!(2, count_incidents.len()); // Should return only 2 incidents

        // Check that the 2 incidents are the first 2 from the all_incidents array
        for (i, incident) in count_incidents.iter().enumerate() {
            assert_eq!(format!("{:?}", incident), format!("{:?}", all_incidents[i]));
        }
    }

    #[tokio::test]
    async fn test_get_all_incidents_with_offset_and_count() {
        let infra = TestInfrastructure::new().await;
        let state = create_app_state(&infra).await;

        // Get all incidents first to determine their order
        let all_filtering = Default::default();

        let all_incidents = web_server::web_api::get_all_incidents(State(state.clone()), Query(all_filtering)).await;
        assert!(all_incidents.is_ok());

        let all_incidents: Vec<Incident> = all_incidents.expect("Should be OK").0.incidents;
        assert_eq!(5, all_incidents.len());

        // Get incidents with offset 1 and count 2
        let filtering = IncidentsFiltering {
            offset: Some(1),
            count: Some(2),
            ..Default::default()
        };

        let resp = web_server::web_api::get_all_incidents(State(state), Query(filtering)).await;
        assert!(resp.is_ok());

        let incidents: Vec<Incident> = resp.expect("Should be OK").0.incidents;
        assert_eq!(2, incidents.len()); // Should return exactly 2 incidents

        // Check that the 2 incidents are the correct ones from the all_incidents array
        for (i, incident) in incidents.iter().enumerate() {
            assert_eq!(format!("{:?}", incident), format!("{:?}", all_incidents[i + 1]));
        }
    }
}
