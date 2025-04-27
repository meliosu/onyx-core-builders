use serde::{Serialize, Deserialize};

use crate::database::Database;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SiteType {
    PowerPlant,
    Road,
    Housing,
    Bridge,
    Park,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Meidum,
    High,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Profession {
    Electrician,
    Plumber,
    Welder,
    Driver,
    Mason,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Qualification {
    Technician,
    Technologist,
    Engineer,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Position {
    Master,
    Foreman,
}

pub fn router() -> axum::Router<Database> {
    axum::Router::new()
}