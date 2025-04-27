use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Qualification, Position, Gender}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Qualification-specific fields
#[derive(Serialize, Deserialize)]
#[serde(tag = "qualification")]
pub enum QualificationFields {
    Technician(TechnicianFields),
    Technologist(TechnologistFields),
    Engineer(EngineerFields),
}

// Technical personnel qualification-specific field structs
#[derive(Serialize, Deserialize)]
pub struct TechnicianFields {
    pub safety_training_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct TechnologistFields {
    pub management_tools: String,
}

#[derive(Serialize, Deserialize)]
pub struct EngineerFields {
    pub pe_license_id: i32,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "technical_personnel/list.html")]
pub struct TechnicalPersonnelListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/details.html")]
pub struct TechnicalPersonnelDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "technical_personnel/new.html")]
pub struct TechnicalPersonnelNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/edit.html")]
pub struct TechnicalPersonnelEditTemplate {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub qualification: Qualification,
    pub position: Position,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelQualificationQuery {
    pub qualification: Qualification,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/details.html")]
pub struct TechnicalPersonnelApiDetailsTemplate {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    pub position: Position,
    pub education_level: String,
    pub software_skills: Option<String>,
    pub is_project_manager: bool,
    #[serde(flatten)]
    pub qualification_fields: QualificationFields,
    pub supervising_department_id: Option<i64>,
    pub supervising_department_name: Option<String>,
    pub supervising_area_id: Option<i64>,
    pub supervising_area_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/qualification-fields.html")]
pub struct TechnicalPersonnelQualificationFieldsTemplate {
    pub qualification: Qualification,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelUpdateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    pub position: Position,
    pub education_level: String,
    pub software_skills: Option<String>,
    pub is_project_manager: bool,
    #[serde(flatten)]
    pub qualification_fields: QualificationFields,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelCreateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    pub position: Position,
    pub education_level: String,
    pub software_skills: Option<String>,
    pub is_project_manager: bool,
    #[serde(flatten)]
    pub qualification_fields: QualificationFields,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub qualification: Option<Qualification>,
    pub position: Option<Position>,
    pub department_id: Option<i64>,
    pub area_id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/list.html")]
pub struct TechnicalPersonnelListTemplate {
    pub technical_personnel: Vec<TechnicalPersonnelListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelListItem {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub qualification: Qualification,
    pub position: Position,
    pub department_id: Option<i64>,
    pub department_name: Option<String>,
    pub area_id: Option<i64>,
    pub area_name: Option<String>,
}

// Handler functions for page endpoints

async fn technical_personnel_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return TechnicalPersonnelListTemplate
    todo!()
}

async fn technical_personnel_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return TechnicalPersonnelDetailsTemplate with personnel ID
    todo!()
}

async fn technical_personnel_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return TechnicalPersonnelNewTemplate
    todo!()
}

async fn technical_personnel_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return TechnicalPersonnelEditTemplate with personnel data
    todo!()
}

// Handler functions for HTMX endpoints

async fn technical_personnel_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return TechnicalPersonnelApiDetailsTemplate with personnel data
    todo!()
}

async fn technical_personnel_qualification_fields_handler(
    State(db): State<Database>,
    Query(query): Query<TechnicalPersonnelQualificationQuery>,
) -> Html<String> {
    // Should return TechnicalPersonnelQualificationFieldsTemplate for the specific qualification
    todo!()
}

async fn technical_personnel_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<TechnicalPersonnelUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn technical_personnel_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn technical_personnel_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<TechnicalPersonnelListFilter>,
) -> Html<String> {
    // Should return TechnicalPersonnelListTemplate with filtered personnel list
    todo!()
}

async fn technical_personnel_create_handler(
    State(db): State<Database>,
    Form(form): Form<TechnicalPersonnelCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/technical-personnel", get(technical_personnel_list_handler))
        .route("/technical-personnel/{id}", get(technical_personnel_details_handler))
        .route("/technical-personnel/new", get(technical_personnel_new_handler))
        .route("/technical-personnel/{id}/edit", get(technical_personnel_edit_handler))
        // HTMX endpoints
        .route("/api/technical-personnel/{id}", get(technical_personnel_api_details_handler))
        .route("/api/technical-personnel/qualification-fields", get(technical_personnel_qualification_fields_handler))
        .route("/api/technical-personnel/{id}", put(technical_personnel_update_handler))
        .route("/api/technical-personnel/{id}", delete(technical_personnel_delete_handler))
        .route("/api/technical-personnel", get(technical_personnel_list_api_handler))
        .route("/api/technical-personnel", post(technical_personnel_create_handler))
}