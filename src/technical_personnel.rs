use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Qualification, Position, Gender}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};
use crate::utils::empty_string_as_none;
use crate::utils::deserialize_sequence;
use crate::utils::deserialize_checkbox;

// Qualification-specific fields
#[derive(Serialize, Deserialize)]
#[serde(tag = "qualification_fields_type", rename_all = "snake_case")]
pub enum QualificationFields {
    Technician(TechnicianFields),
    Technologist(TechnologistFields),
    Engineer(EngineerFields),
}

// Technical personnel qualification-specific field structs
#[derive(Serialize, Deserialize)]
pub struct TechnicianFields {
    pub safety_training_level: String,
}

#[derive(Serialize, Deserialize)]
pub struct TechnologistFields {
    pub management_tools: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EngineerFields {
    pub pe_license_id: i32,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "technical_personnel/list.html")]
pub struct TechnicalPersonnelListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/details.html")]
pub struct TechnicalPersonnelDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "technical_personnel/new.html")]
pub struct TechnicalPersonnelNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/edit.html")]
pub struct TechnicalPersonnelEditTemplate {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Vec<String>,
    pub is_project_manager: bool,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelQualificationQuery {
    pub qualification: Qualification,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/details.html")]
pub struct TechnicalPersonnelApiDetailsTemplate {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Vec<String>,
    pub is_project_manager: bool,
    pub qualification_fields: QualificationFields,
    pub supervising_department_id: Option<i32>,
    pub supervising_department_name: Option<String>,
    pub supervising_area_id: Option<i32>,
    pub supervising_area_name: Option<String>,
    pub area_id: Option<i32>,
    pub area_name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/qualification-fields.html")]
pub struct TechnicalPersonnelQualificationFieldsTemplate {
    pub qualification: Qualification,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelUpdateForm {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub position: Option<Position>,
    pub education_level: String,
    pub software_skills: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_checkbox")]
    pub is_project_manager: bool,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub area_id: Option<i32>,
    #[serde(flatten)]
    pub qualification_fields: QualificationFields,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelCreateForm {
    pub first_name: String,
    pub last_name: String,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub qualification: Qualification,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub position: Option<Position>,
    pub education_level: String,
    #[serde(default, deserialize_with = "deserialize_sequence")]
    pub software_skills: Vec<String>,
    #[serde(default)]
    pub is_project_manager: bool,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub area_id: Option<i32>,
    #[serde(flatten)]
    pub qualification_fields: QualificationFields,
}

#[derive(Serialize, Deserialize)]
pub struct TechnicalPersonnelListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub qualification: Option<Qualification>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub position: Option<Position>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub department_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub area_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "technical_personnel/api/list.html")]
pub struct TechnicalPersonnelApiListTemplate {
    pub technical_personnel: Vec<TechnicalPersonnelListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TechnicalPersonnelListItem {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub qualification: Qualification,
    pub position: Option<Position>,
    pub area_id: Option<i32>,
    pub area_name: Option<String>,
}

// Helper structs for database queries
#[derive(FromRow)]
struct TechnicalPersonnelBasicInfo {
    id: i32,
    first_name: String,
    last_name: String,
    middle_name: Option<String>,
    gender: Gender,
    photo: Option<String>,
    phone_number: String,
    salary: i32,
    qualification: Qualification,
    position: Option<Position>,
    education_level: String,
    software_skills: Vec<String>,
    is_project_manager: bool,
}

#[derive(FromRow)]
struct TechnicianData {
    safety_training_level: String,
}

#[derive(FromRow)]
struct TechnologistData {
    management_tools: Vec<String>,
}

#[derive(FromRow)]
struct EngineerData {
    pe_license_id: i32,
}

#[derive(FromRow)]
struct SupervisorInfo {
    department_id: Option<i32>,
    department_name: Option<String>,
    area_id: Option<i32>,
    area_name: Option<String>,
}

// Handler functions for page endpoints

async fn technical_personnel_list_handler(State(_db): State<Database>) -> Html<String> {
    // Return the list page template
    let template = TechnicalPersonnelListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_details_handler(
    State(_db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Return the details page with personnel ID
    let template = TechnicalPersonnelDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_new_handler(State(_db): State<Database>) -> Html<String> {
    // Return the new personnel form template
    let template = TechnicalPersonnelNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get personnel data from database
    let query = sqlx::query_as::<_, TechnicalPersonnelBasicInfo>(
        "SELECT e.id, e.first_name, e.last_name, e.middle_name, e.gender, 
         e.photo, e.phone_number, e.salary, tp.qualification, tp.position, 
         tp.education_level, tp.software_skills, tp.is_project_manager
         FROM employee e
         JOIN technical_personnel tp ON e.id = tp.id
         WHERE e.id = $1"
    )
    .bind(id);

    let personnel = match query.fetch_optional(&*db.pool).await {
        Ok(Some(personnel)) => personnel,
        Ok(None) => {
            return Html::from(format!("<p>Technical personnel with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching technical personnel: {}</p>", e));
        }
    };

    // Render the template with personnel data
    let template = TechnicalPersonnelEditTemplate {
        id: personnel.id,
        first_name: personnel.first_name,
        last_name: personnel.last_name,
        middle_name: personnel.middle_name,
        gender: personnel.gender,
        phone_number: personnel.phone_number,
        salary: personnel.salary,
        qualification: personnel.qualification,
        position: personnel.position,
        education_level: personnel.education_level,
        software_skills: personnel.software_skills,
        is_project_manager: personnel.is_project_manager,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

async fn technical_personnel_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get basic personnel information
    let personnel_query = sqlx::query_as::<_, TechnicalPersonnelBasicInfo>(
        "SELECT e.id, e.first_name, e.last_name, e.middle_name, e.gender, 
         e.photo, e.phone_number, e.salary, tp.qualification, tp.position, 
         tp.education_level, tp.software_skills, tp.is_project_manager
         FROM employee e
         JOIN technical_personnel tp ON e.id = tp.id
         WHERE e.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let personnel = match personnel_query {
        Ok(Some(personnel)) => personnel,
        Ok(None) => {
            return Html::from(format!("<p>Technical personnel with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching technical personnel: {}</p>", e));
        }
    };

    // Get supervisor information and area assignment
    let supervisor_query = sqlx::query(
        "SELECT 
            d.id as department_id, 
            d.name as department_name,
            s_area.id as supervising_area_id, 
            s_area.name as supervising_area_name,
            a.id as area_id,
            a.name as area_name
         FROM technical_personnel tp
         LEFT JOIN department d ON tp.id = d.supervisor_id
         LEFT JOIN area s_area ON tp.id = s_area.supervisor_id
         LEFT JOIN area a ON tp.area_id = a.id
         WHERE tp.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let (supervising_department_id, supervising_department_name, 
         supervising_area_id, supervising_area_name,
         area_id, area_name) = match supervisor_query {
        Ok(Some(row)) => (
            row.get::<Option<i32>, _>("department_id"),
            row.get::<Option<String>, _>("department_name"),
            row.get::<Option<i32>, _>("supervising_area_id"),
            row.get::<Option<String>, _>("supervising_area_name"),
            row.get::<Option<i32>, _>("area_id"),
            row.get::<Option<String>, _>("area_name"),
        ),
        Ok(None) | Err(_) => (None, None, None, None, None, None),
    };

    // Get qualification-specific fields based on qualification
    let qualification_fields = match personnel.qualification {
        Qualification::Technician => {
            let technician_query = sqlx::query_as::<_, TechnicianData>(
                "SELECT safety_training_level FROM technician WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match technician_query {
                Ok(data) => QualificationFields::Technician(TechnicianFields {
                    safety_training_level: data.safety_training_level,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching technician data: {}</p>", e));
                }
            }
        },
        Qualification::Technologist => {
            let technologist_query = sqlx::query_as::<_, TechnologistData>(
                "SELECT management_tools FROM technologist WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match technologist_query {
                Ok(data) => QualificationFields::Technologist(TechnologistFields {
                    management_tools: data.management_tools,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching technologist data: {}</p>", e));
                }
            }
        },
        Qualification::Engineer => {
            let engineer_query = sqlx::query_as::<_, EngineerData>(
                "SELECT pe_license_id FROM engineer WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match engineer_query {
                Ok(data) => QualificationFields::Engineer(EngineerFields {
                    pe_license_id: data.pe_license_id,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching engineer data: {}</p>", e));
                }
            }
        },
    };

    // Construct the complete personnel details
    let template = TechnicalPersonnelApiDetailsTemplate {
        id: personnel.id,
        first_name: personnel.first_name,
        last_name: personnel.last_name,
        middle_name: personnel.middle_name,
        gender: personnel.gender,
        photo: personnel.photo,
        phone_number: personnel.phone_number,
        salary: personnel.salary,
        qualification: personnel.qualification,
        position: personnel.position,
        education_level: personnel.education_level,
        software_skills: personnel.software_skills,
        is_project_manager: personnel.is_project_manager,
        qualification_fields,
        supervising_department_id,
        supervising_department_name,
        supervising_area_id,
        supervising_area_name,
        area_id,
        area_name,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_qualification_fields_handler(
    State(_db): State<Database>,
    Query(query): Query<TechnicalPersonnelQualificationQuery>,
) -> Html<String> {
    let template = TechnicalPersonnelQualificationFieldsTemplate {
        qualification: query.qualification,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<TechnicalPersonnelUpdateForm>,
) -> Html<String> {
    // Begin transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };

    // Update employee table
    let employee_result = sqlx::query(
        "UPDATE employee 
         SET first_name = $1, last_name = $2, middle_name = $3, gender = $4, 
             phone_number = $5, salary = $6 
         WHERE id = $7"
    )
    .bind(&form.first_name)
    .bind(&form.last_name)
    .bind(&form.middle_name)
    .bind(form.gender)
    .bind(&form.phone_number)
    .bind(form.salary)
    .bind(id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = employee_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating employee data: {}</p>", e));
    }

    // Check if qualification has changed
    let current_qualification = sqlx::query_scalar::<_, Qualification>(
        "SELECT qualification FROM technical_personnel WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let current_qualification = match current_qualification {
        Ok(Some(qualification)) => qualification,
        Ok(None) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Technical personnel with ID {} not found</p>", id));
        }
        Err(e) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error fetching current qualification: {}</p>", e));
        }
    };

    // If qualification changed, we need to delete from the old qualification table and insert into the new one
    if current_qualification != form.qualification {
        // Delete from old qualification table
        let delete_result = match current_qualification {
            Qualification::Technician => sqlx::query("DELETE FROM technician WHERE id = $1").bind(id).execute(&mut *tx).await,
            Qualification::Technologist => sqlx::query("DELETE FROM technologist WHERE id = $1").bind(id).execute(&mut *tx).await,
            Qualification::Engineer => sqlx::query("DELETE FROM engineer WHERE id = $1").bind(id).execute(&mut *tx).await,
        };

        if let Err(e) = delete_result {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error removing old qualification data: {}</p>", e));
        }
    }

    // Update technical_personnel table
    let tp_result = sqlx::query(
        "UPDATE technical_personnel 
         SET qualification = $1, position = $2, education_level = $3,
             software_skills = $4, is_project_manager = $5, area_id = $6
         WHERE id = $7"
    )
    .bind(form.qualification)
    .bind(form.position)
    .bind(&form.education_level)
    .bind(&form.software_skills)
    .bind(form.is_project_manager)
    .bind(form.area_id)
    .bind(id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = tp_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating technical personnel data: {}</p>", e));
    }

    // Update or insert qualification-specific fields
    let qualification_result = match &form.qualification_fields {
        QualificationFields::Technician(fields) => {
            if current_qualification == Qualification::Technician {
                sqlx::query(
                    "UPDATE technician SET safety_training_level = $1 WHERE id = $2"
                )
                .bind(&fields.safety_training_level)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO technician (id, safety_training_level) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(&fields.safety_training_level)
                .execute(&mut *tx)
                .await
            }
        },
        QualificationFields::Technologist(fields) => {
            if current_qualification == Qualification::Technologist {
                sqlx::query(
                    "UPDATE technologist SET management_tools = $1 WHERE id = $2"
                )
                .bind(&fields.management_tools)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO technologist (id, management_tools) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(&fields.management_tools)
                .execute(&mut *tx)
                .await
            }
        },
        QualificationFields::Engineer(fields) => {
            if current_qualification == Qualification::Engineer {
                sqlx::query(
                    "UPDATE engineer SET pe_license_id = $1 WHERE id = $2"
                )
                .bind(fields.pe_license_id)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO engineer (id, pe_license_id) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(fields.pe_license_id)
                .execute(&mut *tx)
                .await
            }
        },
    };

    if let Err(e) = qualification_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating qualification-specific data: {}</p>", e));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }

    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Technical personnel {} {} updated successfully", form.first_name, form.last_name)),
        redirect: Some(format!("/technical-personnel/{}", id)),
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn technical_personnel_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if the technical personnel is supervising any department or area
    let is_supervisor = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM department WHERE supervisor_id = $1
            UNION
            SELECT 1 FROM area WHERE supervisor_id = $1
        )"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;

    match is_supervisor {
        Ok(true) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Cannot delete technical personnel: they are supervising a department or area".to_string()),
                redirect: Some(format!("/technical-personnel/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
        Ok(false) => {},
        Err(e) => {
            return Html::from(format!("<p>Error checking if personnel is a supervisor: {}</p>", e));
        }
    }

    // Begin transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };

    // Get personnel's qualification to know which qualification-specific table to delete from
    let qualification = sqlx::query_scalar::<_, Qualification>(
        "SELECT qualification FROM technical_personnel WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    match qualification {
        Ok(Some(qualification)) => {
            // Delete from qualification-specific table
            let delete_qualification = match qualification {
                Qualification::Technician => sqlx::query("DELETE FROM technician WHERE id = $1").bind(id).execute(&mut *tx).await,
                Qualification::Technologist => sqlx::query("DELETE FROM technologist WHERE id = $1").bind(id).execute(&mut *tx).await,
                Qualification::Engineer => sqlx::query("DELETE FROM engineer WHERE id = $1").bind(id).execute(&mut *tx).await,
            };

            if let Err(e) = delete_qualification {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error deleting qualification data: {}</p>", e));
            }

            // Delete from technical_personnel table
            if let Err(e) = sqlx::query("DELETE FROM technical_personnel WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
            {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error deleting technical personnel: {}</p>", e));
            }

            // Delete from employee table
            if let Err(e) = sqlx::query("DELETE FROM employee WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
            {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error deleting employee: {}</p>", e));
            }

            // Commit transaction
            if let Err(e) = tx.commit().await {
                return Html::from(format!("<p>Error committing transaction: {}</p>", e));
            }

            // Return success notification
            let template = NotificationTemplate {
                result: NotificationResult::Success,
                message: Some("Technical personnel deleted successfully".to_string()),
                redirect: Some("/technical-personnel".to_string()),
            };

            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(None) => {
            let _ = tx.rollback().await;
            Html::from(format!("<p>Technical personnel with ID {} not found</p>", id))
        },
        Err(e) => {
            let _ = tx.rollback().await;
            Html::from(format!("<p>Error fetching personnel's qualification: {}</p>", e))
        }
    }
}

async fn technical_personnel_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<TechnicalPersonnelListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT 
            tp.id, 
            e.first_name, 
            e.last_name, 
            tp.qualification,
            tp.position,
            tp.area_id,
            a.name as area_name
        FROM technical_personnel tp
        JOIN employee e ON tp.id = e.id
        LEFT JOIN area a ON tp.area_id = a.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(qualification) = &filter.qualification {
        query_builder.push(" WHERE tp.qualification = ");
        query_builder.push_bind(qualification);
        where_added = true;
    }

    if let Some(position) = &filter.position {
        if where_added {
            query_builder.push(" AND tp.position = ");
        } else {
            query_builder.push(" WHERE tp.position = ");
            where_added = true;
        }
        query_builder.push_bind(position);
    }

    if let Some(department_id) = &filter.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM area WHERE department_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM area WHERE department_id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(" AND id = tp.area_id)");
    }

    if let Some(area_id) = &filter.area_id {
        if where_added {
            query_builder.push(" AND tp.area_id = ");
        } else {
            query_builder.push(" WHERE tp.area_id = ");
            where_added = true;
        }
        query_builder.push_bind(area_id);
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND (e.last_name ILIKE ");
        } else {
            query_builder.push(" WHERE (e.last_name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", name));
        query_builder.push(" OR e.first_name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
        query_builder.push(")");
    }

    // Count total results for pagination - PROPERLY BIND PARAMETERS
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM technical_personnel tp
        JOIN employee e ON tp.id = e.id
        LEFT JOIN area a ON tp.area_id = a.id"
    );
    
    let mut count_where_added = false;
    
    if let Some(qualification) = &filter.qualification {
        count_query_builder.push(" WHERE tp.qualification = ");
        count_query_builder.push_bind(qualification);
        count_where_added = true;
    }

    if let Some(position) = &filter.position {
        if count_where_added {
            count_query_builder.push(" AND tp.position = ");
        } else {
            count_query_builder.push(" WHERE tp.position = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(position);
    }

    if let Some(department_id) = &filter.department_id {
        if count_where_added {
            count_query_builder.push(" AND EXISTS (SELECT 1 FROM area WHERE department_id = ");
        } else {
            count_query_builder.push(" WHERE EXISTS (SELECT 1 FROM area WHERE department_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(department_id);
        count_query_builder.push(" AND id = tp.area_id)");
    }

    if let Some(area_id) = &filter.area_id {
        if count_where_added {
            count_query_builder.push(" AND tp.area_id = ");
        } else {
            count_query_builder.push(" WHERE tp.area_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(area_id);
    }

    if let Some(name) = &filter.name {
        if count_where_added {
            count_query_builder.push(" AND (e.last_name ILIKE ");
        } else {
            count_query_builder.push(" WHERE (e.last_name ILIKE ");
            count_where_added = true;
        }
        count_query_builder.push_bind(format!("%{}%", name));
        count_query_builder.push(" OR e.first_name ILIKE ");
        count_query_builder.push_bind(format!("%{}%", name));
        count_query_builder.push(")");
    }
    
    let count = match count_query_builder.build_query_scalar::<i64>().fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting technical personnel: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("e.last_name, e.first_name"),
        "qualification" => query_builder.push("tp.qualification"),
        "position" => query_builder.push("tp.position"),
        _ => query_builder.push("tp.id"),
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
    let query = query_builder.build_query_as::<TechnicalPersonnelListItem>();
    let personnel = match query.fetch_all(&*db.pool).await {
        Ok(personnel) => personnel,
        Err(e) => return Html::from(format!("<p>Error fetching technical personnel: {}</p>", e)),
    };

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = TechnicalPersonnelApiListTemplate {
        technical_personnel: personnel,
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

async fn technical_personnel_create_handler(
    State(db): State<Database>,
    Form(form): Form<TechnicalPersonnelCreateForm>,
) -> Html<String> {
    // Begin transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };

    // Insert into employee table
    let employee_result = sqlx::query(
        "INSERT INTO employee (class, first_name, last_name, middle_name, gender, phone_number, salary) 
         VALUES ('technical_personnel', $1, $2, $3, $4, $5, $6) 
         RETURNING id"
    )
    .bind(&form.first_name)
    .bind(&form.last_name)
    .bind(&form.middle_name)
    .bind(form.gender)
    .bind(&form.phone_number)
    .bind(form.salary)
    .fetch_one(&mut *tx)
    .await;

    let employee_id = match employee_result {
        Ok(row) => {
            let id: i32 = row.get(0);
            id
        },
        Err(e) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error creating employee: {}</p>", e));
        }
    };

    // Insert into technical_personnel table
    let tp_result = sqlx::query(
        "INSERT INTO technical_personnel (id, qualification, position, education_level, software_skills, is_project_manager, area_id) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(employee_id)
    .bind(form.qualification)
    .bind(form.position)
    .bind(&form.education_level)
    .bind(&form.software_skills)
    .bind(form.is_project_manager)
    .bind(form.area_id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = tp_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error creating technical personnel: {}</p>", e));
    }

    // Insert into qualification-specific table
    let qualification_result = match &form.qualification_fields {
        QualificationFields::Technician(fields) => {
            sqlx::query(
                "INSERT INTO technician (id, safety_training_level) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(&fields.safety_training_level)
            .execute(&mut *tx)
            .await
        },
        QualificationFields::Technologist(fields) => {
            sqlx::query(
                "INSERT INTO technologist (id, management_tools) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(&fields.management_tools)
            .execute(&mut *tx)
            .await
        },
        QualificationFields::Engineer(fields) => {
            sqlx::query(
                "INSERT INTO engineer (id, pe_license_id) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(fields.pe_license_id)
            .execute(&mut *tx)
            .await
        },
    };

    if let Err(e) = qualification_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error creating qualification-specific data: {}</p>", e));
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }

    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Technical personnel {} {} created successfully", form.first_name, form.last_name)),
        redirect: Some(format!("/technical-personnel/{}", employee_id)),
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
        .route("/technical-personnel", get(technical_personnel_list_handler))
        .route("/technical-personnel/{id}", get(technical_personnel_details_handler))
        .route("/technical-personnel/new", get(technical_personnel_new_handler))
        .route("/technical-personnel/{id}/edit", get(technical_personnel_edit_handler))
        // HTMX endpoints
        .route("/api/technical-personnel/{id}", get(technical_personnel_api_details_handler))
        .route("/api/technical-personnel/qualification-fields", get(technical_personnel_qualification_fields_handler))
        .route("/api/technical-personnel/{id}", put(technical_personnel_update_handler))
        .route("/api/technical-personnel/{id}", delete(technical_personnel_delete_handler))
        .route("/api/technical-personnel", get(technical_personnel_list_api_handler))
        .route("/api/technical-personnel", post(technical_personnel_create_handler))
}
