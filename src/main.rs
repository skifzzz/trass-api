mod cli;
mod dto;

use std::env;
use std::sync::Arc;

use axum::{Router, Json};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, put};
use cli::Arguments;
use dto::recommendations::Recommendations;
use trass_core::config::Config;
use trass_core::data_access::DataAccessService;

use tower_http::cors::{Any, CorsLayer};
use http::Method;

use serde_json::json;

#[tokio::main]
async fn main() {
    let arguments = Arguments::parse_or_default();
    let config_dir = arguments.config_dir.expect("Can't read config");

    let config = Config::init(config_dir).expect("Can't init config");
    let order_history_dir = arguments.data_dir.as_ref().expect("Can't find data dir");

    let data_access_service = DataAccessService::new(&config, order_history_dir).await;
    // data_access_service.register_order_history_watcher(order_history_dir).expect("Can't register wather");

    let shared_service = Arc::new(data_access_service);


    shared_service
        .import_order_history_from(order_history_dir)
        .await;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(Any);

    let app = Router::new()
        .route("/history/reload", put(reload_history))
        .route("/recommendation", get(get_recommendations))
        .layer(cors)
        .with_state(shared_service);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}

async fn reload_history(State(das): State<Arc<DataAccessService>>) -> impl IntoResponse {
    let path_buf = env::current_dir().map(|p| p.join("data")).ok().expect("Can't find data dir");
    das.import_order_history_from(path_buf.as_path())
        .await;

    "reload history"
}

async fn get_recommendations(State(das): State<Arc<DataAccessService>>) -> impl IntoResponse {
    match das.get_recommendations().await {
        Ok(recommendations) => {
            Json(json!(Recommendations::from(recommendations)))
        }
        Err(e) => Json(json!(format!("Not available, {:?}",e)))
    }
}