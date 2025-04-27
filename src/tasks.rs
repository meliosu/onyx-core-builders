use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use chrono::NaiveDate;

use crate::{database::Database, general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate}};

// Tab selector for task details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskTab {
    Materials,
    Progress,
}

// Task status
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Planned,
    InProgress,
    Completed,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "tasks/list.html")]
pub struct TasksListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/details.html")]
pub struct TaskDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
#[template(path = "tasks/new.html")]
pub struct TaskNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/edit.html")]
pub struct TaskEditTemplate {
    pub id: i64,
    pub name: String,
    pub site_id: i64,
    pub site_name: String,
    pub brigade_id: Option<i64>,
    pub brigader_name: Option<String>,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct TaskTabQuery {
    pub tab: TaskTab,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/details.html")]
pub struct TaskApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub site_id: i64,
    pub site_name: String,
    pub brigade_id: i64,
    pub brigade_name: String,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub status: TaskStatus,
    pub tab: TaskTab,
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdateForm {
    pub name: String,
    pub description: Option<String>,
    pub site_id: i64,
    pub brigade_id: i64,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCreateForm {
    pub name: String,
    pub description: Option<String>,
    pub site_id: i64,
    pub brigade_id: i64,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub site_id: Option<i64>,
    pub brigade_id: Option<i64>,
    pub status: Option<TaskStatus>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub name: Option<String>,
    pub exceeded_deadline: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/list.html")]
pub struct TaskListTemplate {
    pub tasks: Vec<TaskListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListItem {
    pub id: i64,
    pub name: String,
    pub site_id: i64,
    pub site_name: String,
    pub brigade_id: Option<i64>,
    pub brigader_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub status: TaskStatus,
    pub deadline_exceeded: bool,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/materials.html")]
pub struct TaskMaterialsTemplate {
    pub materials: Vec<TaskMaterialItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialItem {
    pub material_id: i64,
    pub name: String,
    pub expected_amount: f64,
    pub actual_amount: Option<f64>,
    pub units: String,
    pub cost: f64,
    pub total_cost: f64,
    pub excess: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialForm {
    pub material_id: i64,
    pub expected_amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialUpdateForm {
    pub actual_amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCompleteForm {
    pub actual_period_end: NaiveDate,
}

// Handler functions for page endpoints

async fn tasks_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return TasksListTemplate
    todo!()
}

async fn task_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return TaskDetailsTemplate with task ID
    todo!()
}

async fn task_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return TaskNewTemplate
    todo!()
}

async fn task_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return TaskEditTemplate with task data
    todo!()
}

// Handler functions for HTMX endpoints

async fn task_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(query): Query<TaskTabQuery>,
) -> Html<String> {
    // Should return TaskApiDetailsTemplate with task data and the selected tab
    todo!()
}

async fn task_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<TaskUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn task_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn tasks_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<TaskListFilter>,
) -> Html<String> {
    // Should return TaskListTemplate with filtered tasks list
    todo!()
}

async fn task_create_handler(
    State(db): State<Database>,
    Form(form): Form<TaskCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn task_materials_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return TaskMaterialsTemplate with materials for this task
    todo!()
}

async fn task_add_material_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<TaskMaterialForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn task_update_material_handler(
    State(db): State<Database>,
    Path((task_id, material_id)): Path<(i64, i64)>,
    Form(form): Form<TaskMaterialUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn task_complete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<TaskCompleteForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/tasks", get(tasks_list_handler))
        .route("/tasks/{id}", get(task_details_handler))
        .route("/tasks/new", get(task_new_handler))
        .route("/tasks/{id}/edit", get(task_edit_handler))
        // HTMX endpoints
        .route("/api/tasks/{id}", get(task_api_details_handler))
        .route("/api/tasks/{id}", put(task_update_handler))
        .route("/api/tasks/{id}", delete(task_delete_handler))
        .route("/api/tasks", get(tasks_list_api_handler))
        .route("/api/tasks", post(task_create_handler))
        .route("/api/tasks/{id}/materials", get(task_materials_handler))
        .route("/api/tasks/{id}/materials", post(task_add_material_handler))
        .route("/api/tasks/{id}/materials/{material_id}", put(task_update_material_handler))
        .route("/api/tasks/{id}/complete", put(task_complete_handler))
}