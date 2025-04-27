use serde::{Serialize, Deserialize};
use axum::{
    extract::{Query, State}, http::StatusCode, response::Html, routing::get, Json
};
use askama::Template;

use crate::database::Database;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Meidum,
    High,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Profession {
    Electrician,
    Plumber,
    Welder,
    Driver,
    Mason,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Qualification {
    Technician,
    Technologist,
    Engineer,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Master,
    Foreman,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationResult {
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub page_number: u32,
    pub page_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
    pub sort_by: String,
    pub sort_direction: SortDirection,
}

#[derive(Serialize, Deserialize)]
pub struct QueryInfo {
    pub num_pages: u32,
    pub num_items: u32,
}

// Template types
#[derive(Template)]
#[template(path = "general/index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "general/404.html")]
pub struct NotFoundTemplate;

#[derive(Template)]
#[template(path = "general/500.html")]
pub struct ServerErrorTemplate {
    pub message: String,
}

#[derive(Template)]
#[template(path = "general/notification.html")]
pub struct NotificationTemplate {
    pub result: NotificationResult,
    pub message: Option<String>,
    pub redirect: Option<String>,
}

// Handler functions
async fn index_handler(State(db): State<Database>) -> Html<String> {
    // Should return IndexTemplate (empty page for now)
    todo!()
}

async fn not_found_handler(State(db): State<Database>) -> Html<String> {
    // Should return NotFoundTemplate
    todo!()
}

async fn server_error_handler(
    State(db): State<Database>,
) -> Html<String> {
    // Should return ServerErrorTemplate with error message
    todo!()
}

pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        .route("/", get(index_handler))
        .route("/404", get(not_found_handler))
        .route("/500", get(server_error_handler))
}