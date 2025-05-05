use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use chrono::NaiveDate;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate}};
use crate::utils::empty_string_as_none;

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

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planned" => Ok(TaskStatus::Planned),
            "in_progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            _ => Err(format!("Invalid TaskStatus: {}", s)),
        }
    }
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "tasks/list.html")]
pub struct TasksListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/details.html")]
pub struct TaskDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "tasks/new.html")]
pub struct TaskNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/edit.html")]
pub struct TaskEditTemplate {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub site_id: i32,
    pub site_name: String,
    pub brigade_id: Option<i32>,
    pub brigadier_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct TaskTabQuery {
    pub tab: TaskTab,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/details.html")]
pub struct TaskApiDetailsTemplate {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub site_id: i32,
    pub site_name: String,
    pub brigade_id: Option<i32>,
    pub brigade_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub status: TaskStatus,
    pub tab: TaskTab,
}

#[derive(Serialize, Deserialize)]
pub struct TaskUpdateForm {
    pub name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub description: Option<String>,
    pub site_id: i32,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigade_id: Option<i32>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCreateForm {
    pub name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub description: Option<String>,
    pub site_id: i32,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigade_id: Option<i32>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub site_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigade_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub status: Option<TaskStatus>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub date_from: Option<NaiveDate>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub date_to: Option<NaiveDate>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub exceeded_deadline: Option<bool>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/list.html")]
pub struct TaskListTemplate {
    pub tasks: Vec<TaskListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize)]
pub struct TaskListItem {
    pub id: i32,
    pub name: String,
    pub site_id: i32,
    pub site_name: String,
    pub brigade_id: Option<i32>,
    pub brigadier_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub status: TaskStatus,
    pub deadline_exceeded: bool,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "tasks/api/materials.html")]
pub struct TaskMaterialsTemplate {
    pub id: i32,
    pub materials: Vec<TaskMaterialItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialItem {
    pub material_id: i32,
    pub name: String,
    pub expected_amount: f32,
    pub actual_amount: Option<f32>,
    pub units: String,
    pub cost: f32,
    pub total_cost: f32,
    pub excess: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialForm {
    pub material_id: i32,
    pub expected_amount: f32,
}

#[derive(Serialize, Deserialize)]
pub struct TaskMaterialUpdateForm {
    pub actual_amount: f32,
}

#[derive(Serialize, Deserialize)]
pub struct TaskCompleteForm {
    pub actual_period_end: NaiveDate,
}

// Handler functions for page endpoints

async fn tasks_list_handler(State(db): State<Database>) -> Html<String> {
    // Return the tasks list page template
    let template = TasksListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn task_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the task details page with the task ID
    let template = TaskDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn task_new_handler(State(db): State<Database>) -> Html<String> {
    // Return the new task form template
    let template = TaskNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

#[derive(FromRow)]
struct TaskEditData {
    id: i32,
    name: String,
    description: Option<String>,
    site_id: i32,
    site_name: String,
    brigade_id: Option<i32>,
    brigade_name: Option<String>,
    period_start: NaiveDate,
    expected_period_end: NaiveDate,
}

async fn task_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get task data from database
    let query = sqlx::query_as::<_, TaskEditData>(
        "SELECT t.id, t.name, t.description, t.site_id, s.name as site_name, 
         t.brigade_id, 
         CASE WHEN t.brigade_id IS NOT NULL THEN 
            (SELECT CONCAT(e.last_name, ' ', e.first_name) 
             FROM worker w
             JOIN brigade b ON w.id = b.brigadier_id
             JOIN employee e ON w.id = e.id
             WHERE b.id = t.brigade_id)
         ELSE NULL END as brigade_name,
         t.period_start, t.expected_period_end
         FROM task t
         JOIN site s ON t.site_id = s.id
         WHERE t.id = $1"
    )
    .bind(id);

    let task = match query.fetch_optional(&*db.pool).await {
        Ok(Some(task)) => task,
        Ok(None) => {
            return Html::from(format!("<p>Task with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching task: {}</p>", e));
        }
    };

    // Return the task edit template with data
    let template = TaskEditTemplate {
        id: task.id,
        name: task.name,
        description: task.description,
        site_id: task.site_id,
        site_name: task.site_name,
        brigade_id: task.brigade_id,
        brigadier_name: task.brigade_name,  // Changed from brigade_name to brigadier_name
        period_start: task.period_start,
        expected_period_end: task.expected_period_end,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

#[derive(FromRow)]
struct TaskDetails {
    id: i32,
    name: String,
    description: Option<String>,
    site_id: i32,
    site_name: String,
    brigade_id: Option<i32>,
    brigade_name: Option<String>,
    period_start: NaiveDate,
    expected_period_end: NaiveDate,
    actual_period_end: Option<NaiveDate>,
}

async fn task_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(query): Query<TaskTabQuery>,
) -> Html<String> {
    // Fetch the task details from database
    let query_result = sqlx::query_as::<_, TaskDetails>(
        "SELECT t.id, t.name, t.description, t.site_id, s.name as site_name, 
         t.brigade_id, 
         CASE WHEN t.brigade_id IS NOT NULL THEN 
            (SELECT CONCAT(e.last_name, ' ', e.first_name) 
             FROM worker w
             JOIN brigade b ON w.id = b.brigadier_id
             JOIN employee e ON w.id = e.id
             WHERE b.id = t.brigade_id)
         ELSE NULL END as brigade_name,
         t.period_start, t.expected_period_end, t.actual_period_end
         FROM task t
         JOIN site s ON t.site_id = s.id
         WHERE t.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let task = match query_result {
        Ok(Some(task)) => task,
        Ok(None) => {
            return Html::from(format!("<p>Task with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching task details: {}</p>", e));
        }
    };

    // Determine task status
    let status = if task.actual_period_end.is_some() {
        TaskStatus::Completed
    } else if task.brigade_id.is_some() {
        TaskStatus::InProgress
    } else {
        TaskStatus::Planned
    };

    // Return task details with selected tab
    let template = TaskApiDetailsTemplate {
        id: task.id,
        name: task.name,
        description: task.description,
        site_id: task.site_id,
        site_name: task.site_name,
        brigade_id: task.brigade_id,
        brigade_name: task.brigade_name,
        period_start: task.period_start,
        expected_period_end: task.expected_period_end,
        actual_period_end: task.actual_period_end,
        status,
        tab: query.tab,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn task_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<TaskUpdateForm>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(form.site_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = site_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Site with ID {} does not exist", form.site_id)),
                redirect: Some(format!("/tasks/{}/edit", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Check if brigade exists if provided
    if let Some(brigade_id) = form.brigade_id {
        let brigade_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brigade WHERE id = $1)")
            .bind(brigade_id)
            .fetch_one(&*db.pool)
            .await;
        
        if let Ok(exists) = brigade_exists {
            if !exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Brigade with ID {} does not exist", brigade_id)),
                    redirect: Some(format!("/tasks/{}/edit", id)),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Validate dates
    if form.period_start >= form.expected_period_end {
        let template = NotificationTemplate {
            result: NotificationResult::Error,
            message: Some("Expected end date must be after start date".to_string()),
            redirect: Some(format!("/tasks/{}/edit", id)),
        };
        return match template.render() {
            Ok(html) => Html::from(html),
            Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
        };
    }

    // Update task in the database
    let result = sqlx::query(
        "UPDATE task SET name = $1, description = $2, site_id = $3, brigade_id = $4, 
         period_start = $5, expected_period_end = $6 WHERE id = $7"
    )
    .bind(&form.name)
    .bind(form.description)
    .bind(form.site_id)
    .bind(form.brigade_id)
    .bind(form.period_start)
    .bind(form.expected_period_end)
    .bind(id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some(format!("Task '{}' updated successfully", form.name))),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update task: {}", e))),
    };

    let redirect = Some(format!("/tasks/{}", id));
    
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

async fn task_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if task has expenditures, if yes, can't delete
    let has_expenditures = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM expenditure WHERE task_id = $1")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(count) = has_expenditures {
        if count > 0 {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Cannot delete task: it has {} material expenditures. Remove them first.", count)),
                redirect: Some("/tasks".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Delete task from database
    let result = sqlx::query("DELETE FROM task WHERE id = $1")
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Task deleted successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to delete task: {}", e))),
    };

    let redirect = Some("/tasks".to_string());
    
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
struct TaskListRow {
    id: i32,
    name: String,
    site_id: i32,
    site_name: String,
    brigade_id: Option<i32>,
    brigade_name: Option<String>,
    period_start: NaiveDate,
    expected_period_end: NaiveDate,
    actual_period_end: Option<NaiveDate>,
    current_date: NaiveDate,
}

async fn tasks_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<TaskListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT t.id, t.name, t.site_id, s.name as site_name, 
         t.brigade_id, 
         CASE WHEN t.brigade_id IS NOT NULL THEN 
            (SELECT CONCAT(e.last_name, ' ', e.first_name) 
             FROM worker w
             JOIN brigade b ON w.id = b.brigadier_id
             JOIN employee e ON w.id = e.id
             WHERE b.id = t.brigade_id)
         ELSE NULL END as brigade_name,
         t.period_start, t.expected_period_end, t.actual_period_end,
         CURRENT_DATE as current_date
         FROM task t
         JOIN site s ON t.site_id = s.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(name) = &filter.name {
        query_builder.push(" WHERE t.name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        where_added = true;
    }

    if let Some(site_id) = &filter.site_id {
        if where_added {
            query_builder.push(" AND t.site_id = ");
        } else {
            query_builder.push(" WHERE t.site_id = ");
            where_added = true;
        }
        query_builder.push_bind(site_id);
    }

    if let Some(brigade_id) = &filter.brigade_id {
        if where_added {
            query_builder.push(" AND t.brigade_id = ");
        } else {
            query_builder.push(" WHERE t.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
    }

    if let Some(status) = &filter.status {
        if where_added {
            query_builder.push(" AND ");
        } else {
            query_builder.push(" WHERE ");
            where_added = true;
        }

        match status {
            TaskStatus::Completed => {
                query_builder.push("t.actual_period_end IS NOT NULL");
            },
            TaskStatus::InProgress => {
                query_builder.push("t.actual_period_end IS NULL AND t.brigade_id IS NOT NULL");
            },
            TaskStatus::Planned => {
                query_builder.push("t.brigade_id IS NULL");
            },
        }
    }

    if let Some(date_from) = &filter.date_from {
        if where_added {
            query_builder.push(" AND t.period_start >= ");
        } else {
            query_builder.push(" WHERE t.period_start >= ");
            where_added = true;
        }
        query_builder.push_bind(date_from);
    }

    if let Some(date_to) = &filter.date_to {
        if where_added {
            query_builder.push(" AND t.period_start <= ");
        } else {
            query_builder.push(" WHERE t.period_start <= ");
            where_added = true;
        }
        query_builder.push_bind(date_to);
    }

    if let Some(exceeded_deadline) = &filter.exceeded_deadline {
        if *exceeded_deadline {
            if where_added {
                query_builder.push(" AND ((t.actual_period_end IS NOT NULL AND t.actual_period_end > t.expected_period_end) OR (t.actual_period_end IS NULL AND CURRENT_DATE > t.expected_period_end))");
            } else {
                query_builder.push(" WHERE ((t.actual_period_end IS NOT NULL AND t.actual_period_end > t.expected_period_end) OR (t.actual_period_end IS NULL AND CURRENT_DATE > t.expected_period_end))");
                where_added = true;
            }
        }
    }

    // Count total results for pagination - PROPERLY BIND PARAMETERS
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM task t JOIN site s ON t.site_id = s.id"
    );
    
    // Copy the same where conditions for the count query
    let mut count_where_added = false;
    
    if let Some(name) = &filter.name {
        count_query_builder.push(" WHERE t.name ILIKE ");
        count_query_builder.push_bind(format!("%{}%", name));
        count_where_added = true;
    }

    if let Some(site_id) = &filter.site_id {
        if count_where_added {
            count_query_builder.push(" AND t.site_id = ");
        } else {
            count_query_builder.push(" WHERE t.site_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(site_id);
    }

    if let Some(brigade_id) = &filter.brigade_id {
        if count_where_added {
            count_query_builder.push(" AND t.brigade_id = ");
        } else {
            count_query_builder.push(" WHERE t.brigade_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(brigade_id);
    }

    if let Some(status) = &filter.status {
        if count_where_added {
            count_query_builder.push(" AND ");
        } else {
            count_query_builder.push(" WHERE ");
            count_where_added = true;
        }

        match status {
            TaskStatus::Completed => {
                count_query_builder.push("t.actual_period_end IS NOT NULL");
            },
            TaskStatus::InProgress => {
                count_query_builder.push("t.actual_period_end IS NULL AND t.brigade_id IS NOT NULL");
            },
            TaskStatus::Planned => {
                count_query_builder.push("t.brigade_id IS NULL");
            },
        }
    }

    if let Some(date_from) = &filter.date_from {
        if count_where_added {
            count_query_builder.push(" AND t.period_start >= ");
        } else {
            count_query_builder.push(" WHERE t.period_start >= ");
            count_where_added = true;
        }
        count_query_builder.push_bind(date_from);
    }

    if let Some(date_to) = &filter.date_to {
        if count_where_added {
            count_query_builder.push(" AND t.period_start <= ");
        } else {
            count_query_builder.push(" WHERE t.period_start <= ");
            count_where_added = true;
        }
        count_query_builder.push_bind(date_to);
    }

    if let Some(exceeded_deadline) = &filter.exceeded_deadline {
        if *exceeded_deadline {
            if count_where_added {
                count_query_builder.push(" AND ((t.actual_period_end IS NOT NULL AND t.actual_period_end > t.expected_period_end) OR (t.actual_period_end IS NULL AND CURRENT_DATE > t.expected_period_end))");
            } else {
                count_query_builder.push(" WHERE ((t.actual_period_end IS NOT NULL AND t.actual_period_end > t.expected_period_end) OR (t.actual_period_end IS NULL AND CURRENT_DATE > t.expected_period_end))");
            }
        }
    }
    
    let count = match count_query_builder.build_query_scalar::<i64>().fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting tasks: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("t.name"),
        "site" => query_builder.push("site_name"),
        "brigade" => query_builder.push("brigade_name"),
        "period_start" => query_builder.push("t.period_start"),
        "expected_period_end" => query_builder.push("t.expected_period_end"),
        "status" => query_builder.push("t.actual_period_end, t.brigade_id"),
        _ => query_builder.push("t.id"),
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
    let query = query_builder.build_query_as::<TaskListRow>();
    let tasks = match query.fetch_all(&*db.pool).await {
        Ok(tasks) => tasks,
        Err(e) => return Html::from(format!("<p>Error fetching tasks: {}</p>", e)),
    };

    // Convert to template items
    let task_items = tasks.into_iter().map(|t| {
        let status = if t.actual_period_end.is_some() {
            TaskStatus::Completed
        } else if t.brigade_id.is_some() {
            TaskStatus::InProgress
        } else {
            TaskStatus::Planned
        };
        
        let deadline_exceeded = match t.actual_period_end {
            Some(end_date) => end_date > t.expected_period_end,
            None => t.current_date > t.expected_period_end,
        };
        
        TaskListItem {
            id: t.id,
            name: t.name,
            site_id: t.site_id,
            site_name: t.site_name,
            brigade_id: t.brigade_id,
            brigadier_name: t.brigade_name,  // Changed from brigade_name to brigadier_name
            period_start: t.period_start,
            expected_period_end: t.expected_period_end,
            actual_period_end: t.actual_period_end,
            status,
            deadline_exceeded,
        }
    }).collect();

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = TaskListTemplate {
        tasks: task_items,
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

async fn task_create_handler(
    State(db): State<Database>,
    Form(form): Form<TaskCreateForm>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(form.site_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = site_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Site with ID {} does not exist", form.site_id)),
                redirect: Some("/tasks/new".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Check if brigade exists if provided
    if let Some(brigade_id) = form.brigade_id {
        let brigade_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brigade WHERE id = $1)")
            .bind(brigade_id)
            .fetch_one(&*db.pool)
            .await;
        
        if let Ok(exists) = brigade_exists {
            if !exists {
                let template = NotificationTemplate {
                    result: NotificationResult::Error,
                    message: Some(format!("Brigade with ID {} does not exist", brigade_id)),
                    redirect: Some("/tasks/new".to_string()),
                };
                return match template.render() {
                    Ok(html) => Html::from(html),
                    Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                };
            }
        }
    }

    // Validate dates
    if form.period_start >= form.expected_period_end {
        let template = NotificationTemplate {
            result: NotificationResult::Error,
            message: Some("Expected end date must be after start date".to_string()),
            redirect: Some("/tasks/new".to_string()),
        };
        return match template.render() {
            Ok(html) => Html::from(html),
            Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
        };
    }

    // Insert new task into database
    let result = sqlx::query(
        "INSERT INTO task (name, description, site_id, brigade_id, period_start, expected_period_end) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id"
    )
    .bind(&form.name)
    .bind(form.description)
    .bind(form.site_id)
    .bind(form.brigade_id)
    .bind(form.period_start)
    .bind(form.expected_period_end)
    .fetch_optional(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message, redirect) = match result {
        Ok(Some(row)) => {
            let id: i32 = row.get(0);
            (
                NotificationResult::Success,
                Some(format!("Task '{}' created successfully", form.name)),
                Some(format!("/tasks/{}", id))
            )
        },
        Ok(None) => (
            NotificationResult::Error, 
            Some("Failed to create task: no ID returned".to_string()),
            Some("/tasks".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to create task: {}", e)),
            Some("/tasks".to_string())
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
struct TaskMaterialRow {
    material_id: i32,
    name: String,
    expected_amount: f32,
    actual_amount: Option<f32>,
    units: String,
    cost: f32,
}

async fn task_materials_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Query materials for this task
    let query = sqlx::query_as::<_, TaskMaterialRow>(
        "SELECT m.id as material_id, m.name, e.expected_amount, e.actual_amount, m.units, m.cost
         FROM expenditure e
         JOIN material m ON e.material_id = m.id
         WHERE e.task_id = $1
         ORDER BY m.name
         LIMIT $2 OFFSET $3"
    )
    .bind(id)
    .bind(pagination.page_size as i32)
    .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);

    let materials = match query.fetch_all(&*db.pool).await {
        Ok(materials) => materials,
        Err(e) => return Html::from(format!("<p>Error fetching task materials: {}</p>", e)),
    };

    // Convert to template items
    let material_items = materials.into_iter().map(|m| {
        let actual = m.actual_amount.unwrap_or(0.0);
        let excess = if let Some(actual_amount) = m.actual_amount {
            if actual_amount > m.expected_amount {
                Some(actual_amount - m.expected_amount)
            } else {
                None
            }
        } else {
            None
        };
        
        TaskMaterialItem {
            material_id: m.material_id,
            name: m.name,
            expected_amount: m.expected_amount,
            actual_amount: m.actual_amount,
            units: m.units,
            cost: m.cost,
            total_cost: actual * m.cost,
            excess,
        }
    }).collect();

    let template = TaskMaterialsTemplate {
        id,
        materials: material_items,
        pagination,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn task_add_material_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<TaskMaterialForm>,
) -> Html<String> {
    // Check if material exists
    let material_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM material WHERE id = $1)")
        .bind(form.material_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = material_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Material with ID {} does not exist", form.material_id)),
                redirect: Some(format!("/tasks/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Check if task exists
    let task_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM task WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(exists) = task_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Task with ID {} does not exist", id)),
                redirect: Some("/tasks".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Check if this material is already added to this task
    let material_already_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM expenditure WHERE task_id = $1 AND material_id = $2)"
    )
    .bind(id)
    .bind(form.material_id)
    .fetch_one(&*db.pool)
    .await;
    
    if let Ok(exists) = material_already_exists {
        if exists {
            // Update the existing expenditure instead
            let update_result = sqlx::query(
                "UPDATE expenditure SET expected_amount = $1 WHERE task_id = $2 AND material_id = $3"
            )
            .bind(form.expected_amount)
            .bind(id)
            .bind(form.material_id)
            .execute(&*db.pool)
            .await;

            let (result, message) = match update_result {
                Ok(_) => (NotificationResult::Success, Some("Material expenditure updated successfully".to_string())),
                Err(e) => (NotificationResult::Error, Some(format!("Failed to update material expenditure: {}", e))),
            };

            let template = NotificationTemplate {
                result,
                message,
                redirect: Some(format!("/tasks/{}", id)),
            };

            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Insert new expenditure into database
    let result = sqlx::query(
        "INSERT INTO expenditure (task_id, material_id, expected_amount) 
         VALUES ($1, $2, $3)"
    )
    .bind(id)
    .bind(form.material_id)
    .bind(form.expected_amount)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (notification_result, message) = match result {
        Ok(_) => (
            NotificationResult::Success,
            Some("Material expenditure added successfully".to_string())
        ),
        Err(e) => (
            NotificationResult::Error, 
            Some(format!("Failed to add material expenditure: {}", e))
        ),
    };
    
    let template = NotificationTemplate {
        result: notification_result,
        message,
        redirect: Some(format!("/tasks/{}", id)),
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn task_update_material_handler(
    State(db): State<Database>,
    Path((task_id, material_id)): Path<(i32, i32)>,
    Form(form): Form<TaskMaterialUpdateForm>,
) -> Html<String> {
    // Check if expenditure exists
    let expenditure_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM expenditure WHERE task_id = $1 AND material_id = $2)"
    )
    .bind(task_id)
    .bind(material_id)
    .fetch_one(&*db.pool)
    .await;
    
    if let Ok(exists) = expenditure_exists {
        if !exists {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Material with ID {} is not assigned to this task", material_id)),
                redirect: Some(format!("/tasks/{}", task_id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    }

    // Update actual amount in the database
    let result = sqlx::query(
        "UPDATE expenditure SET actual_amount = $1 WHERE task_id = $2 AND material_id = $3"
    )
    .bind(form.actual_amount)
    .bind(task_id)
    .bind(material_id)
    .execute(&*db.pool)
    .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => (NotificationResult::Success, Some("Material usage updated successfully".to_string())),
        Err(e) => (NotificationResult::Error, Some(format!("Failed to update material usage: {}", e))),
    };

    let redirect = Some(format!("/tasks/{}", task_id));
    
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

async fn task_complete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<TaskCompleteForm>,
) -> Html<String> {
    // Check if task exists
    let task_result = sqlx::query("SELECT period_start, expected_period_end FROM task WHERE id = $1")
        .bind(id)
        .fetch_optional(&*db.pool)
        .await;
    
    let (period_start, expected_period_end) = match task_result {
        Ok(Some(row)) => {
            let start: NaiveDate = row.get("period_start");
            let end: NaiveDate = row.get("expected_period_end");
            (start, end)
        },
        Ok(None) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Task with ID {} does not exist", id)),
                redirect: Some("/tasks".to_string()),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
        Err(e) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some(format!("Failed to fetch task: {}", e)),
                redirect: Some(format!("/tasks/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
    };

    // Check if completion date is valid (can't be before start date)
    if form.actual_period_end < period_start {
        let template = NotificationTemplate {
            result: NotificationResult::Error,
            message: Some("Completion date cannot be before the start date".to_string()),
            redirect: Some(format!("/tasks/{}", id)),
        };
        return match template.render() {
            Ok(html) => Html::from(html),
            Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
        };
    }

    // Mark task as completed in the database
    let result = sqlx::query("UPDATE task SET actual_period_end = $1 WHERE id = $2")
        .bind(form.actual_period_end)
        .bind(id)
        .execute(&*db.pool)
        .await;

    // Create notification based on result
    let (result, message) = match result {
        Ok(_) => {
            let deadline_message = if form.actual_period_end > expected_period_end {
                " (deadline exceeded)"
            } else {
                ""
            };
            (NotificationResult::Success, Some(format!("Task marked as completed{}", deadline_message)))
        },
        Err(e) => (NotificationResult::Error, Some(format!("Failed to complete task: {}", e))),
    };

    let redirect = Some(format!("/tasks/{}", id));
    
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