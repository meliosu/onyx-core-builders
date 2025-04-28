use crate::database::Database;
use crate::general::{Qualification, SiteType, Position, Profession};
use askama::Template;
use axum::{
    extract::{Form, Query, State},
    response::Html,
    routing::get,
};
use serde::{Deserialize, Serialize};

// -------- Selector Types --------

// Department selectors
#[derive(Deserialize)]
pub struct DepartmentFilter {
    pub name: Option<String>,
}

// #[derive(Template, Serialize)]
// #[template(path = "selectors/departments.html")]
pub struct DepartmentSelectorTemplate {
    pub departments: Vec<DepartmentSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/areas.html")]
pub struct AreaSelectorTemplate {
    pub areas: Vec<AreaSelectorItem>,
}

#[derive(Serialize)]
pub struct AreaSelectorItem {
    pub id: i64,
    pub name: String,
}

// Client selectors
#[derive(Deserialize)]
pub struct ClientFilter {
    pub name: Option<String>,
}

// #[derive(Template, Serialize)]
// #[template(path = "selectors/clients.html")]
pub struct ClientSelectorTemplate {
    pub clients: Vec<ClientSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/technical_personnel.html")]
pub struct TechnicalPersonnelSelectorTemplate {
    pub personnel: Vec<TechnicalPersonnelSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/workers.html")]
pub struct WorkerSelectorTemplate {
    pub workers: Vec<WorkerSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/brigades.html")]
pub struct BrigadeSelectorTemplate {
    pub brigades: Vec<BrigadeSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/sites.html")]
pub struct SiteSelectorTemplate {
    pub sites: Vec<SiteSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/equipment.html")]
pub struct EquipmentSelectorTemplate {
    pub equipment: Vec<EquipmentSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/materials.html")]
pub struct MaterialSelectorTemplate {
    pub materials: Vec<MaterialSelectorItem>,
}

#[derive(Serialize)]
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

// #[derive(Template, Serialize)]
// #[template(path = "selectors/tasks.html")]
pub struct TaskSelectorTemplate {
    pub tasks: Vec<TaskSelectorItem>,
}

#[derive(Serialize)]
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
    // Should return DepartmentSelectorTemplate with filtered departments
    Html::from(String::new())
}

// Area selectors
async fn areas_selector_handler(
    State(db): State<Database>,
    Query(query): Query<AreaQuery>,
    Form(filter): Form<AreaFilter>,
) -> Html<String> {
    // Should return AreaSelectorTemplate with filtered areas
    Html::from(String::new())
}

// Client selectors
async fn clients_selector_handler(
    State(db): State<Database>,
    Form(filter): Form<ClientFilter>,
) -> Html<String> {
    // Should return ClientSelectorTemplate with filtered clients
    Html::from(String::new())
}

// Technical personnel selectors
async fn technical_personnel_selector_handler(
    State(db): State<Database>,
    Query(query): Query<TechnicalPersonnelQuery>,
    Form(filter): Form<TechnicalPersonnelFilter>,
) -> Html<String> {
    // Should return TechnicalPersonnelSelectorTemplate with filtered personnel
    Html::from(String::new())
}

// Worker selectors
async fn workers_selector_handler(
    State(db): State<Database>,
    Query(query): Query<WorkerQuery>,
    Form(filter): Form<WorkerFilter>,
) -> Html<String> {
    // Should return WorkerSelectorTemplate with filtered workers
    Html::from(String::new())
}

// Brigade selectors
async fn brigades_selector_handler(
    State(db): State<Database>,
    Query(query): Query<BrigadeQuery>,
    Form(filter): Form<BrigadeFilter>,
) -> Html<String> {
    // Should return BrigadeSelectorTemplate with filtered brigades
    Html::from(String::new())
}

// Site selectors
async fn sites_selector_handler(
    State(db): State<Database>,
    Query(query): Query<SiteQuery>,
    Form(filter): Form<SiteFilter>,
) -> Html<String> {
    // Should return SiteSelectorTemplate with filtered sites
    Html::from(String::new())
}

// Equipment selectors
async fn equipment_selector_handler(
    State(db): State<Database>,
    Query(query): Query<EquipmentQuery>,
    Form(filter): Form<EquipmentFilter>,
) -> Html<String> {
    // Should return EquipmentSelectorTemplate with filtered equipment
    Html::from(String::new())
}

// Material selectors
async fn materials_selector_handler(
    State(db): State<Database>,
    Form(filter): Form<MaterialFilter>,
) -> Html<String> {
    // Should return MaterialSelectorTemplate with filtered materials
    Html::from(String::new())
}

// Task selectors
async fn tasks_selector_handler(
    State(db): State<Database>,
    Query(query): Query<TaskQuery>,
    Form(filter): Form<TaskFilter>,
) -> Html<String> {
    // Should return TaskSelectorTemplate with filtered tasks
    Html::from(String::new())
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
