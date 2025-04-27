use chrono::{Date, NaiveDate, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Qualification, SiteType, RiskLevel, Position}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Tab selector for site details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SiteTab {
    Schedule,
    Materials,
    Equipment,
    Brigades,
    Reports,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SiteFields {
    PowerPlant(PowerPlantFields),
    Road(RoadFields),
    Housing(HousingFields),
    Bridge(BridgeFields),
    Park(ParkFields),
}

// Site type-specific field structs
#[derive(Serialize, Deserialize)]
pub struct PowerPlantFields {
    pub energy_output: f64,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RoadFields {
    pub length: f64,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Serialize, Deserialize)]
pub struct HousingFields {
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub energy_efficiency: String,
}

#[derive(Serialize, Deserialize)]
pub struct BridgeFields {
    pub length: f64,
    pub road_material: String,
    pub max_load: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ParkFields {
    pub area: f64,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "sites/list.html")]
pub struct SitesListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/details.html")]
pub struct SiteDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "sites/new.html")]
pub struct SiteNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/edit.html")]
pub struct SiteEditTemplate {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub area_id: i64,
    pub area_name: String,
    pub client_id: i64,
    pub client_name: String,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct SiteTabQuery {
    pub tab: SiteTab,
}

#[derive(Serialize, Deserialize)]
pub struct SiteTypeFieldsQuery {
    pub type_: SiteType,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/details.html")]
pub struct SiteApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub area_id: i64,
    pub area_name: String,
    pub client_id: i64,
    pub client_name: String,
    pub location: String,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    pub tab: SiteTab,
    #[serde(flatten)]
    pub type_fields: SiteFields,
}

#[derive(Serialize, Deserialize)]
pub struct SiteUpdateForm {
    pub name: String,
    pub area_id: i64,
    pub client_id: i64,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub location: String,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    #[serde(flatten)]
    pub type_fields: SiteFields,
}

#[derive(Serialize, Deserialize)]
pub struct SiteCreateForm {
    pub name: String,
    pub area_id: i64,
    pub client_id: i64,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub location: String,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    #[serde(flatten)]
    pub type_fields: SiteFields,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub area_id: Option<i64>,
    pub department_id: Option<i64>,
    pub client_id: Option<i64>,
    #[serde(rename = "type")]
    pub type_: Option<SiteType>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/list.html")]
pub struct SiteListTemplate {
    pub sites: Vec<SiteListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListItem {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub area_id: i64,
    pub area_name: String,
    pub department_id: i64,
    pub department_name: String,
    pub client_id: i64,
    pub client_name: String,
    pub status: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/schedule.html")]
pub struct SiteScheduleTemplate {
    pub tasks: Vec<TaskListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListItem {
    pub id: i64,
    pub name: String,
    pub brigade_id: Option<i64>,
    pub brigader_name: Option<String>,
    pub period_start: NaiveDateTime,
    pub expected_period_end: NaiveDateTime,
    pub actual_period_end: Option<NaiveDateTime>,
    pub status: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/materials.html")]
pub struct SiteMaterialsTemplate {
    pub materials: Vec<MaterialListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialListItem {
    pub id: i64,
    pub name: String,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    pub units: String,
    pub cost: f64,
    pub total_cost: f64,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/equipment.html")]
pub struct SiteEquipmentTemplate {
    pub equipment: Vec<EquipmentListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListItem {
    pub id: i64,
    pub name: String,
    pub amount: u32,
    pub period_start: NaiveDateTime,
    pub period_end: NaiveDateTime,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/brigades.html")]
pub struct SiteBrigadesTemplate {
    pub brigades: Vec<BrigadeListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeListItem {
    pub id: i64,
    pub brigadier_id: Option<i64>,
    pub brigadier_name: Option<String>,
    pub worker_count: u32,
    pub current_task: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/reports.html")]
pub struct SiteReportsTemplate {
    pub reports: Vec<ReportListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct ReportListItem {
    pub task_id: i64,
    pub task_name: String,
    pub period_start: NaiveDateTime,
    pub expected_period_end: NaiveDateTime,
    pub actual_period_end: Option<NaiveDateTime>,
    pub delay: u32,
}

// Handler functions for page endpoints

async fn sites_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return SitesListTemplate
    todo!()
}

async fn site_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return SiteDetailsTemplate with site ID
    todo!()
}

async fn site_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return SiteNewTemplate
    todo!()
}

async fn site_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return SiteEditTemplate with site data
    todo!()
}

// Handler functions for HTMX endpoints

async fn site_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(query): Query<SiteTabQuery>,
) -> Html<String> {
    // Should return SiteApiDetailsTemplate with site data and the selected tab
    todo!()
}

async fn site_type_fields_handler(
    State(db): State<Database>,
    Query(query): Query<SiteTypeFieldsQuery>,
) -> Html<String> {
    // Should return SiteTypeFieldsTemplate with form fields specific to site type
    todo!()
}

async fn site_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<SiteUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn site_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn sites_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<SiteListFilter>,
) -> Html<String> {
    // Should return SiteListTemplate with filtered sites list
    todo!()
}

async fn site_create_handler(
    State(db): State<Database>,
    Form(form): Form<SiteCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn site_schedule_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return SiteScheduleTemplate with tasks for this site
    todo!()
}

async fn site_materials_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return SiteMaterialsTemplate with materials for this site
    todo!()
}

async fn site_equipment_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return SiteEquipmentTemplate with equipment allocated to this site
    todo!()
}

async fn site_brigades_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return SiteBrigadesTemplate with brigades assigned to this site
    todo!()
}

async fn site_reports_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return SiteReportsTemplate with reports for this site
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/sites", get(sites_list_handler))
        .route("/sites/{id}", get(site_details_handler))
        .route("/sites/new", get(site_new_handler))
        .route("/sites/{id}/edit", get(site_edit_handler))
        // HTMX endpoints
        .route("/api/sites/{id}", get(site_api_details_handler))
        .route("/api/sites/type-fields", get(site_type_fields_handler))
        .route("/api/sites/{id}", put(site_update_handler))
        .route("/api/sites/{id}", delete(site_delete_handler))
        .route("/api/sites", get(sites_list_api_handler))
        .route("/api/sites", post(site_create_handler))
        .route("/api/sites/{id}/schedule", get(site_schedule_handler))
        .route("/api/sites/{id}/materials", get(site_materials_handler))
        .route("/api/sites/{id}/equipment", get(site_equipment_handler))
        .route("/api/sites/{id}/brigades", get(site_brigades_handler))
        .route("/api/sites/{id}/reports", get(site_reports_handler))
}