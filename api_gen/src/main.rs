use utoipa::OpenApi;

fn main() {
    let content = web_server::api::ApiDoc::openapi();
    std::fs::write("openapi.yml", content.to_yaml().unwrap()).unwrap();
}
