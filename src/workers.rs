use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Profession, Gender}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};

// Profession-specific fields
#[derive(Serialize, Deserialize)]
#[serde(tag = "profession")]
pub enum ProfessionFields {
    Electrician(ElectricianFields),
    Plumber(PlumberFields),
    Welder(WelderFields),
    Driver(DriverFields),
    Mason(MasonFields),
}

// Worker profession-specific field structs
#[derive(Serialize, Deserialize)]
pub struct ElectricianFields {
    pub voltage_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlumberFields {
    pub pipe_specialization: String,
}

#[derive(Serialize, Deserialize)]
pub struct WelderFields {
    pub welding_machine: String,
}

#[derive(Serialize, Deserialize)]
pub struct DriverFields {
    pub vehicle_type: String,
    pub number_of_accidents: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MasonFields {
    pub hq_restoration_skills: bool,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "workers/list.html")]
pub struct WorkersListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/details.html")]
pub struct WorkerDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "workers/new.html")]
pub struct WorkerNewTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/edit.html")]
pub struct WorkerEditTemplate {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i32>,
    pub brigade_name: Option<String>,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct WorkerProfessionQuery {
    pub profession: Profession,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/details.html")]
pub struct WorkerApiDetailsTemplate {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub photo: Option<String>,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i32>,
    pub brigade_name: Option<String>,
    pub is_brigadier: bool,
    pub profession_fields: ProfessionFields,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/profession-fields.html")]
pub struct WorkerProfessionFieldsTemplate {
    pub profession: Profession,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerUpdateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i32>,
    #[serde(flatten)]
    pub profession_fields: ProfessionFields,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerCreateForm {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub gender: Gender,
    pub phone_number: String,
    pub salary: i32,
    pub profession: Profession,
    pub union_name: Option<String>,
    pub brigade_id: Option<i32>,
    #[serde(flatten)]
    pub profession_fields: ProfessionFields,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    pub profession: Option<Profession>,
    pub brigade_id: Option<i32>,
    pub is_brigadier: Option<bool>,
    pub name: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "workers/api/list.html")]
pub struct WorkerListTemplate {
    pub workers: Vec<WorkerListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct WorkerListItem {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub profession: Profession,
    pub brigade_id: Option<i32>,
    pub brigade_name: Option<String>,
    pub is_brigadier: bool,
}

// Helper structs for database queries
#[derive(FromRow)]
struct WorkerBasicInfo {
    id: i32,
    first_name: String,
    last_name: String,
    middle_name: Option<String>,
    gender: Gender,
    photo: Option<String>,
    phone_number: String,
    salary: i32,
    profession: Profession,
    union_name: Option<String>,
}

#[derive(FromRow)]
struct ElectricianData {
    voltage_specialization: String,
}

#[derive(FromRow)]
struct PlumberData {
    pipe_specialization: String,
}

#[derive(FromRow)]
struct WelderData {
    welding_machine: String,
}

#[derive(FromRow)]
struct DriverData {
    vehicle_type: String,
    number_of_accidents: i32,
}

#[derive(FromRow)]
struct MasonData {
    hq_restoration_skills: bool,
}

#[derive(FromRow)]
struct BrigadeInfo {
    brigade_id: Option<i32>,
    brigade_name: Option<String>,
    is_brigadier: bool,
}

// Handler functions for page endpoints

async fn workers_list_handler(State(_db): State<Database>) -> Html<String> {
    let template = WorkersListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_details_handler(
    State(_db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    let template = WorkerDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_new_handler(State(_db): State<Database>) -> Html<String> {
    let template = WorkerNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get worker data from database
    let query = sqlx::query_as::<_, WorkerBasicInfo>(
        "SELECT e.id, e.first_name, e.last_name, e.middle_name, e.gender, 
         e.photo, e.phone_number, e.salary, w.profession, w.union_name
         FROM employee e
         JOIN worker w ON e.id = w.id
         WHERE e.id = $1"
    )
    .bind(id);

    let worker = match query.fetch_optional(&*db.pool).await {
        Ok(Some(worker)) => worker,
        Ok(None) => {
            return Html::from(format!("<p>Worker with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching worker: {}</p>", e));
        }
    };

    // Get brigade information
    let brigade_query = sqlx::query_as::<_, BrigadeInfo>(
        "SELECT 
            a.brigade_id,
            CONCAT(be.last_name, ' ', be.first_name) as brigade_name,
            CASE WHEN b.brigadier_id = $1 THEN true ELSE false END as is_brigadier
        FROM worker w
        LEFT JOIN assignment a ON w.id = a.worker_id
        LEFT JOIN brigade b ON a.brigade_id = b.id
        LEFT JOIN worker bw ON b.brigadier_id = bw.id
        LEFT JOIN employee be ON bw.id = be.id
        WHERE w.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let brigade_info = match brigade_query {
        Ok(Some(info)) => info,
        Ok(None) => BrigadeInfo {
            brigade_id: None,
            brigade_name: None,
            is_brigadier: false,
        },
        Err(e) => {
            return Html::from(format!("<p>Error fetching brigade information: {}</p>", e));
        }
    };

    // Render the template with worker data
    let template = WorkerEditTemplate {
        id: worker.id,
        first_name: worker.first_name,
        last_name: worker.last_name,
        middle_name: worker.middle_name,
        gender: worker.gender,
        phone_number: worker.phone_number,
        salary: worker.salary,
        profession: worker.profession,
        union_name: worker.union_name,
        brigade_id: brigade_info.brigade_id,
        brigade_name: brigade_info.brigade_name,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

async fn worker_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Get basic worker information
    let worker_query = sqlx::query_as::<_, WorkerBasicInfo>(
        "SELECT e.id, e.first_name, e.last_name, e.middle_name, e.gender, 
         e.photo, e.phone_number, e.salary, w.profession, w.union_name
         FROM employee e
         JOIN worker w ON e.id = w.id
         WHERE e.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let worker = match worker_query {
        Ok(Some(worker)) => worker,
        Ok(None) => {
            return Html::from(format!("<p>Worker with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching worker: {}</p>", e));
        }
    };

    // Get brigade information
    let brigade_query = sqlx::query_as::<_, BrigadeInfo>(
        "SELECT 
            a.brigade_id,
            CONCAT(be.last_name, ' ', be.first_name) as brigade_name,
            CASE WHEN b.brigadier_id = $1 THEN true ELSE false END as is_brigadier
        FROM worker w
        LEFT JOIN assignment a ON w.id = a.worker_id
        LEFT JOIN brigade b ON a.brigade_id = b.id
        LEFT JOIN worker bw ON b.brigadier_id = bw.id
        LEFT JOIN employee be ON bw.id = be.id
        WHERE w.id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let brigade_info = match brigade_query {
        Ok(Some(info)) => info,
        Ok(None) => BrigadeInfo {
            brigade_id: None,
            brigade_name: None,
            is_brigadier: false,
        },
        Err(e) => {
            return Html::from(format!("<p>Error fetching brigade information: {}</p>", e));
        }
    };

    // Get profession-specific fields based on profession
    let profession_fields = match worker.profession {
        Profession::Electrician => {
            let electrician_query = sqlx::query_as::<_, ElectricianData>(
                "SELECT voltage_specialization FROM electrician WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match electrician_query {
                Ok(data) => ProfessionFields::Electrician(ElectricianFields {
                    voltage_specialization: data.voltage_specialization,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching electrician data: {}</p>", e));
                }
            }
        },
        Profession::Plumber => {
            let plumber_query = sqlx::query_as::<_, PlumberData>(
                "SELECT pipe_specialization FROM plumber WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match plumber_query {
                Ok(data) => ProfessionFields::Plumber(PlumberFields {
                    pipe_specialization: data.pipe_specialization,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching plumber data: {}</p>", e));
                }
            }
        },
        Profession::Welder => {
            let welder_query = sqlx::query_as::<_, WelderData>(
                "SELECT welding_machine FROM welder WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match welder_query {
                Ok(data) => ProfessionFields::Welder(WelderFields {
                    welding_machine: data.welding_machine,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching welder data: {}</p>", e));
                }
            }
        },
        Profession::Driver => {
            let driver_query = sqlx::query_as::<_, DriverData>(
                "SELECT vehicle_type, number_of_accidents FROM driver WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match driver_query {
                Ok(data) => ProfessionFields::Driver(DriverFields {
                    vehicle_type: data.vehicle_type,
                    number_of_accidents: data.number_of_accidents,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching driver data: {}</p>", e));
                }
            }
        },
        Profession::Mason => {
            let mason_query = sqlx::query_as::<_, MasonData>(
                "SELECT hq_restoration_skills FROM mason WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&*db.pool)
            .await;

            match mason_query {
                Ok(data) => ProfessionFields::Mason(MasonFields {
                    hq_restoration_skills: data.hq_restoration_skills,
                }),
                Err(e) => {
                    return Html::from(format!("<p>Error fetching mason data: {}</p>", e));
                }
            }
        },
    };

    // Construct the complete worker details
    let template = WorkerApiDetailsTemplate {
        id: worker.id,
        first_name: worker.first_name,
        last_name: worker.last_name,
        middle_name: worker.middle_name,
        gender: worker.gender,
        photo: worker.photo,
        phone_number: worker.phone_number,
        salary: worker.salary,
        profession: worker.profession,
        union_name: worker.union_name,
        brigade_id: brigade_info.brigade_id,
        brigade_name: brigade_info.brigade_name,
        is_brigadier: brigade_info.is_brigadier,
        profession_fields,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_profession_fields_handler(
    State(_db): State<Database>,
    Query(query): Query<WorkerProfessionQuery>,
) -> Html<String> {
    let template = WorkerProfessionFieldsTemplate {
        profession: query.profession,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<WorkerUpdateForm>,
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

    // Check if profession has changed
    let current_profession = sqlx::query_scalar::<_, Profession>("SELECT profession FROM worker WHERE id = $1")
        .bind(id)
        .fetch_optional(&*db.pool)
        .await;

    let current_profession = match current_profession {
        Ok(Some(profession)) => profession,
        Ok(None) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Worker with ID {} not found</p>", id));
        }
        Err(e) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error fetching current profession: {}</p>", e));
        }
    };

    // If profession changed, we need to delete from the old profession table and insert into the new one
    if current_profession != form.profession {
        // Delete from old profession table
        let delete_result = match current_profession {
            Profession::Electrician => sqlx::query("DELETE FROM electrician WHERE id = $1").bind(id).execute(&mut *tx).await,
            Profession::Plumber => sqlx::query("DELETE FROM plumber WHERE id = $1").bind(id).execute(&mut *tx).await,
            Profession::Welder => sqlx::query("DELETE FROM welder WHERE id = $1").bind(id).execute(&mut *tx).await,
            Profession::Driver => sqlx::query("DELETE FROM driver WHERE id = $1").bind(id).execute(&mut *tx).await,
            Profession::Mason => sqlx::query("DELETE FROM mason WHERE id = $1").bind(id).execute(&mut *tx).await,
        };

        if let Err(e) = delete_result {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error removing old profession data: {}</p>", e));
        }
    }

    // Update worker table
    let worker_result = sqlx::query(
        "UPDATE worker 
         SET profession = $1, union_name = $2
         WHERE id = $3"
    )
    .bind(form.profession)
    .bind(&form.union_name)
    .bind(id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = worker_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating worker data: {}</p>", e));
    }

    // Update or insert profession-specific fields
    let profession_result = match &form.profession_fields {
        ProfessionFields::Electrician(fields) => {
            if current_profession == Profession::Electrician {
                sqlx::query(
                    "UPDATE electrician SET voltage_specialization = $1 WHERE id = $2"
                )
                .bind(&fields.voltage_specialization)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO electrician (id, voltage_specialization) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(&fields.voltage_specialization)
                .execute(&mut *tx)
                .await
            }
        },
        ProfessionFields::Plumber(fields) => {
            if current_profession == Profession::Plumber {
                sqlx::query(
                    "UPDATE plumber SET pipe_specialization = $1 WHERE id = $2"
                )
                .bind(&fields.pipe_specialization)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO plumber (id, pipe_specialization) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(&fields.pipe_specialization)
                .execute(&mut *tx)
                .await
            }
        },
        ProfessionFields::Welder(fields) => {
            if current_profession == Profession::Welder {
                sqlx::query(
                    "UPDATE welder SET welding_machine = $1 WHERE id = $2"
                )
                .bind(&fields.welding_machine)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO welder (id, welding_machine) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(&fields.welding_machine)
                .execute(&mut *tx)
                .await
            }
        },
        ProfessionFields::Driver(fields) => {
            if current_profession == Profession::Driver {
                sqlx::query(
                    "UPDATE driver SET vehicle_type = $1, number_of_accidents = $2 WHERE id = $3"
                )
                .bind(&fields.vehicle_type)
                .bind(fields.number_of_accidents)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO driver (id, vehicle_type, number_of_accidents) VALUES ($1, $2, $3)"
                )
                .bind(id)
                .bind(&fields.vehicle_type)
                .bind(fields.number_of_accidents)
                .execute(&mut *tx)
                .await
            }
        },
        ProfessionFields::Mason(fields) => {
            if current_profession == Profession::Mason {
                sqlx::query(
                    "UPDATE mason SET hq_restoration_skills = $1 WHERE id = $2"
                )
                .bind(fields.hq_restoration_skills)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO mason (id, hq_restoration_skills) VALUES ($1, $2)"
                )
                .bind(id)
                .bind(fields.hq_restoration_skills)
                .execute(&mut *tx)
                .await
            }
        },
    };

    if let Err(e) = profession_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating profession-specific data: {}</p>", e));
    }

    // Handle brigade assignment if needed
    // First check if worker is already in a brigade or is a brigadier
    let current_assignment = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT brigade_id FROM assignment WHERE worker_id = $1"
    )
    .bind(id)
    .fetch_optional(&*db.pool)
    .await;

    let is_brigadier = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM brigade WHERE brigadier_id = $1)"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;

    match (current_assignment, is_brigadier) {
        (Ok(current_brigade), Ok(is_brigadier_value)) => {
            // If worker is a brigadier, they can't be assigned to a different brigade
            if is_brigadier_value && form.brigade_id.is_some() && form.brigade_id != current_brigade.flatten() {
                let _ = tx.rollback().await;
                return Html::from("<p>Error: Brigadier cannot be assigned to a different brigade</p>".to_string());
            }

            // Handle brigade assignment changes
            match (current_brigade, form.brigade_id) {
                (Some(current), Some(new)) if current != Some(new) => {
                    // Remove from current brigade
                    if let Err(e) = sqlx::query("DELETE FROM assignment WHERE worker_id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await 
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error removing worker from current brigade: {}</p>", e));
                    }

                    // Add to new brigade
                    if let Err(e) = sqlx::query("INSERT INTO assignment (worker_id, brigade_id) VALUES ($1, $2)")
                        .bind(id)
                        .bind(new)
                        .execute(&mut *tx)
                        .await 
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error adding worker to new brigade: {}</p>", e));
                    }
                },
                (None, Some(new)) => {
                    // Add to a brigade for the first time
                    if let Err(e) = sqlx::query("INSERT INTO assignment (worker_id, brigade_id) VALUES ($1, $2)")
                        .bind(id)
                        .bind(new)
                        .execute(&mut *tx)
                        .await 
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error adding worker to brigade: {}</p>", e));
                    }
                },
                (Some(_), None) => {
                    // Remove from current brigade
                    if let Err(e) = sqlx::query("DELETE FROM assignment WHERE worker_id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await 
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error removing worker from brigade: {}</p>", e));
                    }
                },
                _ => {}, // No change in brigade assignment
            }
        },
        (Err(e), _) | (_, Err(e)) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error checking worker's brigade status: {}</p>", e));
        }
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }

    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Worker {} {} updated successfully", form.first_name, form.last_name)),
        redirect: Some(format!("/workers/{}", id)),
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn worker_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if worker is a brigadier
    let is_brigadier = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM brigade WHERE brigadier_id = $1)"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;

    match is_brigadier {
        Ok(true) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Cannot delete worker: Worker is a brigadier. Remove brigade first.".to_string()),
                redirect: Some(format!("/workers/{}", id)),
            };
            return match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            };
        }
        Ok(false) => {},
        Err(e) => {
            return Html::from(format!("<p>Error checking if worker is a brigadier: {}</p>", e));
        }
    }

    // Begin transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };

    // Get worker's profession to know which profession-specific table to delete from
    let profession = sqlx::query_scalar::<_, Profession>("SELECT profession FROM worker WHERE id = $1")
        .bind(id)
        .fetch_optional(&*db.pool)
        .await;

    match profession {
        Ok(Some(profession)) => {
            // Remove from assignment if any
            if let Err(e) = sqlx::query("DELETE FROM assignment WHERE worker_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
            {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error removing worker from assignments: {}</p>", e));
            }

            // Delete from profession-specific table
            let delete_profession = match profession {
                Profession::Electrician => sqlx::query("DELETE FROM electrician WHERE id = $1").bind(id).execute(&mut *tx).await,
                Profession::Plumber => sqlx::query("DELETE FROM plumber WHERE id = $1").bind(id).execute(&mut *tx).await,
                Profession::Welder => sqlx::query("DELETE FROM welder WHERE id = $1").bind(id).execute(&mut *tx).await,
                Profession::Driver => sqlx::query("DELETE FROM driver WHERE id = $1").bind(id).execute(&mut *tx).await,
                Profession::Mason => sqlx::query("DELETE FROM mason WHERE id = $1").bind(id).execute(&mut *tx).await,
            };

            if let Err(e) = delete_profession {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error deleting profession data: {}</p>", e));
            }

            // Delete from worker table
            if let Err(e) = sqlx::query("DELETE FROM worker WHERE id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await
            {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error deleting worker: {}</p>", e));
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
                message: Some("Worker deleted successfully".to_string()),
                redirect: Some("/workers".to_string()),
            };

            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(None) => {
            let _ = tx.rollback().await;
            Html::from(format!("<p>Worker with ID {} not found</p>", id))
        },
        Err(e) => {
            let _ = tx.rollback().await;
            Html::from(format!("<p>Error fetching worker's profession: {}</p>", e))
        }
    }
}

async fn workers_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<WorkerListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT 
            w.id, 
            e.first_name, 
            e.last_name, 
            w.profession,
            a.brigade_id,
            CASE 
                WHEN a.brigade_id IS NOT NULL THEN 
                    (SELECT CONCAT(be.last_name, ' ', be.first_name) 
                     FROM brigade b
                     JOIN worker bw ON b.brigadier_id = bw.id
                     JOIN employee be ON bw.id = be.id
                     WHERE b.id = a.brigade_id)
                ELSE NULL
            END AS brigade_name,
            CASE WHEN EXISTS (SELECT 1 FROM brigade b WHERE b.brigadier_id = w.id) THEN true ELSE false END as is_brigadier
        FROM worker w
        JOIN employee e ON w.id = e.id
        LEFT JOIN assignment a ON w.id = a.worker_id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(profession) = &filter.profession {
        query_builder.push(" WHERE w.profession = ");
        query_builder.push_bind(profession);
        where_added = true;
    }

    if let Some(brigade_id) = &filter.brigade_id {
        if where_added {
            query_builder.push(" AND a.brigade_id = ");
        } else {
            query_builder.push(" WHERE a.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
    }

    if let Some(is_brigadier) = &filter.is_brigadier {
        let subquery = if *is_brigadier {
            "EXISTS (SELECT 1 FROM brigade b WHERE b.brigadier_id = w.id)"
        } else {
            "NOT EXISTS (SELECT 1 FROM brigade b WHERE b.brigadier_id = w.id)"
        };

        if where_added {
            query_builder.push(" AND ");
        } else {
            query_builder.push(" WHERE ");
            where_added = true;
        }
        query_builder.push(subquery);
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

    // Count total results for pagination
    let count_query = format!("SELECT COUNT(*) FROM ({}) as count_query", query_builder.sql());
    let count = match sqlx::query_scalar::<_, i64>(&count_query).fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting workers: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("e.last_name, e.first_name"),
        "profession" => query_builder.push("w.profession"),
        "is_brigadier" => query_builder.push("is_brigadier DESC"),
        _ => query_builder.push("w.id"),
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
    let query = query_builder.build_query_as::<WorkerListItem>();
    let workers = match query.fetch_all(&*db.pool).await {
        Ok(workers) => workers,
        Err(e) => return Html::from(format!("<p>Error fetching workers: {}</p>", e)),
    };

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = WorkerListTemplate {
        workers,
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

async fn worker_create_handler(
    State(db): State<Database>,
    Form(form): Form<WorkerCreateForm>,
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
         VALUES ('worker', $1, $2, $3, $4, $5, $6) 
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

    // Insert into worker table
    let worker_result = sqlx::query(
        "INSERT INTO worker (id, profession, union_name) 
         VALUES ($1, $2, $3)"
    )
    .bind(employee_id)
    .bind(form.profession)
    .bind(&form.union_name)
    .execute(&mut *tx)
    .await;

    if let Err(e) = worker_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error creating worker: {}</p>", e));
    }

    // Insert into profession-specific table
    let profession_result = match &form.profession_fields {
        ProfessionFields::Electrician(fields) => {
            sqlx::query(
                "INSERT INTO electrician (id, voltage_specialization) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(&fields.voltage_specialization)
            .execute(&mut *tx)
            .await
        },
        ProfessionFields::Plumber(fields) => {
            sqlx::query(
                "INSERT INTO plumber (id, pipe_specialization) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(&fields.pipe_specialization)
            .execute(&mut *tx)
            .await
        },
        ProfessionFields::Welder(fields) => {
            sqlx::query(
                "INSERT INTO welder (id, welding_machine) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(&fields.welding_machine)
            .execute(&mut *tx)
            .await
        },
        ProfessionFields::Driver(fields) => {
            sqlx::query(
                "INSERT INTO driver (id, vehicle_type, number_of_accidents) VALUES ($1, $2, $3)"
            )
            .bind(employee_id)
            .bind(&fields.vehicle_type)
            .bind(fields.number_of_accidents)
            .execute(&mut *tx)
            .await
        },
        ProfessionFields::Mason(fields) => {
            sqlx::query(
                "INSERT INTO mason (id, hq_restoration_skills) VALUES ($1, $2)"
            )
            .bind(employee_id)
            .bind(fields.hq_restoration_skills)
            .execute(&mut *tx)
            .await
        },
    };

    if let Err(e) = profession_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error creating profession-specific data: {}</p>", e));
    }

    // Handle brigade assignment if needed
    if let Some(brigade_id) = form.brigade_id {
        // Check if brigade exists
        let brigade_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM brigade WHERE id = $1)")
            .bind(brigade_id)
            .fetch_one(&*db.pool)
            .await;

        match brigade_exists {
            Ok(true) => {
                // Assign worker to brigade
                let assign_result = sqlx::query(
                    "INSERT INTO assignment (worker_id, brigade_id) VALUES ($1, $2)"
                )
                .bind(employee_id)
                .bind(brigade_id)
                .execute(&mut *tx)
                .await;

                if let Err(e) = assign_result {
                    let _ = tx.rollback().await;
                    return Html::from(format!("<p>Error assigning worker to brigade: {}</p>", e));
                }
            },
            Ok(false) => {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Brigade with ID {} does not exist</p>", brigade_id));
            },
            Err(e) => {
                let _ = tx.rollback().await;
                return Html::from(format!("<p>Error checking brigade existence: {}</p>", e));
            }
        }
    }

    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }

    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Worker {} {} created successfully", form.first_name, form.last_name)),
        redirect: Some(format!("/workers/{}", employee_id)),
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
        .route("/workers", get(workers_list_handler))
        .route("/workers/{id}", get(worker_details_handler))
        .route("/workers/new", get(worker_new_handler))
        .route("/workers/{id}/edit", get(worker_edit_handler))
        // HTMX endpoints
        .route("/api/workers/{id}", get(worker_api_details_handler))
        .route("/api/workers/profession-fields", get(worker_profession_fields_handler))
        .route("/api/workers/{id}", put(worker_update_handler))
        .route("/api/workers/{id}", delete(worker_delete_handler))
        .route("/api/workers", get(workers_list_api_handler))
        .route("/api/workers", post(worker_create_handler))
}