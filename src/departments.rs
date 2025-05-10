use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
    Json,
};
use askama::Template;
use sqlx::FromRow;
use sqlx::Row;

use crate::{database::Database, general::{Qualification, SiteType, NotificationResult}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationTemplate};
use crate::utils::empty_string_as_none;

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
    pub id: i32,
}

#[derive(Template)]
#[template(path = "departments/new.html")]
pub struct DepartmentNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/edit.html")]
pub struct DepartmentEditTemplate {
    pub id: i32,
    pub name: String,
    pub supervisor_id: Option<i32>,
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
    pub id: i32,
    pub name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>,
    pub tab: DepartmentTab,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentUpdateForm {
    pub name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentCreateForm {
    pub name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct DepartmentListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub supervisor_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
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
    pub id: i32,
    pub name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/areas.html")]
pub struct DepartmentAreasTemplate {
    pub areas: Vec<AreaListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct AreaListItem {
    pub id: i32,
    pub name: String,
    pub supervisor_id: Option<i32>,
    pub supervisor_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/equipment.html")]
pub struct DepartmentEquipmentTemplate {
    pub id: i32,
    pub equipment: Vec<EquipmentListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListItem {
    pub id: i32,
    pub name: String,
    pub amount: u32,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/sites.html")]
pub struct DepartmentSitesTemplate {
    pub id: i32,
    pub sites: Vec<SiteListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListItem {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: SiteType, 
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "departments/api/personnel.html")]
pub struct DepartmentPersonnelTemplate {
    pub id: i32,
    pub personnel: Vec<PersonnelListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PersonnelListItem {
    pub id: i32,
    pub name: String,
    pub qualification: Qualification,
}

// Handler functions for page endpoints

async fn departments_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the departments list page template
    let template = DepartmentsListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

async fn department_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Render the department details page with the department ID
    let template = DepartmentDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

async fn department_new_handler(State(db): State<Database>) -> Html<String> {
    // Render the new department form template
    let template = DepartmentNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentEditData {
    id: i32,
    name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn department_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get department data from database
    let query = sqlx::query_as::<_, DepartmentEditData>(
        "SELECT d.id, d.name, d.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM department d
         LEFT JOIN technical_personnel tp ON d.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id
         WHERE d.id = $1"
    )
    .bind(id);

    let dept = match query.fetch_optional(&*db.pool).await {
        Ok(Some(dept)) => dept,
        Ok(None) => {
            return Html::from("<p>Department not found</p>".to_string());
        }
        Err(_) => {
            return Html::from("<p>Error fetching department</p>".to_string());
        }
    };

    let template = DepartmentEditTemplate {
        id: dept.id,
        name: dept.name,
        supervisor_id: dept.supervisor_id,
        supervisor_name: dept.supervisor_name,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct DepartmentDetails {
    id: i32,
    name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn department_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(query): Query<DepartmentTabQuery>,
) -> Html<String> {
    // Fetch the department details from database
    let query_result = sqlx::query_as::<_, DepartmentDetails>(
        "SELECT d.id, d.name, d.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM department d
         LEFT JOIN technical_personnel tp ON d.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id
         WHERE d.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let department = match query_result {
        Ok(Some(dept)) => dept,
        Ok(None) => {
            return Html::from("<p>Department not found</p>".to_string());
        }
        Err(_) => {
            return Html::from("<p>Error fetching department details</p>".to_string());
        }
    };

    let template = DepartmentApiDetailsTemplate {
        id: department.id,
        name: department.name,
        supervisor_id: department.supervisor_id,
        supervisor_name: department.supervisor_name,
        tab: query.tab,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

async fn department_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<DepartmentUpdateForm>,
) -> Html<String> {
    // Update department in the database
    let result = sqlx::query(
        "UPDATE department SET name = $1, supervisor_id = $2 WHERE id = $3"
    )
    .bind(&form.name)
    .bind(form.supervisor_id)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some(format!("Department '{}' updated successfully", form.name))),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update department: {}", e))),
    };

    let redirect = Some(format!("/departments/{}", id));
    
    let template = NotificationTemplate {
        result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

async fn department_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if department has areas, if yes, can't delete
    let has_areas = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM area WHERE department_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_areas {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete department: it has {} areas. Remove areas first.", count)),
                redirect: Some("/departments".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
            };
        }
    }

    // Delete department from database
    let result = sqlx::query("DELETE FROM department WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Department deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete department: {}", e))),
    };

    let redirect = Some("/departments".to_string());
    
    let template = NotificationTemplate {
        result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentListRow {
    id: i32,
    name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn departments_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<DepartmentListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT d.id, d.name, d.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM department d
         LEFT JOIN technical_personnel tp ON d.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE d.name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(supervisor_id) = &filter.supervisor_id {
        if where_added {
            query_builder.push(" AND d.supervisor_id = ");
        } else {
            query_builder.push(" WHERE d.supervisor_id = ");
            where_added = true;
        }
        query_builder.push_bind(supervisor_id);
    }

    // Count total results for pagination - REPLACE THIS SECTION
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM department d
         LEFT JOIN technical_personnel tp ON d.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id"
    );

    let mut count_where_added = false;

    // Add the same filter conditions to the count query
    if let Some(name) = &filter.name {
        count_query_builder.push(" WHERE d.name ILIKE ");
        count_query_builder.push_bind(format!("%{}%", name));
        count_where_added = true;
    }

    if let Some(supervisor_id) = &filter.supervisor_id {
        if count_where_added {
            count_query_builder.push(" AND d.supervisor_id = ");
        } else {
            count_query_builder.push(" WHERE d.supervisor_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(supervisor_id);
    }

    let count = match count_query_builder.build_query_scalar::<i64>().fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting departments: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("d.name"),
        "supervisor" => query_builder.push("supervisor_name"),
        _ => query_builder.push("d.id"),
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
    query_builder.push_bind(pagination.page_size as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind((pagination.page_number as i64 - 1) * pagination.page_size as i64);

    // Execute query
    let query = query_builder.build_query_as::<DepartmentListRow>();
    let departments = match query.fetch_all(&*db.pool).await {
        Ok(deps) => deps,
        Err(e) => return Html::from(format!("<p>Error fetching departments: {e}</p>")),
    };

    // Convert to template items
    let department_items = departments.into_iter().map(|d| DepartmentListItem {
        id: d.id,
        name: d.name,
        supervisor_id: d.supervisor_id,
        supervisor_name: d.supervisor_name,
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = DepartmentListTemplate {
        departments: department_items,
        pagination,
        query_info: QueryInfo {
            num_pages,
            num_items: count as u32,
        },
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

async fn department_create_handler(
    State(db): State<Database>,
    Form(form): Form<DepartmentCreateForm>,
) -> Html<String> {
    // Insert new department into database
    let result = sqlx::query("INSERT INTO department (name, supervisor_id) VALUES ($1, $2) RETURNING id")
        .bind(&form.name)
        .bind(form.supervisor_id)
        .fetch_optional(&*db.pool)
        .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Department '{}' created successfully", form.name)),
                Some(format!("/departments/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create department: no ID returned".to_string()),
            Some("/departments".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create department: {}", e)),
            Some("/departments".to_string())
        ),
    };
    
    let template = NotificationTemplate {
        result: notification_result,
        message,
        redirect,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentAreaRow {
    id: i32,
    name: String,
    supervisor_id: Option<i32>,
    supervisor_name: Option<String>,
}

async fn department_areas_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query areas for this department
    let query = sqlx::query_as::<_, DepartmentAreaRow>(
        "SELECT a.id, a.name, a.supervisor_id, 
         CASE WHEN tp.id IS NOT NULL THEN CONCAT(e.last_name, ' ', e.first_name) ELSE NULL END as supervisor_name
         FROM area a
         LEFT JOIN technical_personnel tp ON a.supervisor_id = tp.id
         LEFT JOIN employee e ON tp.id = e.id
         WHERE a.department_id = $1
         ORDER BY a.name
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i64)
    .bind((pagination.page_number as i64 - 1) * pagination.page_size as i64);

    let areas = match query.fetch_all(&*db.pool).await {
        Ok(areas) => areas,
        Err(_) => vec![],
    };

    // Convert to template items
    let area_items = areas.into_iter().map(|a| AreaListItem {
        id: a.id,
        name: a.name,
        supervisor_id: a.supervisor_id,
        supervisor_name: a.supervisor_name,
    }).collect();

    let template = DepartmentAreasTemplate { areas: area_items };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentEquipmentRow {
    id: i32,
    name: String,
    amount: i32,
}

async fn department_equipment_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Query equipment for this department
    let query = sqlx::query_as::<_, DepartmentEquipmentRow>(
        "SELECT e.id, e.name, COALESCE(ea.amount, 0) as amount
         FROM equipment e
         LEFT JOIN equipment_allocation ea ON e.id = ea.equipment_id AND ea.department_id = $1 
                                        AND ea.site_id IS NULL
                                        AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE)
         WHERE ea.department_id = $1
         ORDER BY e.name"
    )
    .bind(id);

    let equipment = match query.fetch_all(&*db.pool).await {
        Ok(equip) => equip,
        Err(_) => vec![],
    };

    // Convert to template items
    let equipment_items = equipment.into_iter().map(|e| EquipmentListItem {
        id: e.id,
        name: e.name,
        amount: e.amount as u32,
    }).collect();

    let template = DepartmentEquipmentTemplate { 
        id,
        equipment: equipment_items,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentSiteRow {
    id: i32,
    name: String,
    #[sqlx(rename = "type")]
    type_: SiteType,
}

async fn department_sites_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Query sites for this department
    let query = sqlx::query_as::<_, DepartmentSiteRow>(
        "SELECT s.id, s.name, s.type
         FROM site s
         JOIN area a ON s.area_id = a.id
         WHERE a.department_id = $1
         ORDER BY s.name"
    )
    .bind(id);

    let sites = match query.fetch_all(&*db.pool).await {
        Ok(sites) => sites,
        Err(_) => vec![],
    };

    // Convert to template items
    let site_items = sites.into_iter().map(|s| SiteListItem {
        id: s.id,
        name: s.name,
        type_: s.type_,
    }).collect();

    let template = DepartmentSitesTemplate { 
        id,
        sites: site_items,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
}

#[derive(FromRow)]
struct DepartmentPersonnelRow {
    id: i32,
    name: String,
    qualification: Qualification,
}

async fn department_personnel_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Query technical personnel for this department
    let query = sqlx::query_as::<_, DepartmentPersonnelRow>(
        "SELECT tp.id, CONCAT(e.last_name, ' ', e.first_name) as name, tp.qualification
         FROM technical_personnel tp
         JOIN employee e ON tp.id = e.id
         WHERE tp.id = (SELECT supervisor_id FROM department WHERE id = $1)
         OR tp.id IN (SELECT supervisor_id FROM area WHERE department_id = $1)
         ORDER BY e.last_name, e.first_name"
    )
    .bind(id);

    let personnel = match query.fetch_all(&*db.pool).await {
        Ok(personnel) => personnel,
        Err(_) => vec![],
    };

    // Convert to template items
    let personnel_items = personnel.into_iter().map(|p| PersonnelListItem {
        id: p.id,
        name: p.name,
        qualification: p.qualification,
    }).collect();

    let template = DepartmentPersonnelTemplate { 
        id,
        personnel: personnel_items,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from("<p>Error rendering template</p>".to_string()),
    }
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