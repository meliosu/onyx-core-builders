use chrono::{Date, NaiveDate, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::Html,
    routing::{get, put, delete, post},
};
use askama::Template;
use sqlx::{FromRow, Row};

use crate::{database::Database, general::{Qualification, SiteType, RiskLevel, Position}};
use crate::general::{Pagination, Sort, SortDirection, QueryInfo, NotificationResult, NotificationTemplate};
use crate::utils::empty_string_as_none;
use crate::utils::deserialize_from_str;

// Tab selector for site details
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SiteTab {
    Schedule,
    Materials,
    Equipment,
    Brigades,
    Reports,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SiteFields {
    PowerPlant(PowerPlantFields),
    Road(RoadFields),
    Housing(HousingFields),
    Bridge(BridgeFields),
    Park(ParkFields),
}

// Site type-specific field structs
#[derive(Serialize, Deserialize)]
pub struct PowerPlantFields {
    pub energy_output: f32,
    pub energy_source: String,
    pub is_grid_connected: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RoadFields {
    pub length: f32,
    pub lanes: i32,
    pub surface: String,
}

#[derive(Serialize, Deserialize)]
pub struct HousingFields {
    pub number_of_floors: i32,
    pub number_of_entrances: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub energy_efficiency: String,
}

#[derive(Serialize, Deserialize)]
pub struct BridgeFields {
    pub length: f32,
    pub road_material: String,
    pub max_load: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ParkFields {
    pub area: f32,
    pub has_playground: bool,
    pub has_lighting: bool,
}

// Types for page endpoints

#[derive(Template)]
#[template(path = "sites/list.html")]
pub struct SitesListTemplate;

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/details.html")]
pub struct SiteDetailsTemplate {
    pub id: i32,
}

#[derive(Template)]
#[template(path = "sites/new.html")]
pub struct SiteNewTemplate;

#[derive(Template)]
#[template(path = "sites/edit.html")]
pub struct SiteEditTemplate {
    pub id: i32,
    pub name: String,
    pub type_: SiteType,
    pub area_id: i32,
    pub area_name: String,
    pub client_id: i32,
    pub client_name: String,
    pub location: sqlx::postgres::types::PgPoint,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
}

// Types for HTMX endpoints

#[derive(Serialize, Deserialize)]
pub struct SiteTabQuery {
    pub tab: SiteTab,
}

#[derive(Serialize, Deserialize)]
pub struct SiteTypeFieldsQuery {
    #[serde(rename = "type")]
    pub type_: SiteType,
}

#[derive(Template)]
#[template(path = "sites/api/details.html")]
pub struct SiteApiDetailsTemplate {
    pub id: i32,
    pub name: String,
    pub type_: SiteType,
    pub area_id: i32,
    pub area_name: String,
    pub client_id: i32,
    pub client_name: String,
    pub location: sqlx::postgres::types::PgPoint,
    pub risk_level: RiskLevel,
    pub description: Option<String>,
    pub tab: SiteTab,
    pub type_fields: SiteFields,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/type-fields.html")]
pub struct SiteTypeFieldsTemplate {
    #[serde(rename = "type")]
    pub type_: SiteType,
}

#[derive(Deserialize)]
pub struct SiteUpdateForm {
    pub name: String,
    pub area_id: i32,
    pub client_id: i32,
    #[serde(rename = "type")]
    pub type_: SiteType,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub location: sqlx::postgres::types::PgPoint,
    pub risk_level: RiskLevel,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub type_fields: SiteFields,
}

#[derive(Serialize, Deserialize)]
pub struct SiteCreateForm {
    pub name: String,
    pub area_id: i32,
    pub client_id: i32,
    #[serde(rename = "type")]
    pub type_: SiteType,
    pub location: String,
    pub risk_level: RiskLevel,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub type_fields: SiteFields,
}

#[derive(Serialize, Deserialize)]
pub struct SiteListFilter {
    #[serde(flatten)]
    pub sort: Sort,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub area_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub department_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub client_id: Option<i32>,
    #[serde(rename = "type")]
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub type_: Option<SiteType>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub status: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/list.html")]
pub struct SiteListTemplate {
    pub sites: Vec<SiteListItem>,
    pub pagination: Pagination,
    pub query_info: QueryInfo,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct SiteListItem {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: SiteType,
    pub area_id: i32,
    pub area_name: String,
    pub department_id: i32,
    pub department_name: String,
    pub client_id: i32,
    pub client_name: String,
    pub status: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/schedule.html")]
pub struct SiteScheduleTemplate {
    pub id: i32,
    pub tasks: Vec<TaskListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct TaskListItem {
    pub id: i32,
    pub name: String,
    pub brigade_id: Option<i32>,
    pub brigadier_name: Option<String>,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub status: String,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/materials.html")]
pub struct SiteMaterialsTemplate {
    pub id: i32,
    pub materials: Vec<MaterialListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct MaterialListItem {
    pub id: i32,
    pub name: String,
    pub expected_amount: f32,
    pub actual_amount: Option<f32>,
    pub units: String,
    pub cost: f32,
    pub total_cost: f32,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/equipment.html")]
pub struct SiteEquipmentTemplate {
    pub id: i32,
    pub equipment: Vec<EquipmentListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct EquipmentListItem {
    pub id: i32,
    pub name: String,
    pub amount: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/brigades.html")]
pub struct SiteBrigadesTemplate {
    pub id: i32,
    pub brigades: Vec<BrigadeListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BrigadeListItem {
    pub id: i32,
    pub brigadier_id: Option<i32>,
    pub brigadier_name: Option<String>,
    pub worker_count: i64,
    pub current_task: Option<String>,
}

#[derive(Template, Serialize, Deserialize)]
#[template(path = "sites/api/reports.html")]
pub struct SiteReportsTemplate {
    pub id: i32,
    pub reports: Vec<ReportListItem>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReportListItem {
    pub task_id: i32,
    pub task_name: String,
    pub period_start: NaiveDate,
    pub expected_period_end: NaiveDate,
    pub actual_period_end: Option<NaiveDate>,
    pub delay: i32,
}

// Helper structs for database operations

#[derive(FromRow)]
struct SiteBasicInfo {
    id: i32,
    name: String,
    area_id: i32,
    area_name: String,
    client_id: i32,
    client_name: String,
    #[sqlx(rename = "type")]
    type_: SiteType,
    location: sqlx::postgres::types::PgPoint,
    risk_level: RiskLevel,
    description: Option<String>,
}

#[derive(FromRow)]
struct PowerPlantData {
    energy_output: f32,
    energy_source: String,
    is_grid_connected: bool,
}

#[derive(FromRow)]
struct RoadData {
    length: f32,
    lanes: i32,
    surface: String,
}

#[derive(FromRow)]
struct HousingData {
    number_of_floors: i32,
    number_of_entrances: i32,
    #[sqlx(rename = "type")]
    type_: String,
    energy_efficiency: String,
}

#[derive(FromRow)]
struct BridgeData {
    length: f32,
    road_material: String,
    max_load: f32,
}

#[derive(FromRow)]
struct ParkData {
    area: f32,
    has_playground: bool,
    has_lighting: bool,
}

// Handler functions for page endpoints

async fn sites_list_handler(State(_db): State<Database>) -> Html<String> {
    let template = SitesListTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_details_handler(
    State(_db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    let template = SiteDetailsTemplate { id };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_new_handler(State(_db): State<Database>) -> Html<String> {
    let template = SiteNewTemplate;
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_edit_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Fetch basic site information
    let query = sqlx::query_as::<_, SiteBasicInfo>(
        "SELECT s.id, s.name, s.area_id, a.name as area_name, 
         s.client_id, c.name as client_name, s.type, s.location, 
         s.risk_level, s.description
         FROM site s
         JOIN area a ON s.area_id = a.id
         JOIN client c ON s.client_id = c.id
         WHERE s.id = $1"
    )
    .bind(id);
    
    let site = match query.fetch_optional(&*db.pool).await {
        Ok(Some(site)) => site,
        Ok(None) => {
            return Html::from(format!("<p>Site with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching site: {}</p>", e));
        }
    };
    
    let template = SiteEditTemplate {
        id: site.id,
        name: site.name,
        type_: site.type_,
        area_id: site.area_id,
        area_name: site.area_name,
        client_id: site.client_id,
        client_name: site.client_name,
        location: site.location,
        risk_level: site.risk_level,
        description: site.description,
    };

    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

// Handler functions for HTMX endpoints

async fn site_api_details_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(query): Query<SiteTabQuery>,
) -> Html<String> {
    // Fetch basic site information
    let site_query = sqlx::query_as::<_, SiteBasicInfo>(
        "SELECT s.id, s.name, s.area_id, a.name as area_name, 
         s.client_id, c.name as client_name, s.type, s.location, 
         s.risk_level, s.description
         FROM site s
         JOIN area a ON s.area_id = a.id
         JOIN client c ON s.client_id = c.id
         WHERE s.id = $1"
    )
    .bind(id);
    
    let site = match site_query.fetch_optional(&*db.pool).await {
        Ok(Some(site)) => site,
        Ok(None) => {
            return Html::from(format!("<p>Site with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching site details: {}</p>", e));
        }
    };
    
    // Fetch type-specific details based on site type
    let type_fields = match site.type_ {
        SiteType::PowerPlant => {
            let power_plant_query = sqlx::query_as::<_, PowerPlantData>(
                "SELECT energy_output, energy_source, is_grid_connected 
                 FROM power_plant 
                 WHERE site_id = $1"
            )
            .bind(id)
            .fetch_optional(&*db.pool)
            .await;
            
            match power_plant_query {
                Ok(Some(data)) => SiteFields::PowerPlant(PowerPlantFields {
                    energy_output: data.energy_output,
                    energy_source: data.energy_source,
                    is_grid_connected: data.is_grid_connected,
                }),
                Ok(None) => {
                    return Html::from(format!("<p>Power plant data for site ID {} not found</p>", id));
                }
                Err(e) => {
                    return Html::from(format!("<p>Error fetching power plant data: {}</p>", e));
                }
            }
        },
        SiteType::Road => {
            let road_query = sqlx::query_as::<_, RoadData>(
                "SELECT length, lanes, surface 
                 FROM road 
                 WHERE site_id = $1"
            )
            .bind(id)
            .fetch_optional(&*db.pool)
            .await;
            
            match road_query {
                Ok(Some(data)) => SiteFields::Road(RoadFields {
                    length: data.length,
                    lanes: data.lanes,
                    surface: data.surface,
                }),
                Ok(None) => {
                    return Html::from(format!("<p>Road data for site ID {} not found</p>", id));
                }
                Err(e) => {
                    return Html::from(format!("<p>Error fetching road data: {}</p>", e));
                }
            }
        },
        SiteType::Housing => {
            let housing_query = sqlx::query_as::<_, HousingData>(
                "SELECT number_of_floors, number_of_entrances, type, energy_efficiency 
                 FROM housing 
                 WHERE site_id = $1"
            )
            .bind(id)
            .fetch_optional(&*db.pool)
            .await;
            
            match housing_query {
                Ok(Some(data)) => SiteFields::Housing(HousingFields {
                    number_of_floors: data.number_of_floors,
                    number_of_entrances: data.number_of_entrances,
                    type_: data.type_,
                    energy_efficiency: data.energy_efficiency,
                }),
                Ok(None) => {
                    return Html::from(format!("<p>Housing data for site ID {} not found</p>", id));
                }
                Err(e) => {
                    return Html::from(format!("<p>Error fetching housing data: {}</p>", e));
                }
            }
        },
        SiteType::Bridge => {
            let bridge_query = sqlx::query_as::<_, BridgeData>(
                "SELECT length, road_material, max_load 
                 FROM bridge 
                 WHERE site_id = $1"
            )
            .bind(id)
            .fetch_optional(&*db.pool)
            .await;
            
            match bridge_query {
                Ok(Some(data)) => SiteFields::Bridge(BridgeFields {
                    length: data.length,
                    road_material: data.road_material,
                    max_load: data.max_load,
                }),
                Ok(None) => {
                    return Html::from(format!("<p>Bridge data for site ID {} not found</p>", id));
                }
                Err(e) => {
                    return Html::from(format!("<p>Error fetching bridge data: {}</p>", e));
                }
            }
        },
        SiteType::Park => {
            let park_query = sqlx::query_as::<_, ParkData>(
                "SELECT area, has_playground, has_lighting 
                 FROM park 
                 WHERE site_id = $1"
            )
            .bind(id)
            .fetch_optional(&*db.pool)
            .await;
            
            match park_query {
                Ok(Some(data)) => SiteFields::Park(ParkFields {
                    area: data.area,
                    has_playground: data.has_playground,
                    has_lighting: data.has_lighting,
                }),
                Ok(None) => {
                    return Html::from(format!("<p>Park data for site ID {} not found</p>", id));
                }
                Err(e) => {
                    return Html::from(format!("<p>Error fetching park data: {}</p>", e));
                }
            }
        }
    };
    
    // Render the details template with all data
    let template = SiteApiDetailsTemplate {
        id: site.id,
        name: site.name,
        type_: site.type_,
        area_id: site.area_id,
        area_name: site.area_name,
        client_id: site.client_id,
        client_name: site.client_name,
        location: site.location,
        risk_level: site.risk_level,
        description: site.description,
        tab: query.tab,
        type_fields,
    };
    
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_type_fields_handler(
    State(_db): State<Database>,
    Query(query): Query<SiteTypeFieldsQuery>,
) -> Html<String> {
    let template = SiteTypeFieldsTemplate {
        type_: query.type_,
    };
    
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_update_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Form(form): Form<SiteUpdateForm>,
) -> Html<String> {
    // Start transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };

    // Check if area exists
    let area_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM area WHERE id = $1)")
        .bind(form.area_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(false) = area_exists {
        return Html::from(format!("<p>Area with ID {} does not exist</p>", form.area_id));
    }
    
    // Check if client exists
    let client_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM client WHERE id = $1)")
        .bind(form.client_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(false) = client_exists {
        return Html::from(format!("<p>Client with ID {} does not exist</p>", form.client_id));
    }
    
    // Get the current site type
    let current_type = sqlx::query_scalar::<_, SiteType>("SELECT type FROM site WHERE id = $1")
        .bind(id)
        .fetch_optional(&*db.pool)
        .await;
    
    let current_type = match current_type {
        Ok(Some(site_type)) => site_type,
        Ok(None) => {
            return Html::from(format!("<p>Site with ID {} not found</p>", id));
        }
        Err(e) => {
            return Html::from(format!("<p>Error fetching site type: {}</p>", e));
        }
    };
    
    // Update the main site table
    let update_site_result = sqlx::query(
        "UPDATE site
         SET name = $1, area_id = $2, client_id = $3, type = $4, 
         location = $5, risk_level = $6, description = $7
         WHERE id = $8"
    )
    .bind(&form.name)
    .bind(form.area_id)
    .bind(form.client_id)
    .bind(&form.type_)
    .bind(&form.location)
    .bind(form.risk_level)
    .bind(&form.description)
    .bind(id)
    .execute(&mut *tx)
    .await;
    
    if let Err(e) = update_site_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating site: {}</p>", e));
    }
    
    // If site type has changed, remove the old type-specific data
    if current_type != form.type_ {
        let delete_result = match current_type {
            SiteType::PowerPlant => sqlx::query("DELETE FROM power_plant WHERE site_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await,
            SiteType::Road => sqlx::query("DELETE FROM road WHERE site_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await,
            SiteType::Housing => sqlx::query("DELETE FROM housing WHERE site_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await,
            SiteType::Bridge => sqlx::query("DELETE FROM bridge WHERE site_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await,
            SiteType::Park => sqlx::query("DELETE FROM park WHERE site_id = $1")
                .bind(id)
                .execute(&mut *tx)
                .await,
        };
        
        if let Err(e) = delete_result {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error removing old site type data: {}</p>", e));
        }
    }
    
    // Insert or update the type-specific data
    let type_result = match &form.type_fields {
        SiteFields::PowerPlant(fields) => {
            if current_type == SiteType::PowerPlant {
                sqlx::query(
                    "UPDATE power_plant 
                     SET energy_output = $1, energy_source = $2, is_grid_connected = $3 
                     WHERE site_id = $4"
                )
                .bind(fields.energy_output)
                .bind(&fields.energy_source)
                .bind(fields.is_grid_connected)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO power_plant (site_id, energy_output, energy_source, is_grid_connected) 
                     VALUES ($1, $2, $3, $4)"
                )
                .bind(id)
                .bind(fields.energy_output)
                .bind(&fields.energy_source)
                .bind(fields.is_grid_connected)
                .execute(&mut *tx)
                .await
            }
        },
        SiteFields::Road(fields) => {
            if current_type == SiteType::Road {
                sqlx::query(
                    "UPDATE road 
                     SET length = $1, lanes = $2, surface = $3 
                     WHERE site_id = $4"
                )
                .bind(fields.length)
                .bind(fields.lanes)
                .bind(&fields.surface)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO road (site_id, length, lanes, surface) 
                     VALUES ($1, $2, $3, $4)"
                )
                .bind(id)
                .bind(fields.length)
                .bind(fields.lanes)
                .bind(&fields.surface)
                .execute(&mut *tx)
                .await
            }
        },
        SiteFields::Housing(fields) => {
            if current_type == SiteType::Housing {
                sqlx::query(
                    "UPDATE housing 
                     SET number_of_floors = $1, number_of_entrances = $2, type = $3, energy_efficiency = $4 
                     WHERE site_id = $5"
                )
                .bind(fields.number_of_floors)
                .bind(fields.number_of_entrances)
                .bind(&fields.type_)
                .bind(&fields.energy_efficiency)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO housing (site_id, number_of_floors, number_of_entrances, type, energy_efficiency) 
                     VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(id)
                .bind(fields.number_of_floors)
                .bind(fields.number_of_entrances)
                .bind(&fields.type_)
                .bind(&fields.energy_efficiency)
                .execute(&mut *tx)
                .await
            }
        },
        SiteFields::Bridge(fields) => {
            if current_type == SiteType::Bridge {
                sqlx::query(
                    "UPDATE bridge 
                     SET length = $1, road_material = $2, max_load = $3 
                     WHERE site_id = $4"
                )
                .bind(fields.length)
                .bind(&fields.road_material)
                .bind(fields.max_load)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO bridge (site_id, length, road_material, max_load) 
                     VALUES ($1, $2, $3, $4)"
                )
                .bind(id)
                .bind(fields.length)
                .bind(&fields.road_material)
                .bind(fields.max_load)
                .execute(&mut *tx)
                .await
            }
        },
        SiteFields::Park(fields) => {
            if current_type == SiteType::Park {
                sqlx::query(
                    "UPDATE park 
                     SET area = $1, has_playground = $2, has_lighting = $3 
                     WHERE site_id = $4"
                )
                .bind(fields.area)
                .bind(fields.has_playground)
                .bind(fields.has_lighting)
                .bind(id)
                .execute(&mut *tx)
                .await
            } else {
                sqlx::query(
                    "INSERT INTO park (site_id, area, has_playground, has_lighting) 
                     VALUES ($1, $2, $3, $4)"
                )
                .bind(id)
                .bind(fields.area)
                .bind(fields.has_playground)
                .bind(fields.has_lighting)
                .execute(&mut *tx)
                .await
            }
        }
    };
    
    if let Err(e) = type_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error updating site type data: {}</p>", e));
    }
    
    // Commit the transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }
    
    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Site '{}' updated successfully", form.name)),
        redirect: Some(format!("/sites/{}", id)),
    };
    
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_delete_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Html<String> {
    // Check if site has active tasks
    let has_active_tasks = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT 1 FROM task 
            WHERE site_id = $1 AND actual_period_end IS NULL
        )"
    )
    .bind(id)
    .fetch_one(&*db.pool)
    .await;
    
    match has_active_tasks {
        Ok(true) => {
            let template = NotificationTemplate {
                result: NotificationResult::Error,
                message: Some("Cannot delete site with active tasks".to_string()),
                redirect: Some(format!("/sites/{}", id)),
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
            
            // Get site type
            let site_type = sqlx::query_scalar::<_, SiteType>("SELECT type FROM site WHERE id = $1")
                .bind(id)
                .fetch_optional(&*db.pool)
                .await;
            
            match site_type {
                Ok(Some(site_type)) => {
                    // Delete from specific type table
                    let delete_type_result = match site_type {
                        SiteType::PowerPlant => sqlx::query("DELETE FROM power_plant WHERE site_id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await,
                        SiteType::Road => sqlx::query("DELETE FROM road WHERE site_id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await,
                        SiteType::Housing => sqlx::query("DELETE FROM housing WHERE site_id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await,
                        SiteType::Bridge => sqlx::query("DELETE FROM bridge WHERE site_id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await,
                        SiteType::Park => sqlx::query("DELETE FROM park WHERE site_id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await,
                    };
                    
                    if let Err(e) = delete_type_result {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error deleting site type data: {}</p>", e));
                    }
                    
                    // Delete all completed tasks and their related material records
                    if let Err(e) = sqlx::query(
                        "DELETE FROM expenditure 
                         WHERE task_id IN (SELECT id FROM task WHERE site_id = $1 AND actual_period_end IS NOT NULL)"
                    )
                    .bind(id)
                    .execute(&mut *tx)
                    .await 
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error removing task materials: {}</p>", e));
                    }
                    
                    if let Err(e) = sqlx::query("DELETE FROM task WHERE site_id = $1 AND actual_period_end IS NOT NULL")
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error removing tasks: {}</p>", e));
                    }
                    
                    // Delete equipment allocations
                    if let Err(e) = sqlx::query("DELETE FROM equipment_allocation WHERE site_id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await
                    {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error removing equipment allocations: {}</p>", e));
                    }
                    
                    // Finally delete site
                    let delete_site_result = sqlx::query("DELETE FROM site WHERE id = $1")
                        .bind(id)
                        .execute(&mut *tx)
                        .await;
                    
                    if let Err(e) = delete_site_result {
                        let _ = tx.rollback().await;
                        return Html::from(format!("<p>Error deleting site: {}</p>", e));
                    }
                    
                    // Commit transaction
                    if let Err(e) = tx.commit().await {
                        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
                    }
                    
                    let template = NotificationTemplate {
                        result: NotificationResult::Success,
                        message: Some("Site successfully deleted".to_string()),
                        redirect: Some("/sites".to_string()),
                    };
                    
                    match template.render() {
                        Ok(html) => Html::from(html),
                        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
                    }
                },
                Ok(None) => {
                    let _ = tx.rollback().await;
                    Html::from(format!("<p>Site with ID {} not found</p>", id))
                },
                Err(e) => {
                    let _ = tx.rollback().await;
                    Html::from(format!("<p>Error fetching site type: {}</p>", e))
                }
            }
        },
        Err(e) => {
            Html::from(format!("<p>Error checking for active tasks: {}</p>", e))
        }
    }
}

async fn sites_list_api_handler(
    State(db): State<Database>,
    Query(pagination): Query<Pagination>,
    Form(filter): Form<SiteListFilter>,
) -> Html<String> {
    // Build query with filters and sorting
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT s.id, s.name, s.type, s.area_id, a.name as area_name,
         a.department_id, d.name as department_name, s.client_id, c.name as client_name,
         CASE
            WHEN EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id AND t.actual_period_end IS NULL) THEN 'In Progress'
            WHEN NOT EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id) THEN 'Planned'
            ELSE 'Completed'
         END as status
         FROM site s
         JOIN area a ON s.area_id = a.id
         JOIN department d ON a.department_id = d.id
         JOIN client c ON s.client_id = c.id"
    );

    let mut where_added = false;

    // Add filters
    if let Some(area_id) = &filter.area_id {
        query_builder.push(" WHERE s.area_id = ");
        query_builder.push_bind(area_id);
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

    if let Some(client_id) = &filter.client_id {
        if where_added {
            query_builder.push(" AND s.client_id = ");
        } else {
            query_builder.push(" WHERE s.client_id = ");
            where_added = true;
        }
        query_builder.push_bind(client_id);
    }

    if let Some(type_) = &filter.type_ {
        if where_added {
            query_builder.push(" AND s.type = ");
        } else {
            query_builder.push(" WHERE s.type = ");
            where_added = true;
        }
        query_builder.push_bind(type_);
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND s.name ILIKE ");
        } else {
            query_builder.push(" WHERE s.name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", name));
    }

    if let Some(status) = &filter.status {
        if where_added {
            query_builder.push(" AND ");
        } else {
            query_builder.push(" WHERE ");
            where_added = true;
        }
        
        match status.as_str() {
            "in_progress" => {
                query_builder.push("EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id AND t.actual_period_end IS NULL)");
            },
            "planned" => {
                query_builder.push("NOT EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id)");
            },
            "completed" => {
                query_builder.push("NOT EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id AND t.actual_period_end IS NULL) AND EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id)");
            },
            _ => {}
        }
    }

    // Count total results for pagination - PROPERLY BIND PARAMETERS
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM site s
         JOIN area a ON s.area_id = a.id 
         JOIN department d ON a.department_id = d.id
         JOIN client c ON s.client_id = c.id"
    );
    
    let mut count_where_added = false;
    
    if let Some(area_id) = &filter.area_id {
        count_query_builder.push(" WHERE s.area_id = ");
        count_query_builder.push_bind(area_id);
        count_where_added = true;
    }

    if let Some(department_id) = &filter.department_id {
        if count_where_added {
            count_query_builder.push(" AND a.department_id = ");
        } else {
            count_query_builder.push(" WHERE a.department_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(department_id);
    }

    if let Some(client_id) = &filter.client_id {
        if count_where_added {
            count_query_builder.push(" AND s.client_id = ");
        } else {
            count_query_builder.push(" WHERE s.client_id = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(client_id);
    }

    if let Some(type_) = &filter.type_ {
        if count_where_added {
            count_query_builder.push(" AND s.type = ");
        } else {
            count_query_builder.push(" WHERE s.type = ");
            count_where_added = true;
        }
        count_query_builder.push_bind(type_);
    }

    if let Some(name) = &filter.name {
        if count_where_added {
            count_query_builder.push(" AND s.name ILIKE ");
        } else {
            count_query_builder.push(" WHERE s.name ILIKE ");
            count_where_added = true;
        }
        count_query_builder.push_bind(format!("%{}%", name));
    }

    if let Some(status) = &filter.status {
        if count_where_added {
            count_query_builder.push(" AND ");
        } else {
            count_query_builder.push(" WHERE ");
            count_where_added = true;
        }
        
        match status.as_str() {
            "in_progress" => {
                count_query_builder.push("EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id AND t.actual_period_end IS NULL)");
            },
            "planned" => {
                count_query_builder.push("NOT EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id)");
            },
            "completed" => {
                count_query_builder.push("NOT EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id AND t.actual_period_end IS NULL) AND EXISTS (SELECT 1 FROM task t WHERE t.site_id = s.id)");
            },
            _ => {}
        }
    }
    
    let count = match count_query_builder.build_query_scalar::<i64>().fetch_one(&*db.pool).await {
        Ok(count) => count,
        Err(e) => return Html::from(format!("<p>Error counting sites: {}</p>", e)),
    };

    // Add sorting
    query_builder.push(" ORDER BY ");
    match filter.sort.sort_by.as_str() {
        "name" => query_builder.push("s.name"),
        "type" => query_builder.push("s.type"),
        "area" => query_builder.push("a.name"),
        "client" => query_builder.push("c.name"),
        _ => query_builder.push("s.id"),
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
    let query = query_builder.build_query_as::<SiteListItem>();
    let sites = match query.fetch_all(&*db.pool).await {
        Ok(sites) => sites,
        Err(e) => return Html::from(format!("<p>Error fetching sites: {}</p>", e)),
    };

    // Calculate number of pages
    let num_pages = (count as f64 / pagination.page_size as f64).ceil() as u32;

    let template = SiteListTemplate {
        sites,
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

async fn site_create_handler(
    State(db): State<Database>,
    Form(form): Form<SiteCreateForm>,
) -> Html<String> {
    // Start transaction
    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return Html::from(format!("<p>Error starting transaction: {}</p>", e));
        }
    };
    
    // Check if area exists
    let area_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM area WHERE id = $1)")
        .bind(form.area_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(false) = area_exists {
        return Html::from(format!("<p>Area with ID {} does not exist</p>", form.area_id));
    }
    
    // Check if client exists
    let client_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM client WHERE id = $1)")
        .bind(form.client_id)
        .fetch_one(&*db.pool)
        .await;
    
    if let Ok(false) = client_exists {
        return Html::from(format!("<p>Client with ID {} does not exist</p>", form.client_id));
    }

    // Insert into site table
    let site_result = sqlx::query(
        "INSERT INTO site (name, area_id, client_id, type, location, risk_level, description) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id"
    )
    .bind(&form.name)
    .bind(form.area_id)
    .bind(form.client_id)
    .bind(&form.type_)
    .bind(&form.location)
    .bind(form.risk_level)
    .bind(&form.description)
    .fetch_one(&mut *tx)
    .await;
    
    let site_id = match site_result {
        Ok(row) => {
            let id: i32 = row.get(0);
            id
        },
        Err(e) => {
            let _ = tx.rollback().await;
            return Html::from(format!("<p>Error creating site: {}</p>", e));
        }
    };
    
    // Insert into type-specific table
    let type_result = match &form.type_fields {
        SiteFields::PowerPlant(fields) => {
            sqlx::query(
                "INSERT INTO power_plant (site_id, energy_output, energy_source, is_grid_connected) 
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(site_id)
            .bind(fields.energy_output)
            .bind(&fields.energy_source)
            .bind(fields.is_grid_connected)
            .execute(&mut *tx)
            .await
        },
        SiteFields::Road(fields) => {
            sqlx::query(
                "INSERT INTO road (site_id, length, lanes, surface) 
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(site_id)
            .bind(fields.length)
            .bind(fields.lanes)
            .bind(&fields.surface)
            .execute(&mut *tx)
            .await
        },
        SiteFields::Housing(fields) => {
            sqlx::query(
                "INSERT INTO housing (site_id, number_of_floors, number_of_entrances, type, energy_efficiency) 
                 VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(site_id)
            .bind(fields.number_of_floors)
            .bind(fields.number_of_entrances)
            .bind(&fields.type_)
            .bind(&fields.energy_efficiency)
            .execute(&mut *tx)
            .await
        },
        SiteFields::Bridge(fields) => {
            sqlx::query(
                "INSERT INTO bridge (site_id, length, road_material, max_load) 
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(site_id)
            .bind(fields.length)
            .bind(&fields.road_material)
            .bind(fields.max_load)
            .execute(&mut *tx)
            .await
        },
        SiteFields::Park(fields) => {
            sqlx::query(
                "INSERT INTO park (site_id, area, has_playground, has_lighting) 
                 VALUES ($1, $2, $3, $4)"
            )
            .bind(site_id)
            .bind(fields.area)
            .bind(fields.has_playground)
            .bind(fields.has_lighting)
            .execute(&mut *tx)
            .await
        }
    };
    
    if let Err(e) = type_result {
        let _ = tx.rollback().await;
        return Html::from(format!("<p>Error creating site type data: {}</p>", e));
    }
    
    // Commit transaction
    if let Err(e) = tx.commit().await {
        return Html::from(format!("<p>Error committing transaction: {}</p>", e));
    }
    
    // Return success notification
    let template = NotificationTemplate {
        result: NotificationResult::Success,
        message: Some(format!("Site '{}' created successfully", form.name)),
        redirect: Some(format!("/sites/{}", site_id)),
    };
    
    match template.render() {
        Ok(html) => Html::from(html),
        Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
    }
}

async fn site_schedule_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match site_exists {
        Ok(true) => {
            // Fetch tasks for this site
            let tasks_query = sqlx::query_as::<_, TaskListItem>(
                "SELECT 
                    t.id, 
                    t.name, 
                    t.brigade_id,
                    CASE 
                        WHEN t.brigade_id IS NOT NULL THEN 
                            (SELECT CONCAT(e.last_name, ' ', e.first_name) 
                             FROM brigade b
                             JOIN worker w ON b.brigadier_id = w.id
                             JOIN employee e ON w.id = e.id
                             WHERE b.id = t.brigade_id)
                        ELSE NULL
                    END AS brigadier_name,
                    t.period_start, 
                    t.expected_period_end, 
                    t.actual_period_end,
                    CASE
                        WHEN t.actual_period_end IS NOT NULL THEN 'Completed'
                        WHEN t.brigade_id IS NOT NULL THEN 'In Progress'
                        ELSE 'Planned'
                    END as status
                FROM task t
                WHERE t.site_id = $1
                ORDER BY 
                    CASE 
                        WHEN t.actual_period_end IS NULL AND t.brigade_id IS NOT NULL THEN 1
                        WHEN t.actual_period_end IS NULL AND t.brigade_id IS NULL THEN 2
                        ELSE 3
                    END,
                    t.period_start
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let tasks = match tasks_query.fetch_all(&*db.pool).await {
                Ok(tasks) => tasks,
                Err(e) => return Html::from(format!("<p>Error fetching tasks: {}</p>", e)),
            };
            
            let template = SiteScheduleTemplate {
                id,
                tasks,
                pagination,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Site with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking site: {}</p>", e))
        }
    }
}

async fn site_materials_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match site_exists {
        Ok(true) => {
            // Fetch materials for this site
            let materials_query = sqlx::query_as::<_, MaterialListItem>(
                "SELECT 
                    m.id, 
                    m.name, 
                    SUM(e.expected_amount) as expected_amount,
                    SUM(e.actual_amount) as actual_amount,
                    m.units,
                    m.cost,
                    SUM(COALESCE(e.actual_amount, e.expected_amount) * m.cost) as total_cost
                FROM material m
                JOIN expenditure e ON m.id = e.material_id
                JOIN task t ON e.task_id = t.id
                WHERE t.site_id = $1
                GROUP BY m.id, m.name, m.units, m.cost
                ORDER BY m.name
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let materials = match materials_query.fetch_all(&*db.pool).await {
                Ok(materials) => materials,
                Err(e) => return Html::from(format!("<p>Error fetching materials: {}</p>", e)),
            };
            
            let template = SiteMaterialsTemplate {
                id,
                materials,
                pagination,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Site with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking site: {}</p>", e))
        }
    }
}

async fn site_equipment_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match site_exists {
        Ok(true) => {
            // Fetch equipment allocations for this site
            let equipment_query = sqlx::query_as::<_, EquipmentListItem>(
                "SELECT 
                    e.id, 
                    e.name, 
                    ea.amount,
                    ea.period_start,
                    ea.period_end
                FROM equipment e
                JOIN equipment_allocation ea ON e.id = ea.equipment_id
                WHERE ea.site_id = $1
                ORDER BY ea.period_start DESC
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let equipment = match equipment_query.fetch_all(&*db.pool).await {
                Ok(equipment) => equipment,
                Err(e) => return Html::from(format!("<p>Error fetching equipment: {}</p>", e)),
            };
            
            let template = SiteEquipmentTemplate {
                id,
                equipment,
                pagination,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Site with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking site: {}</p>", e))
        }
    }
}

async fn site_brigades_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match site_exists {
        Ok(true) => {
            // Fetch brigades assigned to this site currently or in the past
            let brigades_query = sqlx::query_as::<_, BrigadeListItem>(
                "SELECT DISTINCT
                    b.id, 
                    b.brigadier_id,
                    (SELECT CONCAT(e.last_name, ' ', e.first_name) 
                     FROM worker w
                     JOIN employee e ON w.id = e.id
                     WHERE w.id = b.brigadier_id) as brigadier_name,
                    (SELECT COUNT(*) FROM assignment a WHERE a.brigade_id = b.id) as worker_count,
                    (SELECT t.name 
                     FROM task t 
                     WHERE t.brigade_id = b.id AND t.site_id = $1 AND t.actual_period_end IS NULL
                     LIMIT 1) as current_task
                FROM brigade b
                JOIN task t ON b.id = t.brigade_id
                WHERE t.site_id = $1
                ORDER BY worker_count DESC
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let brigades = match brigades_query.fetch_all(&*db.pool).await {
                Ok(brigades) => brigades,
                Err(e) => return Html::from(format!("<p>Error fetching brigades: {}</p>", e)),
            };
            
            let template = SiteBrigadesTemplate {
                id,
                brigades,
                pagination,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Site with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking site: {}</p>", e))
        }
    }
}

async fn site_reports_handler(
    State(db): State<Database>,
    Path(id): Path<i32>,
    Query(pagination): Query<Pagination>,
) -> Html<String> {
    // Check if site exists
    let site_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM site WHERE id = $1)")
        .bind(id)
        .fetch_one(&*db.pool)
        .await;
    
    match site_exists {
        Ok(true) => {
            // Fetch task reports for this site
            let reports_query = sqlx::query_as::<_, ReportListItem>(
                "SELECT 
                    t.id as task_id, 
                    t.name as task_name,
                    t.period_start,
                    t.expected_period_end,
                    t.actual_period_end,
                    CASE
                        WHEN t.actual_period_end IS NULL THEN 0
                        ELSE (t.actual_period_end - t.expected_period_end)::integer
                    END as delay
                FROM task t
                WHERE t.site_id = $1 AND (t.actual_period_end IS NOT NULL OR
                                         (t.actual_period_end IS NULL AND 
                                          t.expected_period_end < CURRENT_TIMESTAMP))
                ORDER BY t.period_start DESC
                LIMIT $2 OFFSET $3"
            )
            .bind(id)
            .bind(pagination.page_size as i32)
            .bind((pagination.page_number as i32 - 1) * pagination.page_size as i32);
            
            let reports = match reports_query.fetch_all(&*db.pool).await {
                Ok(reports) => reports,
                Err(e) => return Html::from(format!("<p>Error fetching reports: {}</p>", e)),
            };
            
            let template = SiteReportsTemplate {
                id,
                reports,
                pagination,
            };
            
            match template.render() {
                Ok(html) => Html::from(html),
                Err(e) => Html::from(format!("<p>Error rendering template: {}</p>", e)),
            }
        },
        Ok(false) => {
            Html::from(format!("<p>Site with ID {} does not exist</p>", id))
        },
        Err(e) => {
            Html::from(format!("<p>Error checking site: {}</p>", e))
        }
    }
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        // Page endpoints
        .route("/sites", get(sites_list_handler))
        .route("/sites/{id}", get(site_details_handler))
        .route("/sites/new", get(site_new_handler))
        .route("/sites/{id}/edit", get(site_edit_handler))
        // HTMX endpoints
        .route("/api/sites/{id}", get(site_api_details_handler))
        .route("/api/sites/type-fields", get(site_type_fields_handler))
        .route("/api/sites/{id}", put(site_update_handler))
        .route("/api/sites/{id}", delete(site_delete_handler))
        .route("/api/sites", get(sites_list_api_handler))
        .route("/api/sites", post(site_create_handler))
        .route("/api/sites/{id}/schedule", get(site_schedule_handler))
        .route("/api/sites/{id}/materials", get(site_materials_handler))
        .route("/api/sites/{id}/equipment", get(site_equipment_handler))
        .route("/api/sites/{id}/brigades", get(site_brigades_handler))
        .route("/api/sites/{id}/reports", get(site_reports_handler))
}