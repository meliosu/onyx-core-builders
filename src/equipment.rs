use crate::{database::Database, general::FuelType};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate}};

// Types for page endpoints

#[derive(Template)]
pub struct EquipmentPageTemplate;

#[derive(Template, Serialize, Deserialize)]
pub struct EquipmentDetailsTemplate {
    pub id: i64,
}

#[derive(Template)]
pub struct EquipmentNewTemplate;

#[derive(Template, Serialize, Deserialize)]
pub struct EquipmentEditTemplate {
    pub id: i64,
    pub name: String,
    pub amount: u32,
    pub purchase_date: NaiveDateTime,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

// Types for HTMX endpoints

#[derive(Template, Serialize, Deserialize)]
pub struct EquipmentApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub amount: u32,
    pub available_amount: u32,
    pub purchase_date: NaiveDateTime,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentUpdateForm {
    pub name: String,
    pub amount: u32,
    pub purchase_date: NaiveDateTime,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentCreateForm {
    pub name: String,
    pub amount: u32,
    pub purchase_date: NaiveDateTime,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub department_id: Option<i64>,
    pub site_id: Option<i64>,
    pub name: Option<String>,
    pub available: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
pub struct EquipmentListTemplate {
    pub equipment: Vec<EquipmentListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListItem {
    pub id: i64,
    pub name: String,
    pub total_amount: u32,
    pub available_amount: u32,
    pub purchase_date: String,
    pub purchase_cost: f64,
}

#[derive(Template, Serialize, Deserialize)]
pub struct EquipmentAllocationsTemplate {
    pub allocations: Vec<AllocationListItem>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct AllocationListItem {
    pub department_id: i64,
    pub department_name: String,
    pub site_id: i64,
    pub site_name: String,
    pub amount: u32,
    pub period_start: NaiveDateTime,
    pub period_end: NaiveDateTime,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentAllocationForm {
    pub department_id: i64,
    pub site_id: i64,
    pub amount: u32,
    pub period_start: NaiveDateTime,
    pub period_end: NaiveDateTime,
}

// Handler functions for page endpoints

async fn equipment_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return EquipmentPageTemplate
    todo!()
}

async fn equipment_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return EquipmentDetailsTemplate with equipment ID
    todo!()
}

async fn equipment_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return EquipmentNewTemplate
    todo!()
}

async fn equipment_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return EquipmentEditTemplate with equipment data
    todo!()
}

// Handler functions for HTMX endpoints

async fn equipment_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return EquipmentApiDetailsTemplate with equipment data
    todo!()
}

async fn equipment_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<EquipmentUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn equipment_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn equipment_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<EquipmentListFilter>,
) -> Html<String> {
    // Should return EquipmentListTemplate with filtered equipment list
    todo!()
}

async fn equipment_create_handler(
    State(db): State<Database>,
    Form(form): Form<EquipmentCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

async fn equipment_allocations_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Should return EquipmentAllocationsTemplate with allocations for this equipment
    todo!()
}

async fn equipment_create_allocation_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<EquipmentAllocationForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    todo!()
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/equipment", get(equipment_list_handler))
        .route("/equipment/{id}", get(equipment_details_handler))
        .route("/equipment/new", get(equipment_new_handler))
        .route("/equipment/{id}/edit", get(equipment_edit_handler))
        // HTMX endpoints
        .route("/api/equipment/{id}", get(equipment_api_details_handler))
        .route("/api/equipment/{id}", put(equipment_update_handler))
        .route("/api/equipment/{id}", delete(equipment_delete_handler))
        .route("/api/equipment", get(equipment_list_api_handler))
        .route("/api/equipment", post(equipment_create_handler))
        .route("/api/equipment/{id}/allocations", get(equipment_allocations_handler))
        .route("/api/equipment/{id}/allocations", post(equipment_create_allocation_handler))
}