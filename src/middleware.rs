use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use tracing::trace;
use crate::app_state::AppState;

pub async fn my_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    trace!("{:?}", state);

    let is_error = false;
    if is_error {
        let res = (StatusCode::BAD_REQUEST, "Something wrong").into_response();
        Ok(res)
    } else {
        let res = next.run(request).await;
        Ok(res)
    }
}
