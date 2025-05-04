use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{SiteType, NotificationResult, NotificationTemplate}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo};

// Types for page endpoints

#[derive(Template)]
#[template(path = "clients/list.html")]
pub struct ClientsListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "clients/details.html")]
pub struct ClientDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "clients/new.html")]
pub struct ClientNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "clients/edit.html")]
pub struct ClientEditTemplate {
    pub id: i32,
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool
}

// Types for HTMX endpoints

#[derive(Template, Serialize, Deserialize)]
#[template(path = "clients/api/details.html")]
pub struct ClientApiDetailsTemplate {
    pub id: i32,
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
    pub sites: Vec<ClientSite>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientSite {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientUpdateForm {
    pub name: String,
    pub inn: String,
    pub address: String,
    pub contact_person_email: String,
    pub contact_person_name: String,
    pub is_vip: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ClientCreateForm {
    pub name: String,
    pub inn: String,
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
    pub inn: Option<String>,
    pub is_vip: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "clients/api/list.html")]
pub struct ClientListTemplate {
    pub clients: Vec<ClientListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ClientListItem {
    pub id: i32,
    pub name: String,
    pub inn: String,
    pub is_vip: bool,
}

// Handler functions for page endpoints

async fn clients_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the clients list page template
    let template = ClientsListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn client_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the client details page with the client ID
    let template = ClientDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn client_new_handler(State(db): State<Database>) -> Html<String> {
    // Return the new client form template
    let template = ClientNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct ClientEditData {
    id: i32,
    name: String,
    inn: String,
    address: String,
    contact_person_email: String,
    contact_person_name: String,
    is_vip: bool,
}

async fn client_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get client data from database
    let query = sqlx::query_as::<_, ClientEditData>(
        "SELECT id, name, inn, address, contact_person_email, contact_person_name, is_vip 
         FROM client
         WHERE id = $1"
    )
    .bind(id);

    let client = match query.fetch_optional(&*db.pool).await {
        Ok(Some(client)) => client,
        Ok(None) => {
            return Html::from(format!("<p>Client with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching client: {}</p>", e));
        }
    };

    // Render the template with client data
    let template = ClientEditTemplate {
        id: client.id,
        name: client.name,
        inn: client.inn,
        address: client.address,
        contact_person_email: client.contact_person_email,
        contact_person_name: client.contact_person_name,
        is_vip: client.is_vip,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct ClientDetails {
    id: i32,
    name: String,
    inn: String,
    address: String,
    contact_person_email: String,
    contact_person_name: String,
    is_vip: bool,
}

#[derive(FromRow)]
struct ClientSiteRow {
    id: i32,
    name: String,
    #[sqlx(rename = "type")]
    type_: SiteType,
    status: String,
}

async fn client_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch the client details from database
    let query_result = sqlx::query_as::<_, ClientDetails>(
        "SELECT id, name, inn, address, contact_person_email, contact_person_name, is_vip
         FROM client
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let client = match query_result {
        Ok(Some(client)) => client,
        Ok(None) => {
            return Html::from(format!("<p>Client with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching client details: {}</p>", e));
        }
    };

    // Fetch the client's sites
    let sites_query = sqlx::query_as::<_, ClientSiteRow>(
        "SELECT id, name, type,
         CASE
            WHEN EXISTS (SELECT 1 FROM task WHERE site_id = site.id AND actual_period_end IS NULL) THEN 'In Progress'
            WHEN NOT EXISTS (SELECT 1 FROM task WHERE site_id = site.id) THEN 'Planned'
            ELSE 'Completed'
         END as status
         FROM site
         WHERE client_id = $1
         ORDER BY name"
    )
    .bind(id)
    .fetch_all(&*db.pool)
    .await;

    let sites = match sites_query {
        Ok(sites) => sites,
        Err(e) => {
            return Html::from(format!("<p>Error fetching client sites: {}</p>", e));
        }
    };

    // Convert to template sites
    let site_items: Vec<ClientSite> = sites.into_iter().map(|s| ClientSite {
        id: s.id,
        name: s.name,
        type_: s.type_,
        status: s.status,
    }).collect();

    // Return client details with sites
    let template = ClientApiDetailsTemplate {
        id: client.id,
        name: client.name,
        inn: client.inn,
        address: client.address,
        contact_person_email: client.contact_person_email,
        contact_person_name: client.contact_person_name,
        is_vip: client.is_vip,
        sites: site_items,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn client_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<ClientUpdateForm>,
) -> Html<String> {
    // Check if INN is unique (if changed)
    let current_inn = sqlx::query_scalar::<_, String>("SELECT inn FROM client WHERE id = $1")
        .bind(id)
        .fetch_optional(&*db.pool)
        .await;
    
    if let Ok(Some(inn)) = current_inn {
        if inn != form.inn {
            let inn_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM client WHERE inn = $1 AND id != $2)")
                .bind(&form.inn)
                .bind(id)
                .fetch_one(&*db.pool)
                .await;
            
            if let Ok(true) = inn_exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Client with INN {} already exists", form.inn)),
                    redirect: Some(format!("/clients/{}/edit", id)),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Update client in the database
    let result = sqlx::query(
        "UPDATE client SET name = $1, inn = $2, address = $3, 
         contact_person_email = $4, contact_person_name = $5, is_vip = $6 
         WHERE id = $7"
    )
    .bind(&form.name)
    .bind(&form.inn)
    .bind(&form.address)
    .bind(&form.contact_person_email)
    .bind(&form.contact_person_name)
    .bind(form.is_vip)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some(format!("Client '{}' updated successfully", form.name))),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update client: {}", e))),
    };

    let redirect = Some(format!("/clients/{}", id));
    
    let template = NotificationTemplate {
        result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn client_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if client has sites, if yes, can't delete
    let has_sites = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM site WHERE client_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_sites {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete client: client has {} sites. Remove sites first.", count)),
                redirect: Some("/clients".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Delete client from database
    let result = sqlx::query("DELETE FROM client WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Client deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete client: {}", e))),
    };

    let redirect = Some("/clients".to_string());
    
    let template = NotificationTemplate {
        result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct ClientListRow {
    id: i32,
    name: String,
    inn: String,
    is_vip: bool,
}

async fn clients_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<ClientListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, inn, is_vip FROM client"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(inn) = &filter.inn {
        if where_added {
            query_builder.push(" AND inn = ");
        } else {
            query_builder.push(" WHERE inn = ");
            where_added = true;
        }
        query_builder.push_bind(inn);
    }

    if let Some(is_vip) = &filter.is_vip {
        if where_added {
            query_builder.push(" AND is_vip = ");
        } else {
            query_builder.push(" WHERE is_vip = ");
            where_added = true;
        }
        query_builder.push_bind(is_vip);
    }

    // Count total results for pagination
    let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query_builder.sql());
    let count = match sqlx::query_scalar::<_, i64>(&count_query).fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting clients: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("name"),
        "inn" => query_builder.push("inn"),
        _ => query_builder.push("id"),
    };

    // Add sort direction
    match filter.sort.sort_direction {
        SortDirection::Ascending => {
            query_builder.push(" ASC");
        },
        SortDirection::Descending => {
            query_builder.push(" DESC");
        },
    }

    // Add pagination
    query_builder.push(" LIMIT ");
    query_builder.push_bind(pagination.page_size as i32);
    query_builder.push(" OFFSET ");
    query_builder.push_bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    // Execute query
    let query = query_builder.build_query_as::<ClientListRow>();
    let clients = match query.fetch_all(&*db.pool).await {
        Ok(clients) => clients,
        Err(e) => return Html::from(format!("<p>Error fetching clients: {}</p>", e)),
    };

    // Convert to template items
    let client_items = clients.into_iter().map(|c| ClientListItem {
        id: c.id,
        name: c.name,
        inn: c.inn,
        is_vip: c.is_vip,
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = ClientListTemplate {
        clients: client_items,
        pagination,
        query_info: QueryInfo {
            num_pages,
            num_items: count as u32,
        },
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn client_create_handler(
    State(db): State<Database>,
    Form(form): Form<ClientCreateForm>,
) -> Html<String> {
    // Check if INN is unique
    let inn_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM client WHERE inn = $1)")
        .bind(&form.inn)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(true) = inn_exists {
        let template = NotificationTemplate {
            result: NotificationResult::Error,
            message: Some(format!("Client with INN {} already exists", form.inn)),
            redirect: Some("/clients/new".to_string()),
        };
        return match template.render() {
            Ok(html) => Html::from(html),
            Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
        };
    }

    // Insert new client into database
    let result = sqlx::query(
        "INSERT INTO client (name, inn, address, contact_person_email, contact_person_name, is_vip) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id"
    )
    .bind(&form.name)
    .bind(&form.inn)
    .bind(&form.address)
    .bind(&form.contact_person_email)
    .bind(&form.contact_person_name)
    .bind(form.is_vip)
    .fetch_optional(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Client '{}' created successfully", form.name)),
                Some(format!("/clients/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create client: no ID returned".to_string()),
            Some("/clients".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create client: {}", e)),
            Some("/clients".to_string())
        ),
    };
    
    let template = NotificationTemplate {
        result: notification_result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
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