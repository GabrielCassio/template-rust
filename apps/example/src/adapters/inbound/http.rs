use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::{adapters::outbound::memory::InMemoryGreetingRepository, application::greeting::GreetingService};

pub type Service = GreetingService<InMemoryGreetingRepository>;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<Service>,
}

#[derive(Deserialize)]
pub struct GreetingQuery {
    pub name: Option<String>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/greetings", get(greeting))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn greeting(
    State(state): State<AppState>,
    Query(query): Query<GreetingQuery>,
) -> Json<crate::domain::Greeting> {
    Json(state.service.greet(query.name.as_deref().unwrap_or("world")))
}

