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

// -------- Selector Types --------

// Department selectors
#[derive(Deserialize)]
pub struct DepartmentFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/departments.html")]
pub struct DepartmentSelectorTemplate {
    pub departments: Vec<DepartmentSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct DepartmentSelectorItem {
    pub id: i64,
    pub name: String,
}

// Area selectors
#[derive(Deserialize)]
pub struct AreaQuery {
    pub department_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct AreaFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/areas.html")]
pub struct AreaSelectorTemplate {
    pub areas: Vec<AreaSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct AreaSelectorItem {
    pub id: i64,
    pub name: String,
}

// Client selectors
#[derive(Deserialize)]
pub struct ClientFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/clients.html")]
pub struct ClientSelectorTemplate {
    pub clients: Vec<ClientSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct ClientSelectorItem {
    pub id: i64,
    pub name: String,
}

// Technical personnel selectors
#[derive(Deserialize)]
pub struct TechnicalPersonnelQuery {
    pub qualification: Option<Qualification>,
    pub position: Option<Position>,
    pub department_id: Option<i64>,
    pub area_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct TechnicalPersonnelFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/technical_personnel.html")]
pub struct TechnicalPersonnelSelectorTemplate {
    pub personnel: Vec<TechnicalPersonnelSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct TechnicalPersonnelSelectorItem {
    pub id: i64,
    pub name: String,
    pub qualification: Qualification,
}

// Worker selectors
#[derive(Deserialize)]
pub struct WorkerQuery {
    pub profession: Option<Profession>,
    pub brigade_id: Option<i64>,
    pub is_brigadier: Option<bool>,
}

#[derive(Deserialize)]
pub struct WorkerFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/workers.html")]
pub struct WorkerSelectorTemplate {
    pub workers: Vec<WorkerSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct WorkerSelectorItem {
    pub id: i64,
    pub name: String,
    pub profession: Profession,
}

// Brigade selectors
#[derive(Deserialize)]
pub struct BrigadeQuery {
    pub site_id: Option<i64>,
    pub available: Option<bool>,
}

#[derive(Deserialize)]
pub struct BrigadeFilter {
    pub brigadier_name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/brigades.html")]
pub struct BrigadeSelectorTemplate {
    pub brigades: Vec<BrigadeSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct BrigadeSelectorItem {
    pub id: i64,
    pub brigadier_name: String,
    pub worker_count: i64,
}

// Site selectors
#[derive(Deserialize)]
pub struct SiteQuery {
    pub area_id: Option<i64>,
    pub department_id: Option<i64>,
    pub client_id: Option<i64>,
    pub type_: Option<SiteType>,
}

#[derive(Deserialize)]
pub struct SiteFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/sites.html")]
pub struct SiteSelectorTemplate {
    pub sites: Vec<SiteSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct SiteSelectorItem {
    pub id: i64,
    pub name: String,
    pub type_: SiteType,
}

// Equipment selectors
#[derive(Deserialize)]
pub struct EquipmentQuery {
    pub available: Option<bool>,
}

#[derive(Deserialize)]
pub struct EquipmentFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/equipment.html")]
pub struct EquipmentSelectorTemplate {
    pub equipment: Vec<EquipmentSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct EquipmentSelectorItem {
    pub id: i64,
    pub name: String,
    pub available_amount: i64,
}

// Material selectors
#[derive(Deserialize)]
pub struct MaterialFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/materials.html")]
pub struct MaterialSelectorTemplate {
    pub materials: Vec<MaterialSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct MaterialSelectorItem {
    pub id: i64,
    pub name: String,
    pub units: String,
}

// Task selectors
#[derive(Deserialize)]
pub struct TaskQuery {
    pub site_id: Option<i64>,
    pub brigade_id: Option<i64>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct TaskFilter {
    pub name: Option<String>,
}

#[derive(Template, Serialize)]
#[template(path = "selectors/tasks.html")]
pub struct TaskSelectorTemplate {
    pub tasks: Vec<TaskSelectorItem>,
}

#[derive(Serialize, FromRow)]
pub struct TaskSelectorItem {
    pub id: i64,
    pub name: String,
    pub site_name: String,
}

// -------- Handler Functions --------

// Department selectors
async fn departments_selector_handler(
    State(db): State<Database>,
    Form(filter): Form<DepartmentFilter>,
) -> Html<String> {
    let mut query = sqlx::QueryBuilder::new("SELECT id, name FROM department");

    if let Some(name) = &filter.name {
        query.push(" WHERE name ILIKE ");
        query.push_bind(format!("%{}%", name));
    }

    query.push(" ORDER BY name");

    let query = query.build_query_as::<DepartmentSelectorItem>();
    let departments = match query.fetch_all(&*db.pool).await {
        Ok(departments) => departments,
        Err(_) => vec![],
    };

    let template = DepartmentSelectorTemplate { departments };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Area selectors
async fn areas_selector_handler(
    State(db): State<Database>,
    Query(query): Query<AreaQuery>,
    Form(filter): Form<AreaFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new("SELECT id, name FROM area");

    let mut where_added = false;

    if let Some(department_id) = &query.department_id {
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
        Err(_) => vec![],
    };

    let template = AreaSelectorTemplate { areas };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Client selectors
async fn clients_selector_handler(
    State(db): State<Database>,
    Form(filter): Form<ClientFilter>,
) -> Html<String> {
    let mut query = sqlx::QueryBuilder::new("SELECT id, name FROM client");

    if let Some(name) = &filter.name {
        query.push(" WHERE name ILIKE ");
        query.push_bind(format!("%{}%", name));
    }

    query.push(" ORDER BY name");

    let query = query.build_query_as::<ClientSelectorItem>();
    let clients = match query.fetch_all(&*db.pool).await {
        Ok(clients) => clients,
        Err(_) => vec![],
    };

    let template = ClientSelectorTemplate { clients };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Technical personnel selectors
async fn technical_personnel_selector_handler(
    State(db): State<Database>,
    Query(query): Query<TechnicalPersonnelQuery>,
    Form(filter): Form<TechnicalPersonnelFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT tp.id, CONCAT(e.last_name, ' ', e.first_name) as name, tp.qualification 
         FROM technical_personnel tp 
         JOIN employee e ON tp.id = e.id"
    );

    let mut where_added = false;

    if let Some(qualification) = &query.qualification {
        query_builder.push(" WHERE tp.qualification = ");
        query_builder.push_bind(qualification);
        where_added = true;
    }

    if let Some(position) = &query.position {
        if where_added {
            query_builder.push(" AND tp.position = ");
        } else {
            query_builder.push(" WHERE tp.position = ");
            where_added = true;
        }
        query_builder.push_bind(position);
    }

    if let Some(department_id) = &query.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM department d WHERE d.supervisor_id = tp.id AND d.id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM department d WHERE d.supervisor_id = tp.id AND d.id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(")");
    }

    if let Some(area_id) = &query.area_id {
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
        Err(_) => vec![],
    };

    let template = TechnicalPersonnelSelectorTemplate { personnel };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Worker selectors
async fn workers_selector_handler(
    State(db): State<Database>,
    Query(query): Query<WorkerQuery>,
    Form(filter): Form<WorkerFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT w.id, CONCAT(e.last_name, ' ', e.first_name) as name, w.profession 
         FROM worker w 
         JOIN employee e ON w.id = e.id"
    );

    let mut where_added = false;

    if let Some(profession) = &query.profession {
        query_builder.push(" WHERE w.profession = ");
        query_builder.push_bind(profession);
        where_added = true;
    }

    if let Some(brigade_id) = &query.brigade_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM assignment a WHERE a.worker_id = w.id AND a.brigade_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM assignment a WHERE a.worker_id = w.id AND a.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
        query_builder.push(")");
    }

    if let Some(is_brigadier) = &query.is_brigadier {
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
        Err(_) => vec![],
    };

    let template = WorkerSelectorTemplate { workers };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Brigade selectors
async fn brigades_selector_handler(
    State(db): State<Database>,
    Query(query): Query<BrigadeQuery>,
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

    if let Some(site_id) = &query.site_id {
        query_builder.push(" WHERE EXISTS (SELECT 1 FROM task t WHERE t.brigade_id = b.id AND t.site_id = ");
        query_builder.push_bind(site_id);
        query_builder.push(")");
        where_added = true;
    }

    if let Some(available) = &query.available {
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
        Err(_) => vec![],
    };

    let template = BrigadeSelectorTemplate { brigades };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Site selectors
async fn sites_selector_handler(
    State(db): State<Database>,
    Query(query): Query<SiteQuery>,
    Form(filter): Form<SiteFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, type as type_ FROM site"
    );

    let mut where_added = false;

    if let Some(area_id) = &query.area_id {
        query_builder.push(" WHERE area_id = ");
        query_builder.push_bind(area_id);
        where_added = true;
    }

    if let Some(department_id) = &query.department_id {
        if where_added {
            query_builder.push(" AND EXISTS (SELECT 1 FROM area a WHERE a.id = site.area_id AND a.department_id = ");
        } else {
            query_builder.push(" WHERE EXISTS (SELECT 1 FROM area a WHERE a.id = site.area_id AND a.department_id = ");
            where_added = true;
        }
        query_builder.push_bind(department_id);
        query_builder.push(")");
    }

    if let Some(client_id) = &query.client_id {
        if where_added {
            query_builder.push(" AND client_id = ");
        } else {
            query_builder.push(" WHERE client_id = ");
            where_added = true;
        }
        query_builder.push_bind(client_id);
    }

    if let Some(type_) = &query.type_ {
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
        Err(_) => vec![],
    };

    let template = SiteSelectorTemplate { sites };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Equipment selectors
async fn equipment_selector_handler(
    State(db): State<Database>,
    Query(query): Query<EquipmentQuery>,
    Form(filter): Form<EquipmentFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, amount as available_amount FROM equipment"
    );

    let mut where_added = false;

    if let Some(available) = &query.available {
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
        Err(_) => vec![],
    };

    let template = EquipmentSelectorTemplate { equipment };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Material selectors
async fn materials_selector_handler(
    State(db): State<Database>,
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
        Err(_) => vec![],
    };

    let template = MaterialSelectorTemplate { materials };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
    }
}

// Task selectors
async fn tasks_selector_handler(
    State(db): State<Database>,
    Query(query): Query<TaskQuery>,
    Form(filter): Form<TaskFilter>,
) -> Html<String> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT t.id, t.name, s.name as site_name 
         FROM task t 
         JOIN site s ON t.site_id = s.id"
    );

    let mut where_added = false;

    if let Some(site_id) = &query.site_id {
        query_builder.push(" WHERE t.site_id = ");
        query_builder.push_bind(site_id);
        where_added = true;
    }

    if let Some(brigade_id) = &query.brigade_id {
        if where_added {
            query_builder.push(" AND t.brigade_id = ");
        } else {
            query_builder.push(" WHERE t.brigade_id = ");
            where_added = true;
        }
        query_builder.push_bind(brigade_id);
    }

    if let Some(status) = &query.status {
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
        Err(_) => vec![],
    };

    let template = TaskSelectorTemplate { tasks };
    match template.render() {
        Ok(html) => Html::from(html),
        Err(_) => Html::from(String::new()),
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
}
