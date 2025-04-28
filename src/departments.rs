use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
    Json,
};
use askama::Template;

use crate::{database::Database, general::{Qualification, SiteType}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo};

// Tab selector for department details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DepartmentTab {
    Areas,
    Equipment,
    Sites,
    Personnel,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "departments/list.html")]
pub struct DepartmentsListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/details.html")]
pub struct DepartmentDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "departments/new.html")]
pub struct DepartmentNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/edit.html")]
pub struct DepartmentEditTemplate {
    pub id: i64,
    pub name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct DepartmentTabQuery {
    pub tab: DepartmentTab,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/details.html")]
pub struct DepartmentApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>,
    pub tab: DepartmentTab,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentUpdateForm {
    pub name: String,
    pub supervisor_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentCreateForm {
    pub name: String,
    pub supervisor_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub supervisor_id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/list.html")]
pub struct DepartmentListTemplate {
    pub departments: Vec<DepartmentListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentListItem {
    pub id: i64,
    pub name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/areas.html")]
pub struct DepartmentAreasTemplate {
    pub areas: Vec<AreaListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListItem {
    pub id: i64,
    pub name: String,
    pub supervisor_id: Option<i64>,
    pub supervisor_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/equipment.html")]
pub struct DepartmentEquipmentTemplate {
    pub id: i64,
    pub equipment: Vec<EquipmentListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListItem {
    pub id: i64,
    pub name: String,
    pub amount: u32,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/sites.html")]
pub struct DepartmentSitesTemplate {
    pub id: i64,
    pub sites: Vec<SiteListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListItem {
    pub id: i64,
    pub name: String,
    pub type_: SiteType, 
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/personnel.html")]
pub struct DepartmentPersonnelTemplate {
    pub id: i64,
    pub personnel: Vec<PersonnelListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct PersonnelListItem {
    pub id: i64,
    pub name: String,
    pub qualification: Qualification,
}

// Handler functions for page endpoints

async fn departments_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return DepartmentsListTemplate
    todo!()
}

async fn department_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return DepartmentDetailsTemplate with department ID
    todo!()
}

async fn department_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return DepartmentNewTemplate
    todo!()
}

async fn department_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return DepartmentEditTemplate with department data
    todo!()
}

// Handler functions for HTMX endpoints

async fn department_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(query): Query<DepartmentTabQuery>,
) -> Html<String> {
    // Should return DepartmentApiDetailsTemplate with department data and the selected tab
    todo!()
}

async fn department_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<DepartmentUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn department_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn departments_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<DepartmentListFilter>,
) -> Html<String> {
    // Should return DepartmentListTemplate with filtered departments list
    todo!()
}

async fn department_create_handler(
    State(db): State<Database>,
    Form(form): Form<DepartmentCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn department_areas_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return DepartmentAreasTemplate with areas in this department
    todo!()
}

async fn department_equipment_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return DepartmentEquipmentTemplate with equipment for this department
    todo!()
}

async fn department_sites_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return DepartmentSitesTemplate with sites for this department
    todo!()
}

async fn department_personnel_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return DepartmentPersonnelTemplate with personnel for this department
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/departments", get(departments_list_handler))
        .route("/departments/{id}", get(department_details_handler))
        .route("/departments/new", get(department_new_handler))
        .route("/departments/{id}/edit", get(department_edit_handler))
        // HTMX endpoints
        .route("/api/departments/{id}", get(department_api_details_handler))
        .route("/api/departments/{id}", put(department_update_handler))
        .route("/api/departments/{id}", delete(department_delete_handler))
        .route("/api/departments", get(departments_list_api_handler))
        .route("/api/departments", post(department_create_handler))
        .route("/api/departments/{id}/areas", get(department_areas_handler))
        .route("/api/departments/{id}/equipment", get(department_equipment_handler))
        .route("/api/departments/{id}/sites", get(department_sites_handler))
        .route("/api/departments/{id}/personnel", get(department_personnel_handler))
}