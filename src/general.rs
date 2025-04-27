use serde::{Serialize, Deserialize};

use crate::database::Database;

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Meidum,
    High,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Profession {
    Electrician,
    Plumber,
    Welder,
    Driver,
    Mason,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Qualification {
    Technician,
    Technologist,
    Engineer,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Master,
    Foreman,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub page_number: u32,
    pub page_size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Sort {
    pub sort_by: String,
    pub sort_direction: SortDirection,
}

#[derive(Serialize, Deserialize)]
pub struct QueryInfo {
    pub num_pages: u32,
    pub num_items: u32,
}

pub fn router() -> axum::Router<Database> {
    axum::Router::new()
}