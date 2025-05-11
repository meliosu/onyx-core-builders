use crate::database::Database;
use crate::general::{Qualification, SiteType, Position, Profession};
use askama::Template;
use axum::{
    extract::{Form, Query, State},
    response::Html,
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{Encode, FromRow};
use crate::utils::empty_string_as_none;

// -------- Selector Types --------

// Department selectors
#[derive(Deserialize)]
pub struct DepartmentQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct DepartmentFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/departments.html")]
pub struct DepartmentSelectorTemplate {
    pub departments: Vec<DepartmentSelectorItem>,
    pub query: DepartmentQuery,
}

#[derive(Serialize, FromRow)]
pub struct DepartmentSelectorItem {
    pub id: i32,
    pub name: String,
}

// Area selectors
#[derive(Deserialize)]
pub struct AreaQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub department_id: Option<i64>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct AreaFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/areas.html")]
pub struct AreaSelectorTemplate {
    pub areas: Vec<AreaSelectorItem>,
    pub query: AreaQuery,
}

#[derive(Serialize, FromRow)]
pub struct AreaSelectorItem {
    pub id: i32,
    pub name: String,
}

// Client selectors
#[derive(Deserialize)]
pub struct ClientQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct ClientFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/clients.html")]
pub struct ClientSelectorTemplate {
    pub clients: Vec<ClientSelectorItem>,
    pub query: ClientQuery,
}

#[derive(Serialize, FromRow)]
pub struct ClientSelectorItem {
    pub id: i32,
    pub name: String,
}

// Technical personnel selectors
#[derive(Deserialize)]
pub struct TechnicalPersonnelQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub qualification: Option<Qualification>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub position: Option<Position>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub department_id: Option<i64>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub area_id: Option<i64>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct TechnicalPersonnelFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/technical_personnel.html")]
pub struct TechnicalPersonnelSelectorTemplate {
    pub personnel: Vec<TechnicalPersonnelSelectorItem>,
    pub query: TechnicalPersonnelQuery,
}

#[derive(Serialize, FromRow)]
pub struct TechnicalPersonnelSelectorItem {
    pub id: i32,
    pub name: String,
    pub qualification: Qualification,
}

// Worker selectors
#[derive(Deserialize)]
pub struct WorkerQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub profession: Option<Profession>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigade_id: Option<i64>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub is_brigadier: Option<bool>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct WorkerFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/workers.html")]
pub struct WorkerSelectorTemplate {
    pub workers: Vec<WorkerSelectorItem>,
    pub query: WorkerQuery,
}

#[derive(Serialize, FromRow)]
pub struct WorkerSelectorItem {
    pub id: i32,
    pub name: String,
    pub profession: Profession,
}

// Brigade selectors
#[derive(Deserialize)]
pub struct BrigadeQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub site_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub available: Option<bool>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct BrigadeFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigadier_name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/brigades.html")]
pub struct BrigadeSelectorTemplate {
    pub brigades: Vec<BrigadeSelectorItem>,
    pub query: BrigadeQuery,
}

#[derive(Serialize, FromRow)]
pub struct BrigadeSelectorItem {
    pub id: i32,
    pub brigadier_name: String,
    pub worker_count: i64,
}

// Site selectors
#[derive(Deserialize)]
pub struct SiteQuery {
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
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct SiteFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/sites.html")]
pub struct SiteSelectorTemplate {
    pub sites: Vec<SiteSelectorItem>,
    pub query: SiteQuery,
}

#[derive(Serialize, FromRow)]
pub struct SiteSelectorItem {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: SiteType,
}

// Equipment selectors
#[derive(Deserialize)]
pub struct EquipmentQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub available: Option<bool>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct EquipmentFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/equipment.html")]
pub struct EquipmentSelectorTemplate {
    pub equipment: Vec<EquipmentSelectorItem>,
    pub query: EquipmentQuery,
}

#[derive(Serialize, FromRow)]
pub struct EquipmentSelectorItem {
    pub id: i32,
    pub name: String,
    pub available_amount: i64,
}

// Material selectors
#[derive(Deserialize)]
pub struct MaterialQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct MaterialFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/materials.html")]
pub struct MaterialSelectorTemplate {
    pub materials: Vec<MaterialSelectorItem>,
    pub query: MaterialQuery,
}

#[derive(Serialize, FromRow)]
pub struct MaterialSelectorItem {
    pub id: i32,
    pub name: String,
    pub units: String,
}

// Task selectors
#[derive(Deserialize)]
pub struct TaskQuery {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub site_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub brigade_id: Option<i32>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub status: Option<String>,
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub selected_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct TaskFilter {
    #[serde(default, deserialize_with="empty_string_as_none")]
    pub name: Option<String>,
}

#[derive(Template)]
#[template(path = "selectors/tasks.html")]
pub struct TaskSelectorTemplate {
    pub tasks: Vec<TaskSelectorItem>,
    pub query: TaskQuery,
}

#[derive(Serialize, FromRow)]
pub struct TaskSelectorItem {
    pub id: i32,
    pub name: String,
    pub site_name: String,
}

// -------- Handler Functions --------

// Department selectors
async fn departments_selector_handler(
    State(db): State<Database>,
    Query(q): Query<DepartmentQuery>,
    Form(filter): Form<DepartmentFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT id, name FROM department");

    if let Some(name) = &filter.name {
        query_builder.push(" WHERE name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<DepartmentSelectorItem>();
    let departments = match query.fetch_all(&*db.pool).await {
        Ok(departments) => departments,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading departments: {}</p>", err));
        }
    };

    let template = DepartmentSelectorTemplate { 
        departments,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Area selectors
async fn areas_selector_handler(
    State(db): State<Database>,
    Query(q): Query<AreaQuery>,
    Form(filter): Form<AreaFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT id, name FROM area");

    let mut where_added = false;

    if let Some(department_id) = &q.department_id {
        query_builder.push(" WHERE department_id = ");
        query_builder.push_bind(department_id);
        where_added = true;
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND name ILIKE ");
        } else {
            query_builder.push(" WHERE name ILIKE ");
        }
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<AreaSelectorItem>();
    let areas = match query.fetch_all(&*db.pool).await {
        Ok(areas) => areas,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading areas: {}</p>", err));
        }
    };

    let template = AreaSelectorTemplate { 
        areas,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Client selectors
async fn clients_selector_handler(
    State(db): State<Database>,
    Query(q): Query<ClientQuery>,
    Form(filter): Form<ClientFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT id, name FROM client");

    if let Some(name) = &filter.name {
        query_builder.push(" WHERE name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<ClientSelectorItem>();
    let clients = match query.fetch_all(&*db.pool).await {
        Ok(clients) => clients,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading clients: {}</p>", err));
        }
    };

    let template = ClientSelectorTemplate { 
        clients,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Technical personnel selectors
async fn technical_personnel_selector_handler(
    State(db): State<Database>,
    Query(q): Query<TechnicalPersonnelQuery>,
    Form(filter): Form<TechnicalPersonnelFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT tp.id, CONCAT(e.last_name, ' ', e.first_name) as name, tp.qualification 
         FROM technical_personnel tp 
         JOIN employee e ON tp.id = e.id"
    );

    let mut where_added = false;

    if let Some(qualification) = &q.qualification {
        query_builder.push(" WHERE tp.qualification = ");
        query_builder.push_bind(qualification);
        where_added = true;
    }

    if let Some(position) = &q.position {
        if where_added {
            query_builder.push(" AND tp.position = ");
        } else {
            query_builder.push(" WHERE tp.position = ");
            where_added = true;
        }
        query_builder.push_bind(position);
    }

    if let Some(department_id) = &q.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM department d WHERE d.supervisor_id = tp.id AND d.id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM department d WHERE d.supervisor_id = tp.id AND d.id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(")");
    }

    if let Some(area_id) = &q.area_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM area a WHERE a.supervisor_id = tp.id AND a.id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM area a WHERE a.supervisor_id = tp.id AND a.id = ");
            where_added = true;
        }
        query_builder.push_bind(area_id);
        query_builder.push(")");
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

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<TechnicalPersonnelSelectorItem>();
    let personnel = match query.fetch_all(&*db.pool).await {
        Ok(personnel) => personnel,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading technical personnel: {}</p>", err));
        }
    };

    let template = TechnicalPersonnelSelectorTemplate { 
        personnel,
        query: q, // Pass the query to the template
    };
    
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Worker selectors
async fn workers_selector_handler(
    State(db): State<Database>,
    Query(q): Query<WorkerQuery>,
    Form(filter): Form<WorkerFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT w.id, CONCAT(e.last_name, ' ', e.first_name) as name, w.profession 
         FROM worker w 
         JOIN employee e ON w.id = e.id"
    );

    let mut where_added = false;

    if let Some(profession) = &q.profession {
        query_builder.push(" WHERE w.profession = ");
        query_builder.push_bind(profession);
        where_added = true;
    }

    if let Some(brigade_id) = &q.brigade_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM assignment a WHERE a.worker_id = w.id AND a.brigade_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM assignment a WHERE a.worker_id = w.id AND a.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
        query_builder.push(")");
    }

    if let Some(is_brigadier) = &q.is_brigadier {
        let subquery = if *is_brigadier {
            " EXISTS (SELECT 1 FROM brigade b WHERE b.brigadier_id = w.id)"
        } else {
            " NOT EXISTS (SELECT 1 FROM brigade b WHERE b.brigadier_id = w.id)"
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

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<WorkerSelectorItem>();
    let workers = match query.fetch_all(&*db.pool).await {
        Ok(workers) => workers,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading workers: {}</p>", err));
        }
    };

    let template = WorkerSelectorTemplate { 
        workers,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Brigade selectors
async fn brigades_selector_handler(
    State(db): State<Database>,
    Query(q): Query<BrigadeQuery>,
    Form(filter): Form<BrigadeFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT b.id, CONCAT(e.last_name, ' ', e.first_name) as brigadier_name, 
         (SELECT COUNT(*) FROM assignment a WHERE a.brigade_id = b.id) as worker_count 
         FROM brigade b 
         JOIN worker w ON b.brigadier_id = w.id 
         JOIN employee e ON w.id = e.id"
    );

    let mut where_added = false;

    if let Some(site_id) = &q.site_id {
        query_builder.push(" WHERE EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.site_id = ");
        query_builder.push_bind(site_id);
        query_builder.push(")");
        where_added = true;
    }

    if let Some(available) = &q.available {
        let subquery = if *available {
            " NOT EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.actual_period_end IS NULL)"
        } else {
            " EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.actual_period_end IS NULL)"
        };

        if where_added {
            query_builder.push(" AND ");
        } else {
            query_builder.push(" WHERE ");
            where_added = true;
        }
        query_builder.push(subquery);
    }

    if let Some(name) = &filter.brigadier_name {
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

    query_builder.push(" ORDER BY brigadier_name");

    let query = query_builder.build_query_as::<BrigadeSelectorItem>();
    let brigades = match query.fetch_all(&*db.pool).await {
        Ok(brigades) => brigades,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading brigades: {}</p>", err));
        }
    };

    let template = BrigadeSelectorTemplate { 
        brigades,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Site selectors
async fn sites_selector_handler(
    State(db): State<Database>,
    Query(q): Query<SiteQuery>,
    Form(filter): Form<SiteFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, type FROM site"
    );

    let mut where_added = false;

    if let Some(area_id) = &q.area_id {
        query_builder.push(" WHERE area_id = ");
        query_builder.push_bind(area_id);
        where_added = true;
    }

    if let Some(department_id) = &q.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM area a WHERE a.id = site.area_id AND a.department_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM area a WHERE a.id = site.area_id AND a.department_id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(")");
    }

    if let Some(client_id) = &q.client_id {
        if where_added {
            query_builder.push(" AND client_id = ");
        } else {
            query_builder.push(" WHERE client_id = ");
            where_added = true;
        }
        query_builder.push_bind(client_id);
    }

    if let Some(type_) = &q.type_ {
        if where_added {
            query_builder.push(" AND type = ");
        } else {
            query_builder.push(" WHERE type = ");
            where_added = true;
        }
        query_builder.push_bind(type_);
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND name ILIKE ");
        } else {
            query_builder.push(" WHERE name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<SiteSelectorItem>();
    let sites = match query.fetch_all(&*db.pool).await {
        Ok(sites) => sites,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading sites: {}</p>", err));
        }
    };

    let template = SiteSelectorTemplate { 
        sites,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Equipment selectors
async fn equipment_selector_handler(
    State(db): State<Database>,
    Query(q): Query<EquipmentQuery>,
    Form(filter): Form<EquipmentFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, amount as available_amount FROM equipment"
    );

    let mut where_added = false;

    if let Some(available) = &q.available {
        if *available {
            query_builder.push(" WHERE amount > 0");
        } else {
            query_builder.push(" WHERE amount = 0");
        }
        where_added = true;
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND name ILIKE ");
        } else {
            query_builder.push(" WHERE name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<EquipmentSelectorItem>();
    let equipment = match query.fetch_all(&*db.pool).await {
        Ok(equipment) => equipment,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading equipment: {}</p>", err));
        }
    };

    let template = EquipmentSelectorTemplate { 
        equipment,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Material selectors
async fn materials_selector_handler(
    State(db): State<Database>,
    Query(q): Query<MaterialQuery>,
    Form(filter): Form<MaterialFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, units FROM material"
    );

    if let Some(name) = &filter.name {
        query_builder.push(" WHERE name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<MaterialSelectorItem>();
    let materials = match query.fetch_all(&*db.pool).await {
        Ok(materials) => materials,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading materials: {}</p>", err));
        }
    };

    let template = MaterialSelectorTemplate { 
        materials,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

#[derive(Template)]
#[template(path = "selectors/modal/materials.html")]
pub struct MaterialModalSelectorTemplate {
    pub materials: Vec<MaterialSelectorItem>,
    pub query: MaterialQuery,
}

async fn materials_modal_selector_handler(
    State(db): State<Database>,
    Query(q): Query<MaterialQuery>,
    Form(filter): Form<MaterialFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, units FROM material"
    );

    if let Some(name) = &filter.name {
        query_builder.push(" WHERE name ILIKE ");
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY name");

    let query = query_builder.build_query_as::<MaterialSelectorItem>();
    let materials = match query.fetch_all(&*db.pool).await {
        Ok(materials) => materials,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading materials: {}</p>", err));
        }
    };

    let template = MaterialModalSelectorTemplate { 
        materials,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Task selectors
async fn tasks_selector_handler(
    State(db): State<Database>,
    Query(q): Query<TaskQuery>,
    Form(filter): Form<TaskFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT t.id, t.name, s.name as site_name 
         FROM task t 
         JOIN site s ON t.site_id = s.id"
    );

    let mut where_added = false;

    if let Some(site_id) = &q.site_id {
        query_builder.push(" WHERE t.site_id = ");
        query_builder.push_bind(site_id);
        where_added = true;
    }

    if let Some(brigade_id) = &q.brigade_id {
        if where_added {
            query_builder.push(" AND t.brigade_id = ");
        } else {
            query_builder.push(" WHERE t.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
    }

    if let Some(status) = &q.status {
        match status.as_str() {
            "completed" => {
                if where_added {
                    query_builder.push(" AND t.actual_period_end IS NOT NULL");
                } else {
                    query_builder.push(" WHERE t.actual_period_end IS NOT NULL");
                    where_added = true;
                }
            },
            "in_progress" => {
                if where_added {
                    query_builder.push(" AND t.actual_period_end IS NULL AND t.brigade_id IS NOT NULL");
                } else {
                    query_builder.push(" WHERE t.actual_period_end IS NULL AND t.brigade_id IS NOT NULL");
                    where_added = true;
                }
            },
            "unassigned" => {
                if where_added {
                    query_builder.push(" AND t.brigade_id IS NULL");
                } else {
                    query_builder.push(" WHERE t.brigade_id IS NULL");
                    where_added = true;
                }
            },
            _ => {}
        }
    }

    if let Some(name) = &filter.name {
        if where_added {
            query_builder.push(" AND t.name ILIKE ");
        } else {
            query_builder.push(" WHERE t.name ILIKE ");
            where_added = true;
        }
        query_builder.push_bind(format!("%{}%", name));
    }

    query_builder.push(" ORDER BY t.name");

    let query = query_builder.build_query_as::<TaskSelectorItem>();
    let tasks = match query.fetch_all(&*db.pool).await {
        Ok(tasks) => tasks,
        Err(err) => {
            return Html::from(format!("<p class=\"text-error\">Error loading tasks: {}</p>", err));
        }
    };

    let template = TaskSelectorTemplate { 
        tasks,
        query: q,
    };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(err) => Html::from(format!("<p class=\"text-error\">Error rendering template: {}</p>", err)),
    }
}

// Router setup
pub fn router() -> axum::Router<Database> {
    axum::Router::new()
        .route("/api/selectors/departments", get(departments_selector_handler))
        .route("/api/selectors/areas", get(areas_selector_handler))
        .route("/api/selectors/clients", get(clients_selector_handler))
        .route("/api/selectors/technical-personnel", get(technical_personnel_selector_handler))
        .route("/api/selectors/workers", get(workers_selector_handler))
        .route("/api/selectors/brigades", get(brigades_selector_handler))
        .route("/api/selectors/sites", get(sites_selector_handler))
        .route("/api/selectors/equipment", get(equipment_selector_handler))
        .route("/api/selectors/materials", get(materials_selector_handler))
        .route("/api/selectors/tasks", get(tasks_selector_handler))
        .route("/api/selectors/materials/modal", get(materials_modal_selector_handler))
}
