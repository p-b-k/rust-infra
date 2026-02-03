////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Passthrough router
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use axum::{Json, Router, extract::State, routing::get};

use mysql::prelude::{FromRow, Queryable};
use std::sync::Arc;

use infra::state::AppState;
use ui::table::{ColumnDef, TableDef};

use crate::data::{Product, Service};

use log::debug;

pub fn pass_through_router (app : Arc<AppState>) -> Routner<()> {
    Router::new().with_state(app);
}
