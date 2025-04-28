use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Qualification, SiteType, Position}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Tab selector for area details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AreaTab {
    Sites,
    Personnel,
}

// Types for page endpoints

// #[derive(Template)]
// #[template(path = "areas/list.html")]
pub struct AreasListTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/details.html")]
pub struct AreaDetailsTemplate {
    pub id: i64,
}

// #[derive(Template)]
// #[template(path = "areas/new.html")]
pub struct AreaNewTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/edit.html")]
pub struct AreaEditTemplate {
    pub id: i64,
    pub name: String,
    pub department_id: i64,
    pub department_name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct AreaTabQuery {
    pub tab: AreaTab,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/api/details.html")]
pub struct AreaApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub department_id: i64,
    pub department_name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>,
    pub tab: AreaTab,
}

#[derive(Serialize, Deserialize)]
pub struct AreaUpdateForm {
    pub name: String,
    pub department_id: i64,
    pub supervisor_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaCreateForm {
    pub name: String,
    pub department_id: i64,
    pub supervisor_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub department_id: Option<i64>,
    pub supervisor_id: Option<i64>,
    pub name: Option<String>,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/api/list.html")]
pub struct AreaListTemplate {
    pub areas: Vec<AreaListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListItem {
    pub id: i64,
    pub name: String,
    pub department_id: i64,
    pub department_name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/api/sites.html")]
pub struct AreaSitesTemplate {
    pub sites: Vec<SiteListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListItem {
    pub id: i64,
    pub name: String,
    pub type_: SiteType,
    pub client_id: i64,
    pub client_name: String,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "areas/api/personnel.html")]
pub struct AreaPersonnelTemplate {
    pub personnel: Vec<PersonnelListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct PersonnelListItem {
    pub id: i64,
    pub name: String,
    pub qualification: Qualification,
    pub position: Position,
}

// Handler functions for page endpoints

async fn areas_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return AreasListTemplate
    Html::from(String::new())
}

async fn area_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return AreaDetailsTemplate with area ID
    Html::from(String::new())
}

async fn area_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return AreaNewTemplate
    Html::from(String::new())
}

async fn area_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return AreaEditTemplate with area data
    Html::from(String::new())
}

// Handler functions for HTMX endpoints

async fn area_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(query): Query<AreaTabQuery>,
) -> Html<String> {
    // Should return AreaApiDetailsTemplate with area data and the selected tab
    Html::from(String::new())
}

async fn area_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<AreaUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn area_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn areas_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<AreaListFilter>,
) -> Html<String> {
    // Should return AreaListTemplate with filtered areas list
    Html::from(String::new())
}

async fn area_create_handler(
    State(db): State<Database>,
    Form(form): Form<AreaCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn area_sites_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return AreaSitesTemplate with sites in this area
    Html::from(String::new())
}

async fn area_personnel_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return AreaPersonnelTemplate with personnel for this area
    Html::from(String::new())
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/areas", get(areas_list_handler))
        .route("/areas/{id}", get(area_details_handler))
        .route("/areas/new", get(area_new_handler))
        .route("/areas/{id}/edit", get(area_edit_handler))
        // HTMX endpoints
        .route("/api/areas/{id}", get(area_api_details_handler))
        .route("/api/areas/{id}", put(area_update_handler))
        .route("/api/areas/{id}", delete(area_delete_handler))
        .route("/api/areas", get(areas_list_api_handler))
        .route("/api/areas", post(area_create_handler))
        .route("/api/areas/{id}/sites", get(area_sites_handler))
        .route("/api/areas/{id}/personnel", get(area_personnel_handler))
}