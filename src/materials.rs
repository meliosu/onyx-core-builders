use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::database::Database;
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Types for page endpoints

// #[derive(Template)]
// #[template(path = "materials/list.html")]
pub struct MaterialsListTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "materials/details.html")]
pub struct MaterialDetailsTemplate {
    pub id: i64,
}

// #[derive(Template)]
// #[template(path = "materials/new.html")]
pub struct MaterialNewTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "materials/edit.html")]
pub struct MaterialEditTemplate {
    pub id: i64,
    pub name: String,
    pub cost: f64,
    pub units: String,
}

// Types for HTMX endpoints

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "materials/api/details.html")]
pub struct MaterialApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub cost: f64,
    pub units: String,
    pub total_estimated: f64,
    pub total_actual: f64,
    pub total_cost: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialUpdateForm {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialCreateForm {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub name: Option<String>,
    pub cost_min: Option<f64>,
    pub cost_max: Option<f64>,
    pub excess_usage: Option<bool>,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "materials/api/list.html")]
pub struct MaterialListTemplate {
    pub materials: Vec<MaterialListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialListItem {
    pub id: i64,
    pub name: String,
    pub cost: f64,
    pub units: String,
    pub estimated_spendings: f64,
    pub actual_spendings: f64,
    pub excess: bool,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "materials/api/usage.html")]
pub struct MaterialUsageTemplate {
    pub usage: Vec<UsageListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct UsageListItem {
    pub task_id: i64,
    pub task_name: String,
    pub site_id: i64,
    pub site_name: String,
    pub expected_amount: f64,
    pub actual_amount: f64,
    pub excess_amount: f64,
    pub total_cost: f64,
}

// Handler functions for page endpoints

async fn materials_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return MaterialsListTemplate
    Html::from(String::new())
}

async fn material_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return MaterialDetailsTemplate with material ID
    Html::from(String::new())
}

async fn material_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return MaterialNewTemplate
    Html::from(String::new())
}

async fn material_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return MaterialEditTemplate with material data
    Html::from(String::new())
}

// Handler functions for HTMX endpoints

async fn material_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return MaterialApiDetailsTemplate with material details
    Html::from(String::new())
}

async fn material_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<MaterialUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn material_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn materials_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<MaterialListFilter>,
) -> Html<String> {
    // Should return MaterialListTemplate with filtered materials list
    Html::from(String::new())
}

async fn material_create_handler(
    State(db): State<Database>,
    Form(form): Form<MaterialCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn material_usage_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return MaterialUsageTemplate with usage history for this material
    Html::from(String::new())
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/materials", get(materials_list_handler))
        .route("/materials/{id}", get(material_details_handler))
        .route("/materials/new", get(material_new_handler))
        .route("/materials/{id}/edit", get(material_edit_handler))
        // HTMX endpoints
        .route("/api/materials/{id}", get(material_api_details_handler))
        .route("/api/materials/{id}", put(material_update_handler))
        .route("/api/materials/{id}", delete(material_delete_handler))
        .route("/api/materials", get(materials_list_api_handler))
        .route("/api/materials", post(material_create_handler))
        .route("/api/materials/{id}/usage", get(material_usage_handler))
}