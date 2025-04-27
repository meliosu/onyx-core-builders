use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Profession, Gender}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Profession-specific fields
#[derive(Serialize, Deserialize)]
#[serde(tag = "profession")]
pub enum ProfessionFields {
    Electrician(ElectricianFields),
    Plumber(PlumberFields),
    Welder(WelderFields),
    Driver(DriverFields),
    Mason(MasonFields),
}

// Worker profession-specific field structs
#[derive(Serialize, Deserialize)]
pub struct ElectricianFields {
    pub voltage_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlumberFields {
    pub pipe_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct WelderFields {
    pub welding_machine: String,
}

#[derive(Serialize, Deserialize)]
pub struct DriverFields {
    pub vehicle_type: String,
    pub number_of_accidents: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MasonFields {
    pub hq_restoration_skills: bool,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "workers/list.html")]
pub struct WorkersListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/details.html")]
pub struct WorkerDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "workers/new.html")]
pub struct WorkerNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/edit.html")]
pub struct WorkerEditTemplate {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub profession: Profession,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct WorkerProfessionQuery {
    pub profession: Profession,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/details.html")]
pub struct WorkerApiDetailsTemplate {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i64>,
    pub brigade_name: Option<String>,
    pub is_brigadier: bool,
    #[serde(flatten)]
    pub profession_fields: ProfessionFields,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/profession-fields.html")]
pub struct WorkerProfessionFieldsTemplate {
    pub profession: Profession,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerUpdateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i64>,
    #[serde(flatten)]
    pub profession_fields: ProfessionFields,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerCreateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i64>,
    #[serde(flatten)]
    pub profession_fields: ProfessionFields,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub profession: Option<Profession>,
    pub brigade_id: Option<i64>,
    pub is_brigadier: Option<bool>,
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/list.html")]
pub struct WorkerListTemplate {
    pub workers: Vec<WorkerListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerListItem {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub profession: Profession,
    pub brigade_id: Option<i64>,
    pub brigade_name: Option<String>,
    pub is_brigadier: bool,
}

// Handler functions for page endpoints

async fn workers_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return WorkersListTemplate
    todo!()
}

async fn worker_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return WorkerDetailsTemplate with worker ID
    todo!()
}

async fn worker_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return WorkerNewTemplate
    todo!()
}

async fn worker_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return WorkerEditTemplate with worker data
    todo!()
}

// Handler functions for HTMX endpoints

async fn worker_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return WorkerApiDetailsTemplate with worker data
    todo!()
}

async fn worker_profession_fields_handler(
    State(db): State<Database>,
    Query(query): Query<WorkerProfessionQuery>,
) -> Html<String> {
    // Should return WorkerProfessionFieldsTemplate for the specific profession
    todo!()
}

async fn worker_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<WorkerUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn worker_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn workers_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<WorkerListFilter>,
) -> Html<String> {
    // Should return WorkerListTemplate with filtered workers list
    todo!()
}

async fn worker_create_handler(
    State(db): State<Database>,
    Form(form): Form<WorkerCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/workers", get(workers_list_handler))
        .route("/workers/{id}", get(worker_details_handler))
        .route("/workers/new", get(worker_new_handler))
        .route("/workers/{id}/edit", get(worker_edit_handler))
        // HTMX endpoints
        .route("/api/workers/{id}", get(worker_api_details_handler))
        .route("/api/workers/profession-fields", get(worker_profession_fields_handler))
        .route("/api/workers/{id}", put(worker_update_handler))
        .route("/api/workers/{id}", delete(worker_delete_handler))
        .route("/api/workers", get(workers_list_api_handler))
        .route("/api/workers", post(worker_create_handler))
}