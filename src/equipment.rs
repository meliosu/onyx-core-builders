use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate, FuelType}};

// Types for page endpoints

#[derive(Template)]
#[template(path = "equipment/list.html")]
pub struct EquipmentPageTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "equipment/details.html")]
pub struct EquipmentDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "equipment/new.html")]
pub struct EquipmentNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "equipment/edit.html")]
pub struct EquipmentEditTemplate {
    pub id: i32,
    pub name: String,
    pub amount: u32,
    pub purchase_date: NaiveDateTime,
    pub purchase_cost: f64,
    pub fuel_type: Option<FuelType>,
}

// Types for HTMX endpoints

#[derive(Template, Serialize, Deserialize)]
#[template(path = "equipment/api/details.html")]
pub struct EquipmentApiDetailsTemplate {
    pub id: i32,
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
    pub department_id: Option<i32>,
    pub site_id: Option<i32>,
    pub name: Option<String>,
    pub available: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "equipment/api/list.html")]
pub struct EquipmentListTemplate {
    pub equipment: Vec<EquipmentListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentListItem {
    pub id: i32,
    pub name: String,
    pub total_amount: u32,
    pub available_amount: u32,
    pub purchase_date: String,
    pub purchase_cost: f64,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "equipment/api/allocations.html")]
pub struct EquipmentAllocationsTemplate {
    pub allocations: Vec<AllocationListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct AllocationListItem {
    pub department_id: i32,
    pub department_name: String,
    pub site_id: Option<i32>,
    pub site_name: Option<String>,
    pub amount: u32,
    pub period_start: NaiveDateTime,
    pub period_end: Option<NaiveDateTime>,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentAllocationForm {
    pub department_id: i32,
    pub site_id: Option<i32>,
    pub amount: u32,
    pub period_start: NaiveDateTime,
    pub period_end: Option<NaiveDateTime>,
}

// Handler functions for page endpoints

async fn equipment_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the equipment list page template
    let template = EquipmentPageTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn equipment_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the equipment details page with the equipment ID
    let template = EquipmentDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn equipment_new_handler(State(db): State<Database>) -> Html<String> {
    // Return the new equipment form template
    let template = EquipmentNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct EquipmentEditData {
    id: i32,
    name: String,
    amount: i32,
    purchase_date: NaiveDateTime,
    purchase_cost: f64,
    fuel_type: Option<FuelType>,
}

async fn equipment_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get equipment data from database
    let query = sqlx::query_as::<_, EquipmentEditData>(
        "SELECT id, name, amount, purchase_date, purchase_cost, fuel_type 
         FROM equipment
         WHERE id = $1"
    )
    .bind(id);

    let equipment = match query.fetch_optional(&*db.pool).await {
        Ok(Some(equipment)) => equipment,
        Ok(None) => {
            return Html::from(format!("<p>Equipment with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching equipment: {}</p>", e));
        }
    };

    // Render the template with equipment data
    let template = EquipmentEditTemplate {
        id: equipment.id,
        name: equipment.name,
        amount: equipment.amount as u32,
        purchase_date: equipment.purchase_date,
        purchase_cost: equipment.purchase_cost,
        fuel_type: equipment.fuel_type,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct EquipmentDetails {
    id: i32,
    name: String,
    amount: i32,
    available_amount: i32,
    purchase_date: NaiveDateTime,
    purchase_cost: f64,
    fuel_type: Option<FuelType>,
}

async fn equipment_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch the equipment details from database
    let query = sqlx::query_as::<_, EquipmentDetails>(
        "SELECT e.id, e.name, e.amount, 
         (e.amount - COALESCE((SELECT SUM(ea.amount) FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE)), 0)) as available_amount,
         e.purchase_date, e.purchase_cost, e.fuel_type
         FROM equipment e
         WHERE e.id = $1"
    )
    .bind(id);

    let equipment = match query.fetch_optional(&*db.pool).await {
        Ok(Some(equipment)) => equipment,
        Ok(None) => {
            return Html::from(format!("<p>Equipment with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching equipment details: {}</p>", e));
        }
    };

    // Render the template with equipment data
    let template = EquipmentApiDetailsTemplate {
        id: equipment.id,
        name: equipment.name,
        amount: equipment.amount as u32,
        available_amount: equipment.available_amount as u32,
        purchase_date: equipment.purchase_date,
        purchase_cost: equipment.purchase_cost,
        fuel_type: equipment.fuel_type,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn equipment_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<EquipmentUpdateForm>,
) -> Html<String> {
    // Check if the amount is valid (not less than allocated)
    let allocated_amount = sqlx::query_scalar::<_, i32>(
        "SELECT COALESCE(SUM(amount), 0) FROM equipment_allocation 
         WHERE equipment_id = $1 AND (period_end IS NULL OR period_end > CURRENT_DATE)"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;

    if let Ok(amount) = allocated_amount {
        if form.amount < amount as u32 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot set amount less than allocated amount ({})", amount)),
                redirect: Some(format!("/equipment/{}/edit", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Update equipment in the database
    let result = sqlx::query(
        "UPDATE equipment SET name = $1, amount = $2, purchase_date = $3, purchase_cost = $4, fuel_type = $5 
         WHERE id = $6"
    )
    .bind(&form.name)
    .bind(form.amount as i32)
    .bind(form.purchase_date)
    .bind(form.purchase_cost)
    .bind(form.fuel_type)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some(format!("Equipment '{}' updated successfully", form.name))),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update equipment: {}", e))),
    };

    let redirect = Some(format!("/equipment/{}", id));
    
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

async fn equipment_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if equipment has allocations, if yes, can't delete
    let has_allocations = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM equipment_allocation WHERE equipment_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_allocations {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete equipment: it has {} allocations. Remove allocations first.", count)),
                redirect: Some("/equipment".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Delete equipment from database
    let result = sqlx::query("DELETE FROM equipment WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Equipment deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete equipment: {}", e))),
    };

    let redirect = Some("/equipment".to_string());
    
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
struct EquipmentListRow {
    id: i32,
    name: String,
    total_amount: i32,
    available_amount: i32,
    purchase_date: NaiveDateTime,
    purchase_cost: f64,
}

async fn equipment_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<EquipmentListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT e.id, e.name, e.amount as total_amount, 
         (e.amount - COALESCE((SELECT SUM(ea.amount) FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE)), 0)) as available_amount,
         e.purchase_date, e.purchase_cost
         FROM equipment e"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE e.name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(department_id) = &filter.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND ea.department_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND ea.department_id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(" AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE))");
    }

    if let Some(site_id) = &filter.site_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND ea.site_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND ea.site_id = ");
            where_added = true;
        }
        query_builder.push_bind(site_id);
        query_builder.push(" AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE))");
    }

    if let Some(available) = &filter.available {
        let subquery = if *available {
            " (e.amount - COALESCE((SELECT SUM(ea.amount) FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE)), 0)) > 0"
        } else {
            " (e.amount - COALESCE((SELECT SUM(ea.amount) FROM equipment_allocation ea WHERE ea.equipment_id = e.id AND (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE)), 0)) = 0"
        };

        if where_added {
            query_builder.push(" AND ");
        } else {
            query_builder.push(" WHERE ");
            where_added = true;
        }
        query_builder.push(subquery);
    }

    // Count total results for pagination
    let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query_builder.sql());
    let count = match sqlx::query_scalar::<_, i64>(&count_query).fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting equipment: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("e.name"),
        "amount" => query_builder.push("e.amount"),
        "purchase_date" => query_builder.push("e.purchase_date"),
        "purchase_cost" => query_builder.push("e.purchase_cost"),
        _ => query_builder.push("e.id"),
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
    let query = query_builder.build_query_as::<EquipmentListRow>();
    let equipment_rows = match query.fetch_all(&*db.pool).await {
        Ok(equipment) => equipment,
        Err(e) => return Html::from(format!("<p>Error fetching equipment: {}</p>", e)),
    };

    // Convert to template items
    let equipment_items = equipment_rows.into_iter().map(|e| EquipmentListItem {
        id: e.id,
        name: e.name,
        total_amount: e.total_amount as u32,
        available_amount: e.available_amount as u32,
        purchase_date: e.purchase_date.format("%Y-%m-%d").to_string(),
        purchase_cost: e.purchase_cost,
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = EquipmentListTemplate {
        equipment: equipment_items,
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

async fn equipment_create_handler(
    State(db): State<Database>,
    Form(form): Form<EquipmentCreateForm>,
) -> Html<String> {
    // Insert new equipment into database
    let result = sqlx::query(
        "INSERT INTO equipment (name, amount, purchase_date, purchase_cost, fuel_type) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id"
    )
    .bind(&form.name)
    .bind(form.amount as i32)
    .bind(form.purchase_date)
    .bind(form.purchase_cost)
    .bind(form.fuel_type)
    .fetch_optional(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Equipment '{}' created successfully", form.name)),
                Some(format!("/equipment/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create equipment: no ID returned".to_string()),
            Some("/equipment".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create equipment: {}", e)),
            Some("/equipment".to_string())
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
struct AllocationRow {
    department_id: i32,
    department_name: String,
    site_id: Option<i32>,
    site_name: Option<String>,
    amount: i32,
    period_start: NaiveDateTime,
    period_end: Option<NaiveDateTime>,
    is_current: bool,
}

async fn equipment_allocations_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query allocations for this equipment
    let query = sqlx::query_as::<_, AllocationRow>(
        "SELECT ea.department_id, d.name as department_name,
         ea.site_id, s.name as site_name,
         ea.amount, ea.period_start, ea.period_end,
         (ea.period_end IS NULL OR ea.period_end > CURRENT_DATE) as is_current
         FROM equipment_allocation ea
         JOIN department d ON ea.department_id = d.id
         LEFT JOIN site s ON ea.site_id = s.id
         WHERE ea.equipment_id = $1
         ORDER BY is_current DESC, ea.period_start DESC
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i32)
    .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    let allocations = match query.fetch_all(&*db.pool).await {
        Ok(allocations) => allocations,
        Err(e) => return Html::from(format!("<p>Error fetching allocations: {}</p>", e)),
    };

    // Convert to template items
    let allocation_items = allocations.into_iter().map(|a| AllocationListItem {
        department_id: a.department_id,
        department_name: a.department_name,
        site_id: a.site_id,
        site_name: a.site_name,
        amount: a.amount as u32,
        period_start: a.period_start,
        period_end: a.period_end,
        is_current: a.is_current,
    }).collect();

    let template = EquipmentAllocationsTemplate { 
        allocations: allocation_items,
        pagination,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn equipment_create_allocation_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<EquipmentAllocationForm>,
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
                redirect: Some(format!("/equipment/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Check if site exists if provided
    if let Some(site_id) = form.site_id {
        let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
            .bind(site_id)
            .fetch_one(&*db.pool)
            .await;
        
        if let Ok(exists) = site_exists {
            if !exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Site with ID {} does not exist", site_id)),
                    redirect: Some(format!("/equipment/{}", id)),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Check if there is enough available equipment
    let available_amount = sqlx::query_scalar::<_, i32>(
        "SELECT amount - COALESCE((SELECT SUM(amount) FROM equipment_allocation WHERE equipment_id = $1 AND (period_end IS NULL OR period_end > $2)), 0) 
         FROM equipment WHERE id = $1"
    )
    .bind(id)
    .bind(form.period_start)
    .fetch_one(&*db.pool)
    .await;

    if let Ok(amount) = available_amount {
        if form.amount as i32 > amount {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Not enough available equipment. Available: {}, Requested: {}", amount, form.amount)),
                redirect: Some(format!("/equipment/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Insert new allocation into database
    let result = sqlx::query(
        "INSERT INTO equipment_allocation (equipment_id, department_id, site_id, amount, period_start, period_end) 
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id)
    .bind(form.department_id)
    .bind(form.site_id)
    .bind(form.amount as i32)
    .bind(form.period_start)
    .bind(form.period_end)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message) = match result {
        Ok(_) => (
            NotificationResult::Success,
            Some("Equipment allocation created successfully".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create equipment allocation: {}", e))
        ),
    };
    
    let redirect = Some(format!("/equipment/{}", id));
    
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