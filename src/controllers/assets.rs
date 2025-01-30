#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::models::_entities::assets;

#[debug_handler]
pub async fn index(Path(pid): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    let project = assets::Model::find_by_pid(&ctx.db, pid).await?;

    format::json(project)
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/assets/").add("/", get(index))
}
