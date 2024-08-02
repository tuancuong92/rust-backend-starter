use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};

use tokio;
use tokio::net::TcpListener;
use tracing::trace;
use tracing_subscriber;

use vihub_faker::{
    app_state::AppState,
    router::{foo_router, root_handler, handler_not_found},
    middleware::my_middleware
};

#[tokio::main]
async fn main() {
    //initialize tracing
    tracing_subscriber::fmt::init();

    let shared_state = AppState {
        max_retries: 100,
        min_number: 0,
        access_token: None,
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/foo", foo_router())
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            my_middleware,
        ))
        .with_state(shared_state);

    let app = app.fallback(handler_not_found);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    trace!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

