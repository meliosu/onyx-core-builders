use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

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
    pub id: i32,
}

#[derive(Template)]
#[template(path = "brigades/new.html")]
pub struct BrigadeNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/edit.html")]
pub struct BrigadeEditTemplate {
    pub id: i32,
    pub brigadier_id: i32,
    pub brigadier_name: String,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct BrigadeTabQuery {
    pub tab: BrigadeTab,
}

#[derive(Template, Serialize, Deserialize, FromRow)]
#[template(path = "brigades/api/details.html")]
pub struct BrigadeApiDetailsTemplate {
    pub id: i32,
    pub brigadier_id: i32,
    pub brigadier_name: String,
    pub worker_count: i32,
    pub current_task_id: Option<i32>,
    pub current_task_name: Option<String>,
    pub current_site_id: Option<i32>,
    pub current_site_name: Option<String>,
    pub tab: BrigadeTab,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeUpdateForm {
    pub brigadier_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeCreateForm {
    pub brigadier_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BrigadeListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub brigadier_id: Option<i32>,
    pub site_id: Option<i32>,
    pub task_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/list.html")]
pub struct BrigadeListTemplate {
    pub brigades: Vec<BrigadeListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BrigadeListItem {
    pub id: i32,
    pub brigadier_id: i32,
    pub brigadier_name: String,
    pub worker_count: i32,
    pub current_site_id: Option<i32>,
    pub current_site_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/workers.html")]
pub struct BrigadeWorkersTemplate {
    pub id: i32,
    pub workers: Vec<WorkerListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct WorkerListItem {
    pub id: i32,
    pub name: String,
    pub profession: Profession,
    pub is_brigadier: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerAddForm {
    pub worker_id: i32,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "brigades/api/tasks.html")]
pub struct BrigadeTasksTemplate {
    pub id: i32,
    pub tasks: Vec<TaskListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TaskListItem {
    pub id: i32,
    pub name: String,
    pub site_id: i32,
    pub site_name: String,
    pub period_start: NaiveDateTime,
    pub period_end: Option<NaiveDateTime>,
    pub status: String,
}

// Handler functions for page endpoints

async fn brigades_list_handler(State(db): State<Database>) -> Html<String> {
    let template = BrigadesListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn brigade_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    let template = BrigadeDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn brigade_new_handler(State(db): State<Database>) -> Html<String> {
    let template = BrigadeNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct BrigadierData {
    id: i32,
    brigadier_name: String,
}

async fn brigade_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get brigade data from database
    let query = sqlx::query_as::<_, BrigadierData>(
        "SELECT b.id, CONCAT(e.last_name, ' ', e.first_name) as brigadier_name 
         FROM brigade b
         JOIN worker w ON b.brigadier_id = w.id
         JOIN employee e ON w.id = e.id
         WHERE b.id = $1"
    )
    .bind(id);

    let brigade = match query.fetch_optional(&*db.pool).await {
        Ok(Some(brigade)) => brigade,
        Ok(None) => {
            return Html::from(format!("<p>Brigade with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching brigade: {}</p>", e));
        }
    };

    // Get brigadier id
    let brigadier_id = sqlx::query_scalar::<_, i32>("SELECT brigadier_id FROM brigade WHERE id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;

    let brigadier_id = match brigadier_id {
        Ok(id) => id,
        Err(e) => {
            return Html::from(format!("<p>Error fetching brigadier ID: {}</p>", e));
        }
    };

    // Render the template with brigade data
    let template = BrigadeEditTemplate {
        id: brigade.id,
        brigadier_id,
        brigadier_name: brigade.brigadier_name,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

async fn brigade_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(query): Query<BrigadeTabQuery>,
) -> Html<String> {
    // Construct complex query to get brigade details with current task and site
    let query_result = sqlx::query(
        "SELECT 
            b.id, 
            b.brigadier_id, 
            CONCAT(e.last_name, ' ', e.first_name) as brigadier_name,
            (SELECT COUNT(*) FROM assignment a WHERE a.brigade_id = b.id) as worker_count,
            t.id as current_task_id,
            t.name as current_task_name,
            s.id as current_site_id,
            s.name as current_site_name
        FROM brigade b
        JOIN worker w ON b.brigadier_id = w.id
        JOIN employee e ON w.id = e.id
        LEFT JOIN LATERAL (
            SELECT t.* 
            FROM task t 
            WHERE t.brigade_id = b.id AND t.actual_period_end IS NULL
            ORDER BY t.period_start DESC
            LIMIT 1
        ) t ON TRUE
        LEFT JOIN site s ON t.site_id = s.id
        WHERE b.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    match query_result {
        Ok(Some(row)) => {
            // Extract all values from the row
            let id: i32 = row.get("id");
            let brigadier_id: i32 = row.get("brigadier_id");
            let brigadier_name: String = row.get("brigadier_name");
            let worker_count: i32 = row.get("worker_count");
            let current_task_id: Option<i32> = row.get("current_task_id");
            let current_task_name: Option<String> = row.get("current_task_name");
            let current_site_id: Option<i32> = row.get("current_site_id");
            let current_site_name: Option<String> = row.get("current_site_name");

            let template = BrigadeApiDetailsTemplate {
                id,
                brigadier_id,
                brigadier_name,
                worker_count,
                current_task_id,
                current_task_name,
                current_site_id,
                current_site_name,
                tab: query.tab,
            };

            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(None) => {
            Html::from(format!("<p>Brigade with ID {} not found</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error fetching brigade details: {}</p>", e))
        }
    }
}

async fn brigade_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<BrigadeUpdateForm>,
) -> Html<String> {
    // Check if worker exists and is not already a brigadier in another brigade
    let worker_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM worker WHERE id = $1 AND 
            NOT EXISTS (SELECT 1 FROM brigade WHERE brigadier_id = $1 AND id != $2)
        )"
    )
    .bind(form.brigadier_id)
    .bind(id)
    .fetch_one(&*db.pool)
    .await;
    
    match worker_exists {
        Ok(true) => {
            // Update brigade
            let update_result = sqlx::query("UPDATE brigade SET brigadier_id = $1 WHERE id = $2")
                .bind(form.brigadier_id)
                .bind(id)
                .execute(&*db.pool)
                .await;
            
            // Get worker name for notification
            let worker_name = sqlx::query_scalar::<_, String>(
                "SELECT CONCAT(e.last_name, ' ', e.first_name) 
                 FROM employee e 
                 WHERE e.id = $1"
            )
            .bind(form.brigadier_id)
            .fetch_optional(&*db.pool)
            .await
            .ok()
            .flatten();
            
            let worker_name = worker_name.unwrap_or_else(|| "Unknown worker".to_string());
            
            let (result, message) = match update_result {
                Ok(_) => (
                    NotificationResult::Success, 
                    Some(format!("Brigade updated with new brigadier: {}", worker_name))
                ),
                Err(e) => (
                    NotificationResult::Error, 
                    Some(format!("Failed to update brigade: {}", e))
                ),
            };
            
            let template = NotificationTemplate {
                result,
                message,
                redirect: Some(format!("/brigades/{}", id)),
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Worker does not exist or is already a brigadier in another brigade".to_string()),
                redirect: Some(format!("/brigades/{}/edit", id)),
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking worker: {}</p>", e))
        }
    }
}

async fn brigade_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if brigade has active tasks
    let has_active_tasks = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM task 
            WHERE brigade_id = $1 AND actual_period_end IS NULL
        )"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;
    
    match has_active_tasks {
        Ok(true) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Cannot delete brigade with active tasks".to_string()),
                redirect: Some(format!("/brigades/{}", id)),
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            // Begin transaction
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    return Html::from(format!("<p>Error starting transaction: {}</p>", e));
                }
            };
            
            // Remove assignments
            if let Err(e) = sqlx::query("DELETE FROM assignment WHERE brigade_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
            {
                return Html::from(format!("<p>Error removing assignments: {}</p>", e));
            }
            
            // Delete brigade
            let delete_result = sqlx::query("DELETE FROM brigade WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await;
            
            match delete_result {
                Ok(_) => {
                    // Commit transaction
                    if let Err(e) = tx.commit().await {
                        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
                    }
                    
                    let template = NotificationTemplate {
                        result: NotificationResult::Success,
                        message: Some("Brigade successfully deleted".to_string()),
                        redirect: Some("/brigades".to_string()),
                    };
                    
                    match template.render() {
                        Ok(html) => Html::from(html),
                        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                    }
                },
                Err(e) => {
                    // Rollback transaction
                    let _ = tx.rollback().await;
                    
                    let template = NotificationTemplate {
                        result: NotificationResult::Error,
                        message: Some(format!("Failed to delete brigade: {}", e)),
                        redirect: Some(format!("/brigades/{}", id)),
                    };
                    
                    match template.render() {
                        Ok(html) => Html::from(html),
                        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                    }
                }
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking for active tasks: {}</p>", e))
        }
    }
}

async fn brigades_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<BrigadeListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT 
            b.id, 
            b.brigadier_id,
            CONCAT(e.last_name, ' ', e.first_name) as brigadier_name,
            (SELECT COUNT(*) FROM assignment a WHERE a.brigade_id = b.id) as worker_count,
            s.id as current_site_id,
            s.name as current_site_name
        FROM brigade b
        JOIN worker w ON b.brigadier_id = w.id
        JOIN employee e ON w.id = e.id
        LEFT JOIN LATERAL (
            SELECT t.site_id, s.name as site_name
            FROM task t 
            JOIN site s ON t.site_id = s.id
            WHERE t.brigade_id = b.id AND t.actual_period_end IS NULL
            ORDER BY t.period_start DESC
            LIMIT 1
        ) ts ON TRUE
        LEFT JOIN site s ON ts.site_id = s.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(brigadier_id) = &filter.brigadier_id {
        query_builder.push(" WHERE b.brigadier_id = ");
        query_builder.push_bind(brigadier_id);
        where_added = true;
    }

    if let Some(site_id) = &filter.site_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.site_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.site_id = ");
            where_added = true;
        }
        query_builder.push_bind(site_id);
        query_builder.push(")");
    }

    if let Some(task_name) = &filter.task_name {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.name ILIKE ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", task_name));
        query_builder.push(")");
    }

    // Count total results for pagination
    let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query_builder.sql());
    let count = match sqlx::query_scalar::<_, i64>(&count_query).fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting brigades: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "brigadier_name" => query_builder.push("brigadier_name"),
        "worker_count" => query_builder.push("worker_count"),
        _ => query_builder.push("b.id"),
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
    let query = query_builder.build_query_as::<BrigadeListItem>();
    let brigades = match query.fetch_all(&*db.pool).await {
        Ok(brigades) => brigades,
        Err(e) => return Html::from(format!("<p>Error fetching brigades: {}</p>", e)),
    };

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = BrigadeListTemplate {
        brigades,
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

async fn brigade_create_handler(
    State(db): State<Database>,
    Form(form): Form<BrigadeCreateForm>,
) -> Html<String> {
    // Check if worker exists and is not already a brigadier
    let worker_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM worker WHERE id = $1 AND 
            NOT EXISTS (SELECT 1 FROM brigade WHERE brigadier_id = $1)
        )"
    )
    .bind(form.brigadier_id)
    .fetch_one(&*db.pool)
    .await;
    
    match worker_exists {
        Ok(true) => {
            // Insert new brigade
            let result = sqlx::query("INSERT INTO brigade (brigadier_id) VALUES ($1) RETURNING id")
                .bind(form.brigadier_id)
                .fetch_optional(&*db.pool)
                .await;
            
            // Get worker name for notification
            let worker_name = sqlx::query_scalar::<_, String>(
                "SELECT CONCAT(e.last_name, ' ', e.first_name) 
                 FROM employee e 
                 WHERE e.id = $1"
            )
            .bind(form.brigadier_id)
            .fetch_optional(&*db.pool)
            .await
            .ok()
            .flatten();
            
            let worker_name = worker_name.unwrap_or_else(|| "Unknown worker".to_string());
                
            // Create notification based on result
            let (notification_result, message, redirect) = match result {
                Ok(Some(row)) => {
                    let id: i32 = row.get(0);
                    (
                        NotificationResult::Success,
                        Some(format!("Brigade with brigadier {} created successfully", worker_name)),
                        Some(format!("/brigades/{}", id))
                    )
                },
                Ok(None) => (
                    NotificationResult::Error, 
                    Some("Failed to create brigade: no ID returned".to_string()),
                    Some("/brigades".to_string())
                ),
                Err(e) => (
                    NotificationResult::Error, 
                    Some(format!("Failed to create brigade: {}", e)),
                    Some("/brigades".to_string())
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
        },
        Ok(false) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Worker does not exist or is already a brigadier".to_string()),
                redirect: Some("/brigades/new".to_string()),
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking worker: {}</p>", e))
        }
    }
}

async fn brigade_workers_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if brigade exists
    let brigade_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brigade WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match brigade_exists {
        Ok(true) => {
            // Build query for workers in this brigade
            let query = sqlx::query_as::<_, WorkerListItem>(
                "SELECT 
                    w.id, 
                    CONCAT(e.last_name, ' ', e.first_name) as name,
                    w.profession,
                    (b.brigadier_id = w.id) as is_brigadier
                FROM worker w
                JOIN employee e ON w.id = e.id
                JOIN assignment a ON a.worker_id = w.id
                JOIN brigade b ON a.brigade_id = b.id
                WHERE b.id = $1
                ORDER BY is_brigadier DESC, name
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let workers = match query.fetch_all(&*db.pool).await {
                Ok(workers) => workers,
                Err(e) => return Html::from(format!("<p>Error fetching workers: {}</p>", e)),
            };
            
            // Count total workers
            let count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) 
                 FROM assignment 
                 WHERE brigade_id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;
            
            let count = match count {
                Ok(count) => count,
                Err(e) => return Html::from(format!("<p>Error counting workers: {}</p>", e)),
            };
            
            // Calculate number of pages
            let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;
            
            let template = BrigadeWorkersTemplate {
                id,
                workers,
                pagination: Pagination {
                    page_number: pagination.page_number,
                    page_size: pagination.page_size,
                },
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Brigade with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking brigade: {}</p>", e))
        }
    }
}

async fn worker_add_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<WorkerAddForm>,
) -> Html<String> {
    // Check if worker exists and is not already in this or another brigade
    let worker_check = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM worker WHERE id = $1 AND 
            NOT EXISTS (SELECT 1 FROM assignment WHERE worker_id = $1)
        )"
    )
    .bind(form.worker_id)
    .fetch_one(&*db.pool)
    .await;
    
    match worker_check {
        Ok(true) => {
            // Add worker to brigade
            let result = sqlx::query(
                "INSERT INTO assignment (brigade_id, worker_id) VALUES ($1, $2)"
            )
            .bind(id)
            .bind(form.worker_id)
            .execute(&*db.pool)
            .await;
            
            // Get worker name for notification
            let worker_name = sqlx::query_scalar::<_, String>(
                "SELECT CONCAT(e.last_name, ' ', e.first_name) 
                 FROM employee e 
                 WHERE e.id = $1"
            )
            .bind(form.worker_id)
            .fetch_optional(&*db.pool)
            .await
            .ok()
            .flatten();
            
            let worker_name = worker_name.unwrap_or_else(|| "Unknown worker".to_string());
            
            // Create notification based on result
            let (notification_result, message) = match result {
                Ok(_) => (
                    NotificationResult::Success,
                    Some(format!("Worker {} added to brigade successfully", worker_name)),
                ),
                Err(e) => (
                    NotificationResult::Error, 
                    Some(format!("Failed to add worker to brigade: {}", e)),
                ),
            };
            
            let template = NotificationTemplate {
                result: notification_result,
                message,
                redirect: None,
            };

            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Worker does not exist or is already assigned to a brigade".to_string()),
                redirect: None,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking worker: {}</p>", e))
        }
    }
}

async fn worker_remove_handler(
    State(db): State<Database>,
    Path((id, worker_id)): Path<(i32, i32)>,
) -> Html<String> {
    // Check if worker is the brigadier
    let is_brigadier = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM brigade
            WHERE id = $1 AND brigadier_id = $2
        )"
    )
    .bind(id)
    .bind(worker_id)
    .fetch_one(&*db.pool)
    .await;
    
    match is_brigadier {
        Ok(true) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Cannot remove the brigadier from the brigade".to_string()),
                redirect: None,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            // Remove worker from brigade
            let result = sqlx::query(
                "DELETE FROM assignment WHERE brigade_id = $1 AND worker_id = $2"
            )
            .bind(id)
            .bind(worker_id)
            .execute(&*db.pool)
            .await;
            
            // Get worker name for notification
            let worker_name = sqlx::query_scalar::<_, String>(
                "SELECT CONCAT(e.last_name, ' ', e.first_name) 
                 FROM employee e 
                 WHERE e.id = $1"
            )
            .bind(worker_id)
            .fetch_optional(&*db.pool)
            .await
            .ok()
            .flatten();
            
            let worker_name = worker_name.unwrap_or_else(|| "Unknown worker".to_string());
            
            // Create notification based on result
            let (notification_result, message) = match result {
                Ok(result) => {
                    if result.rows_affected() == 0 {
                        (
                            NotificationResult::Error,
                            Some(format!("Worker {} is not in this brigade", worker_name)),
                        )
                    } else {
                        (
                            NotificationResult::Success,
                            Some(format!("Worker {} removed from brigade successfully", worker_name)),
                        )
                    }
                },
                Err(e) => (
                    NotificationResult::Error, 
                    Some(format!("Failed to remove worker from brigade: {}", e)),
                ),
            };
            
            let template = NotificationTemplate {
                result: notification_result,
                message,
                redirect: None,
            };

            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking if worker is brigadier: {}</p>", e))
        }
    }
}

async fn brigade_tasks_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if brigade exists
    let brigade_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brigade WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match brigade_exists {
        Ok(true) => {
            // Build query for tasks assigned to this brigade
            let query = sqlx::query_as::<_, TaskListItem>(
                "SELECT 
                    t.id,
                    t.name,
                    t.site_id,
                    s.name as site_name,
                    t.period_start,
                    t.actual_period_end as period_end,
                    CASE
                        WHEN t.actual_period_end IS NULL THEN 'In Progress'
                        ELSE 'Completed'
                    END as status
                FROM task t
                JOIN site s ON t.site_id = s.id
                WHERE t.brigade_id = $1
                ORDER BY 
                    CASE WHEN t.actual_period_end IS NULL THEN 0 ELSE 1 END,
                    t.period_start DESC
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let tasks = match query.fetch_all(&*db.pool).await {
                Ok(tasks) => tasks,
                Err(e) => return Html::from(format!("<p>Error fetching tasks: {}</p>", e)),
            };
            
            // Count total tasks
            let count = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) 
                 FROM task 
                 WHERE brigade_id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;
            
            let count = match count {
                Ok(count) => count,
                Err(e) => return Html::from(format!("<p>Error counting tasks: {}</p>", e)),
            };
            
            // Calculate number of pages
            let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;
            
            let template = BrigadeTasksTemplate {
                id,
                tasks,
                pagination: Pagination {
                    page_number: pagination.page_number,
                    page_size: pagination.page_size,
                },
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Brigade with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking brigade: {}</p>", e))
        }
    }
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