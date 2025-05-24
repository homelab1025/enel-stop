use utoipa::OpenApi;

fn main() {
    let open_api = web_server::api::ApiDoc::openapi();
    match open_api.to_yaml() {
        Ok(yaml_desc) => {
            std::fs::write("openapi.yml", yaml_desc).unwrap();
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
