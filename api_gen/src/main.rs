use std::env;
use utoipa::OpenApi;

fn main() {
    let path = env::args().nth(1).expect("Missing path argument for openapi spec file.");
    let open_api = web_server::api::ApiDoc::openapi();
    match open_api.to_yaml() {
        Ok(yaml_desc) => {
            std::fs::write(path, yaml_desc).unwrap();
        }
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
