use std::sync::Arc;
use tokio::process::Command;
use uuid::Uuid;
use warp::http::StatusCode;

use crate::state::State;

pub async fn handle(
    state: Arc<State>,
    id: Uuid,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    let button = state.buttons.get(&id).ok_or_else(warp::reject::not_found)?;

    Command::new("sh")
        .arg("-c")
        .arg(&button.command)
        .status()
        .await
        .map_or_else(
            |_| Ok(StatusCode::INTERNAL_SERVER_ERROR),
            |_| Ok(StatusCode::OK),
        )
}
