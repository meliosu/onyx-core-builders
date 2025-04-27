use crate::database::Database;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Qualification, SiteType, Position, Profession}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Tab selector for brigade details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BrigadeTab {
    Workers,
    Tasks,
    Current,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "brigades/list.html")]
pub struct BrigadesListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/details.html")]
pub struct BrigadeDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "brigades/new.html")]
pub struct BrigadeNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/edit.html")]
pub struct BrigadeEditTemplate {
    pub id: i64,
    pub brigadier_id: i64,
    pub brigadier_name: String,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct BrigadeTabQuery {
    pub tab: BrigadeTab,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/details.html")]
pub struct BrigadeApiDetailsTemplate {
    pub id: i64,
    pub brigadier_id: i64,
    pub brigadier_name: String,
    pub worker_count: u32,
    pub current_task_id: Option<i64>,
    pub current_task_name: Option<String>,
    pub current_site_id: Option<i64>,
    pub current_site_name: Option<String>,
    pub tab: BrigadeTab,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeUpdateForm {
    pub brigadier_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeCreateForm {
    pub brigadier_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub brigadier_id: Option<i64>,
    pub site_id: Option<i64>,
    pub task_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/list.html")]
pub struct BrigadeListTemplate {
    pub brigades: Vec<BrigadeListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeListItem {
    pub id: i64,
    pub brigadier_id: i64,
    pub brigadier_name: String,
    pub worker_count: u32,
    pub current_site_id: Option<i64>,
    pub current_site_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/workers.html")]
pub struct BrigadeWorkersTemplate {
    pub workers: Vec<WorkerListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerListItem {
    pub id: i64,
    pub name: String,
    pub profession: Profession,
    pub is_brigadier: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerAddForm {
    pub worker_id: i64,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/tasks.html")]
pub struct BrigadeTasksTemplate {
    pub tasks: Vec<TaskListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListItem {
    pub id: i64,
    pub name: String,
    pub site_id: i64,
    pub site_name: String,
    pub period_start: NaiveDateTime,
    pub period_end: Option<NaiveDateTime>,
    pub status: String,
}

// Handler functions for page endpoints

async fn brigades_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return BrigadesListTemplate
    todo!()
}

async fn brigade_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return BrigadeDetailsTemplate with brigade ID
    todo!()
}

async fn brigade_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return BrigadeNewTemplate
    todo!()
}

async fn brigade_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return BrigadeEditTemplate with brigade data
    todo!()
}

// Handler functions for HTMX endpoints

async fn brigade_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(query): Query<BrigadeTabQuery>,
) -> Html<String> {
    // Should return BrigadeApiDetailsTemplate with brigade data and the selected tab
    todo!()
}

async fn brigade_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<BrigadeUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn brigade_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn brigades_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<BrigadeListFilter>,
) -> Html<String> {
    // Should return BrigadeListTemplate with filtered brigades list
    todo!()
}

async fn brigade_create_handler(
    State(db): State<Database>,
    Form(form): Form<BrigadeCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn brigade_workers_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return BrigadeWorkersTemplate with workers assigned to this brigade
    todo!()
}

async fn worker_add_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<WorkerAddForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn worker_remove_handler(
    State(db): State<Database>,
    Path((id, worker_id)): Path<(i64, i64)>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn brigade_tasks_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return BrigadeTasksTemplate with task history for this brigade
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/brigades", get(brigades_list_handler))
        .route("/brigades/{id}", get(brigade_details_handler))
        .route("/brigades/new", get(brigade_new_handler))
        .route("/brigades/{id}/edit", get(brigade_edit_handler))
        // HTMX endpoints
        .route("/api/brigades/{id}", get(brigade_api_details_handler))
        .route("/api/brigades/{id}", put(brigade_update_handler))
        .route("/api/brigades/{id}", delete(brigade_delete_handler))
        .route("/api/brigades", get(brigades_list_api_handler))
        .route("/api/brigades", post(brigade_create_handler))
        .route("/api/brigades/{id}/workers", get(brigade_workers_handler))
        .route("/api/brigades/{id}/workers", post(worker_add_handler))
        .route("/api/brigades/{id}/workers/{worker_id}", delete(worker_remove_handler))
        .route("/api/brigades/{id}/tasks", get(brigade_tasks_handler))
}