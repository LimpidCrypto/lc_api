#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::models::_entities::projects;

#[debug_handler]
pub async fn index(State(ctx): State<AppContext>) -> Result<Response> {
    let all_projects = projects::Entity::find().all(&ctx.db).await?;
    format::json(all_projects)
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/projects/").add("/", get(index))
}
