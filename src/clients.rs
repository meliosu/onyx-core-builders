use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;

use crate::{database::Database, general::{SiteType, NotificationResult, NotificationTemplate}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo};

// Types for page endpoints

// #[derive(Template)]
// #[template(path = "clients/list.html")]
pub struct ClientsListTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "clients/details.html")]
pub struct ClientDetailsTemplate {
    pub id: i64,
}

// #[derive(Template)]
// #[template(path = "clients/new.html")]
pub struct ClientNewTemplate;

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "clients/edit.html")]
pub struct ClientEditTemplate {
    pub id: i64,
    pub name: String,
    pub inn: i64,
    pub is_vip: bool
}

// Types for HTMX endpoints

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "clients/api/details.html")]
pub struct ClientApiDetailsTemplate {
    pub id: i64,
    pub name: String,
    pub inn: i64,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
    pub sites: Vec<ClientSite>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientSite {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientUpdateForm {
    pub name: String,
    pub inn: i64,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateForm {
    pub name: String,
    pub inn: i64,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ClientListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub name: Option<String>,
    pub inn: Option<i64>,
    pub is_vip: Option<bool>,
}

// #[derive(Template, Serialize, Deserialize)]
// #[template(path = "clients/api/list.html")]
pub struct ClientListTemplate {
    pub clients: Vec<ClientListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ClientListItem {
    pub id: i64,
    pub name: String,
    pub inn: i64,
    pub is_vip: bool,
}

// Handler functions for page endpoints

async fn clients_list_handler(State(db): State<Database>) -> Html<String> {
    // Should return ClientsListTemplate
    Html::from(String::new())
}

async fn client_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return ClientDetailsTemplate with client ID
    Html::from(String::new())
}

async fn client_new_handler(State(db): State<Database>) -> Html<String> {
    // Should return ClientNewTemplate
    Html::from(String::new())
}

async fn client_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return ClientEditTemplate with client data
    Html::from(String::new())
}

// Handler functions for HTMX endpoints

async fn client_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return ClientApiDetailsTemplate with client data and linked sites
    Html::from(String::new())
}

async fn client_update_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
    Form(form): Form<ClientUpdateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn client_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i64>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

async fn clients_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<ClientListFilter>,
) -> Html<String> {
    // Should return ClientListTemplate with filtered clients list
    Html::from(String::new())
}

async fn client_create_handler(
    State(db): State<Database>,
    Form(form): Form<ClientCreateForm>,
) -> Html<String> {
    // Should return NotificationTemplate with success/error message
    Html::from(String::new())
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/clients", get(clients_list_handler))
        .route("/clients/{id}", get(client_details_handler))
        .route("/clients/new", get(client_new_handler))
        .route("/clients/{id}/edit", get(client_edit_handler))
        // HTMX endpoints
        .route("/api/clients/{id}", get(client_api_details_handler))
        .route("/api/clients/{id}", put(client_update_handler))
        .route("/api/clients/{id}", delete(client_delete_handler))
        .route("/api/clients", get(clients_list_api_handler))
        .route("/api/clients", post(client_create_handler))
}