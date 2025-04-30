use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::database::Database;
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Types for page endpoints

#[derive(Template)]
#[template(path = "materials/list.html")]
pub struct MaterialsListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "materials/details.html")]
pub struct MaterialDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "materials/new.html")]
pub struct MaterialNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "materials/edit.html")]
pub struct MaterialEditTemplate {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub units: String,
}

// Types for HTMX endpoints

#[derive(Template, Serialize, Deserialize, sqlx::FromRow)]
#[template(path = "materials/api/details.html")]
pub struct MaterialApiDetailsTemplate {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub units: String,
    pub total_estimated: f64,
    pub total_actual: f64,
    pub total_cost: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialUpdateForm {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialCreateForm {
    pub name: String,
    pub cost: f64,
    pub units: String,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub name: Option<String>,
    pub cost_min: Option<f64>,
    pub cost_max: Option<f64>,
    pub excess_usage: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "materials/api/list.html")]
pub struct MaterialListTemplate {
    pub materials: Vec<MaterialListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct MaterialListItem {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub units: String,
    pub estimated_spendings: f64,
    pub actual_spendings: f64,
    pub excess: bool,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "materials/api/usage.html")]
pub struct MaterialUsageTemplate {
    pub id: i32,
    pub usage: Vec<UsageListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct UsageListItem {
    pub task_id: i32,
    pub task_name: String,
    pub site_id: i32,
    pub site_name: String,
    pub expected_amount: f64,
    pub actual_amount: f64,
    pub excess_amount: f64,
    pub total_cost: f64,
}

// Handler functions for page endpoints

async fn materials_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the materials list page template
    let template = MaterialsListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn material_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the material details page with the material ID
    let template = MaterialDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn material_new_handler(State(db): State<Database>) -> Html<String> {
    // Return the new material form template
    let template = MaterialNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct MaterialEditData {
    id: i32,
    name: String,
    cost: f64,
    units: String,
}

async fn material_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get material data from database
    let query = sqlx::query_as::<_, MaterialEditData>(
        "SELECT id, name, cost, units FROM material WHERE id = $1"
    )
    .bind(id);

    let material = match query.fetch_optional(&*db.pool).await {
        Ok(Some(material)) => material,
        Ok(None) => {
            return Html::from(format!("<p>Material with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching material: {}</p>", e));
        }
    };

    // Return the material edit template with data
    let template = MaterialEditTemplate {
        id: material.id,
        name: material.name,
        cost: material.cost,
        units: material.units,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct MaterialDetails {
    id: i32,
    name: String,
    cost: f64,
    units: String,
}

async fn material_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch the material details from database
    let query_result = sqlx::query_as::<_, MaterialDetails>(
        "SELECT id, name, cost, units FROM material WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let material = match query_result {
        Ok(Some(material)) => material,
        Ok(None) => {
            return Html::from(format!("<p>Material with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching material details: {}</p>", e));
        }
    };

    // Calculate usage statistics
    let usage_stats = sqlx::query_as::<_, MaterialApiDetailsTemplate>(
        "SELECT 
            COALESCE(SUM(e.expected_amount), 0) as total_estimated,
            COALESCE(SUM(e.actual_amount), 0) as total_actual
        FROM expenditure e
        WHERE e.material_id = $1",
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;

    let (total_estimated, total_actual, total_cost) = match usage_stats {
        Ok(stats) => {
            let estimated = stats.total_estimated;
            let actual = stats.total_actual;
            let cost = actual * material.cost;
            (estimated, actual, cost)
        },
        Err(e) => {
            return Html::from(format!("<p>Error fetching usage statistics: {}</p>", e));
        }
    };

    // Return material details with usage statistics
    let template = MaterialApiDetailsTemplate {
        id: material.id,
        name: material.name,
        cost: material.cost,
        units: material.units,
        total_estimated,
        total_actual,
        total_cost,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn material_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<MaterialUpdateForm>,
) -> Html<String> {
    // Update material in the database
    let result = sqlx::query(
        "UPDATE material SET name = $1, cost = $2, units = $3 
         WHERE id = $4"
    )
    .bind(&form.name)
    .bind(form.cost)
    .bind(&form.units)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (
            NotificationResult::Success, 
            Some(format!("Material '{}' updated successfully", form.name))
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to update material: {}", e))
        ),
    };

    let redirect = Some(format!("/materials/{}", id));
    
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

async fn material_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if material is used in expenditures, if yes, can't delete
    let has_expenditures = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM expenditure WHERE material_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_expenditures {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete material: it is used in {} expenditures", count)),
                redirect: Some("/materials".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Delete material from database
    let result = sqlx::query("DELETE FROM material WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Material deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete material: {}", e))),
    };

    let redirect = Some("/materials".to_string());
    
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
struct MaterialListRow {
    id: i32,
    name: String,
    cost: f64,
    units: String,
    estimated_spendings: Option<f64>,
    actual_spendings: Option<f64>,
}

async fn materials_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<MaterialListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT m.id, m.name, m.cost, m.units,
         (SELECT COALESCE(SUM(e.expected_amount), 0) FROM expenditure e WHERE e.material_id = m.id) as estimated_spendings,
         (SELECT COALESCE(SUM(e.actual_amount), 0) FROM expenditure e WHERE e.material_id = m.id) as actual_spendings
         FROM material m"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE m.name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(cost_min) = &filter.cost_min {
        if where_added {
            query_builder.push(" AND m.cost >= ");
        } else {
            query_builder.push(" WHERE m.cost >= ");
            where_added = true;
        }
        query_builder.push_bind(cost_min);
    }

    if let Some(cost_max) = &filter.cost_max {
        if where_added {
            query_builder.push(" AND m.cost <= ");
        } else {
            query_builder.push(" WHERE m.cost <= ");
            where_added = true;
        }
        query_builder.push_bind(cost_max);
    }

    if let Some(excess_usage) = &filter.excess_usage {
        if *excess_usage {
            let subquery = " EXISTS (SELECT 1 FROM expenditure e WHERE e.material_id = m.id AND e.actual_amount > e.expected_amount)";
            if where_added {
                query_builder.push(" AND ");
            } else {
                query_builder.push(" WHERE ");
                where_added = true;
            }
            query_builder.push(subquery);
        }
    }

    // Count total results for pagination
    let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query_builder.sql());
    let count = match sqlx::query_scalar::<_, i64>(&count_query).fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting materials: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("m.name"),
        "cost" => query_builder.push("m.cost"),
        "units" => query_builder.push("m.units"),
        "estimated_spendings" => query_builder.push("estimated_spendings"),
        "actual_spendings" => query_builder.push("actual_spendings"),
        _ => query_builder.push("m.id"),
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
    let query = query_builder.build_query_as::<MaterialListRow>();
    let materials = match query.fetch_all(&*db.pool).await {
        Ok(materials) => materials,
        Err(e) => return Html::from(format!("<p>Error fetching materials: {}</p>", e)),
    };

    // Convert to template items
    let material_items = materials.into_iter().map(|m| MaterialListItem {
        id: m.id,
        name: m.name,
        cost: m.cost,
        units: m.units,
        estimated_spendings: m.estimated_spendings.unwrap_or(0.0),
        actual_spendings: m.actual_spendings.unwrap_or(0.0),
        excess: m.actual_spendings.unwrap_or(0.0) > m.estimated_spendings.unwrap_or(0.0),
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = MaterialListTemplate {
        materials: material_items,
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

async fn material_create_handler(
    State(db): State<Database>,
    Form(form): Form<MaterialCreateForm>,
) -> Html<String> {
    // Insert new material into database
    let result = sqlx::query(
        "INSERT INTO material (name, cost, units) 
         VALUES ($1, $2, $3) 
         RETURNING id"
    )
    .bind(&form.name)
    .bind(form.cost)
    .bind(&form.units)
    .fetch_optional(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Material '{}' created successfully", form.name)),
                Some(format!("/materials/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create material: no ID returned".to_string()),
            Some("/materials".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create material: {}", e)),
            Some("/materials".to_string())
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
struct UsageRow {
    task_id: i32,
    task_name: String,
    site_id: i32,
    site_name: String,
    expected_amount: f64,
    actual_amount: Option<f64>,
    material_cost: f64,
}

async fn material_usage_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query usage history for this material
    let query = sqlx::query_as::<_, UsageRow>(
        "SELECT t.id as task_id, t.name as task_name, 
         s.id as site_id, s.name as site_name,
         e.expected_amount, e.actual_amount, m.cost as material_cost
         FROM expenditure e
         JOIN task t ON e.task_id = t.id
         JOIN site s ON t.site_id = s.id
         JOIN material m ON e.material_id = m.id
         WHERE e.material_id = $1
         ORDER BY COALESCE(e.actual_amount / e.expected_amount, 0) DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i32)
    .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    let usage_rows = match query.fetch_all(&*db.pool).await {
        Ok(usage) => usage,
        Err(e) => return Html::from(format!("<p>Error fetching material usage: {}</p>", e)),
    };

    // Convert to template items
    let usage_items = usage_rows.into_iter().map(|u| {
        let actual = u.actual_amount.unwrap_or(0.0);
        let excess = if actual > u.expected_amount { actual - u.expected_amount } else { 0.0 };
        UsageListItem {
            task_id: u.task_id,
            task_name: u.task_name,
            site_id: u.site_id,
            site_name: u.site_name,
            expected_amount: u.expected_amount,
            actual_amount: actual,
            excess_amount: excess,
            total_cost: actual * u.material_cost,
        }
    }).collect();

    let template = MaterialUsageTemplate { 
        id,
        usage: usage_items,
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
        .route("/materials", get(materials_list_handler))
        .route("/materials/{id}", get(material_details_handler))
        .route("/materials/new", get(material_new_handler))
        .route("/materials/{id}/edit", get(material_edit_handler))
        // HTMX endpoints
        .route("/api/materials/{id}", get(material_api_details_handler))
        .route("/api/materials/{id}", put(material_update_handler))
        .route("/api/materials/{id}", delete(material_delete_handler))
        .route("/api/materials", get(materials_list_api_handler))
        .route("/api/materials", post(material_create_handler))
        .route("/api/materials/{id}/usage", get(material_usage_handler))
}