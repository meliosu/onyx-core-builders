use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Qualification, SiteType, Position}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};
use crate::utils::empty_string_as_none;

// Tab selector for area details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AreaTab {
    Sites,
    Personnel,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "areas/list.html")]
pub struct AreasListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/details.html")]
pub struct AreaDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "areas/new.html")]
pub struct AreaNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/edit.html")]
pub struct AreaEditTemplate {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
    pub department_name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct AreaTabQuery {
    pub tab: AreaTab,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/api/details.html")]
pub struct AreaApiDetailsTemplate {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
    pub department_name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>,
    pub tab: AreaTab,
}

#[derive(Serialize, Deserialize)]
pub struct AreaUpdateForm {
    pub name: String,
    pub department_id: i32,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaCreateForm {
    pub name: String,
    pub department_id: i32,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub department_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/api/list.html")]
pub struct AreaListTemplate {
    pub areas: Vec<AreaListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListItem {
    pub id: i32,
    pub name: String,
    pub department_id: i32,
    pub department_name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/api/sites.html")]
pub struct AreaSitesTemplate {
    pub id: i32,
    pub sites: Vec<SiteListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListItem {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub client_id: i32,
    pub client_name: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "areas/api/personnel.html")]
pub struct AreaPersonnelTemplate {
    pub id: i32,
    pub personnel: Vec<PersonnelListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct PersonnelListItem {
    pub id: i32,
    pub name: String,
    pub qualification: Qualification,
    pub position: Option<Position>,
}

// Handler functions for page endpoints

async fn areas_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the areas list page template
    let template = AreasListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn area_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the area details page with the area ID
    let template = AreaDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn area_new_handler(State(db): State<Database>) -> Html<String> {
    // Return the new area form template
    let template = AreaNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct AreaEditData {
    id: i32,
    name: String,
    department_id: i32,
    department_name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn area_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get area data from database
    let query = sqlx::query_as::<_, AreaEditData>(
        "SELECT a.id, a.name, a.department_id, d.name as department_name, 
         a.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM area a
         JOIN department d ON a.department_id = d.id
         LEFT JOIN technical_personnel tp ON a.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id
         WHERE a.id = $1"
    )
    .bind(id);

    let area = match query.fetch_optional(&*db.pool).await {
        Ok(Some(area)) => area,
        Ok(None) => {
            return Html::from(format!("<p>Area with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching area: {}</p>", e));
        }
    };

    // Render the template with area data
    let template = AreaEditTemplate {
        id: area.id,
        name: area.name,
        department_id: area.department_id,
        department_name: area.department_name,
        supervisor_id: area.supervisor_id,
        supervisor_name: area.supervisor_name,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct AreaDetails {
    id: i32,
    name: String,
    department_id: i32,
    department_name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn area_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(query): Query<AreaTabQuery>,
) -> Html<String> {
    // Fetch the area details from database
    let query_result = sqlx::query_as::<_, AreaDetails>(
        "SELECT a.id, a.name, a.department_id, d.name as department_name, 
         a.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM area a
         JOIN department d ON a.department_id = d.id
         LEFT JOIN technical_personnel tp ON a.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id
         WHERE a.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let area = match query_result {
        Ok(Some(area)) => area,
        Ok(None) => {
            return Html::from(format!("<p>Area with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching area details: {}</p>", e));
        }
    };

    // Return area details with selected tab
    let template = AreaApiDetailsTemplate {
        id: area.id,
        name: area.name,
        department_id: area.department_id,
        department_name: area.department_name,
        supervisor_id: area.supervisor_id,
        supervisor_name: area.supervisor_name,
        tab: query.tab,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn area_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<AreaUpdateForm>,
) -> Html<String> {
    // Check if department exists
    let department_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM department WHERE id = $1)")
        .bind(form.department_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = department_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Department with ID {} does not exist", form.department_id)),
                redirect: Some(format!("/areas/{}/edit", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }
    
    // Check if supervisor exists if provided
    if let Some(supervisor_id) = form.supervisor_id {
        let supervisor_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM technical_personnel WHERE id = $1)")
            .bind(supervisor_id)
            .fetch_one(&*db.pool)
            .await;
        
        if let Ok(exists) = supervisor_exists {
            if !exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Technical personnel with ID {} does not exist", supervisor_id)),
                    redirect: Some(format!("/areas/{}/edit", id)),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Update area in the database
    let result = sqlx::query(
        "UPDATE area SET name = $1, department_id = $2, supervisor_id = $3 WHERE id = $4"
    )
    .bind(&form.name)
    .bind(form.department_id)
    .bind(form.supervisor_id)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some(format!("Area '{}' updated successfully", form.name))),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update area: {}", e))),
    };

    let redirect = Some(format!("/areas/{}", id));
    
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

async fn area_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if area has sites, if yes, can't delete
    let has_sites = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM site WHERE area_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_sites {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete area: it has {} sites. Remove sites first.", count)),
                redirect: Some("/areas".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Delete area from database
    let result = sqlx::query("DELETE FROM area WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Area deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete area: {}", e))),
    };

    let redirect = Some("/areas".to_string());
    
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
struct AreaListRow {
    id: i32,
    name: String,
    department_id: i32,
    department_name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn areas_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<AreaListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT a.id, a.name, a.department_id, d.name as department_name, 
         a.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM area a
         JOIN department d ON a.department_id = d.id
         LEFT JOIN technical_personnel tp ON a.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE a.name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(department_id) = &filter.department_id {
        if where_added {
            query_builder.push(" AND a.department_id = ");
        } else {
            query_builder.push(" WHERE a.department_id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
    }

    if let Some(supervisor_id) = &filter.supervisor_id {
        if where_added {
            query_builder.push(" AND a.supervisor_id = ");
        } else {
            query_builder.push(" WHERE a.supervisor_id = ");
            where_added = true;
        }
        query_builder.push_bind(supervisor_id);
    }

    let count_query = format!("SELECT COUNT(*) FROM ({})", query_builder.sql());
    let count_query = {
        let mut query = sqlx::query_scalar::<_, i64>(&count_query);

        if let Some(name) = &filter.name {
            query = query.bind(name);
        }

        if let Some(department_id) = &filter.department_id {
            query = query.bind(department_id);
        }

        if let Some(supervisor_id) = &filter.supervisor_id {
            query = query.bind(supervisor_id);
        }

        query
    };

    let count = match count_query.fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting areas: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("a.name"),
        "department" => query_builder.push("department_name"),
        "supervisor" => query_builder.push("supervisor_name"),
        _ => query_builder.push("a.id"),
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
    let query = query_builder.build_query_as::<AreaListRow>();
    let areas = match query.fetch_all(&*db.pool).await {
        Ok(areas) => areas,
        Err(e) => return Html::from(format!("<p>Error fetching areas: {}</p>", e)),
    };

    // Convert to template items
    let area_items = areas.into_iter().map(|a| AreaListItem {
        id: a.id,
        name: a.name,
        department_id: a.department_id,
        department_name: a.department_name,
        supervisor_id: a.supervisor_id,
        supervisor_name: a.supervisor_name,
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = AreaListTemplate {
        areas: area_items,
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

async fn area_create_handler(
    State(db): State<Database>,
    Form(form): Form<AreaCreateForm>,
) -> Html<String> {
    // Check if department exists
    let department_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM department WHERE id = $1)")
        .bind(form.department_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = department_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Department with ID {} does not exist", form.department_id)),
                redirect: Some("/areas/new".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }
    
    // Check if supervisor exists if provided
    if let Some(supervisor_id) = form.supervisor_id {
        let supervisor_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM technical_personnel WHERE id = $1)")
            .bind(supervisor_id)
            .fetch_one(&*db.pool)
            .await;
        
        if let Ok(exists) = supervisor_exists {
            if !exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Technical personnel with ID {} does not exist", supervisor_id)),
                    redirect: Some("/areas/new".to_string()),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Insert new area into database
    let result = sqlx::query("INSERT INTO area (name, department_id, supervisor_id) VALUES ($1, $2, $3) RETURNING id")
        .bind(&form.name)
        .bind(form.department_id)
        .bind(form.supervisor_id)
        .fetch_optional(&*db.pool)
        .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Area '{}' created successfully", form.name)),
                Some(format!("/areas/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create area: no ID returned".to_string()),
            Some("/areas".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create area: {}", e)),
            Some("/areas".to_string())
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

#[derive(FromRow)]
struct AreaSiteRow {
    id: i32,
    name: String,
    #[sqlx(rename = "type")]
    type_: SiteType,
    client_id: i32,
    client_name: String,
}

async fn area_sites_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query sites for this area
    let query = sqlx::query_as::<_, AreaSiteRow>(
        "SELECT s.id, s.name, s.type, s.client_id, c.name as client_name
         FROM site s
         JOIN client c ON s.client_id = c.id
         WHERE s.area_id = $1
         ORDER BY s.name
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i32)
    .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    let sites = match query.fetch_all(&*db.pool).await {
        Ok(sites) => sites,
        Err(e) => return Html::from(format!("<p>Error fetching sites: {}</p>", e)),
    };

    // Convert to template items
    let site_items = sites.into_iter().map(|s| SiteListItem {
        id: s.id,
        name: s.name,
        type_: s.type_,
        client_id: s.client_id,
        client_name: s.client_name,
    }).collect();

    let template = AreaSitesTemplate { 
        id,
        sites: site_items,
        pagination,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct AreaPersonnelRow {
    id: i32,
    name: String,
    qualification: Qualification,
    position: Option<Position>,
}

async fn area_personnel_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query technical personnel for this area
    let query = sqlx::query_as::<_, AreaPersonnelRow>(
        "SELECT tp.id, CONCAT(e.last_name, ' ', e.first_name) as name, tp.qualification, tp.position
         FROM technical_personnel tp
         JOIN employee e ON tp.id = e.id
         WHERE tp.id = (SELECT supervisor_id FROM area WHERE id = $1)
         ORDER BY name
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i32)
    .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    let personnel = match query.fetch_all(&*db.pool).await {
        Ok(personnel) => personnel,
        Err(e) => return Html::from(format!("<p>Error fetching personnel: {}</p>", e)),
    };

    // Convert to template items
    let personnel_items = personnel.into_iter().map(|p| PersonnelListItem {
        id: p.id,
        name: p.name,
        qualification: p.qualification,
        position: p.position,
    }).collect();

    let template = AreaPersonnelTemplate { 
        id,
        personnel: personnel_items,
        pagination,
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