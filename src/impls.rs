use crate::general::{SiteType, RiskLevel, Gender, Profession, Qualification, Position, FuelType};
use serde::{Deserialize, Serialize};

impl std::fmt::Display for SiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SiteType::PowerPlant => write!(f, "Power Plant"),
            SiteType::Road => write!(f, "Road"),
            SiteType::Housing => write!(f, "Housing"),
            SiteType::Bridge => write!(f, "Bridge"),
            SiteType::Park => write!(f, "Park"),
        }
    }
}

impl std::str::FromStr for SiteType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "power_plant" => Ok(SiteType::PowerPlant),
            "road" => Ok(SiteType::Road),
            "housing" => Ok(SiteType::Housing),
            "bridge" => Ok(SiteType::Bridge),
            "park" => Ok(SiteType::Park),
            _ => Err(format!("Invalid SiteType: {}", s)),
        }
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Meidum => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
        }
    }
}

impl std::str::FromStr for RiskLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(RiskLevel::Low),
            "medium" => Ok(RiskLevel::Meidum),
            "high" => Ok(RiskLevel::High),
            _ => Err(format!("Invalid RiskLevel: {}", s)),
        }
    }
}

impl std::fmt::Display for Profession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Profession::Electrician => write!(f, "Electrician"),
            Profession::Plumber => write!(f, "Plumber"),
            Profession::Welder => write!(f, "Welder"),
            Profession::Driver => write!(f, "Driver"),
            Profession::Mason => write!(f, "Mason"),
        }
    }
}

impl std::str::FromStr for Profession {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "electrician" => Ok(Profession::Electrician),
            "plumber" => Ok(Profession::Plumber),
            "welder" => Ok(Profession::Welder),
            "driver" => Ok(Profession::Driver),
            "mason" => Ok(Profession::Mason),
            _ => Err(format!("Invalid Profession: {}", s)),
        }
    }
}

impl std::fmt::Display for Qualification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Qualification::Technician => write!(f, "Technician"),
            Qualification::Technologist => write!(f, "Technologist"),
            Qualification::Engineer => write!(f, "Engineer"),
        }
    }
}

impl std::str::FromStr for Qualification {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "technician" => Ok(Qualification::Technician),
            "technologist" => Ok(Qualification::Technologist),
            "engineer" => Ok(Qualification::Engineer),
            _ => Err(format!("Invalid Qualification: {}", s)),
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Master => write!(f, "Master"),
            Position::Foreman => write!(f, "Foreman"),
        }
    }
}

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "master" => Ok(Position::Master),
            "foreman" => Ok(Position::Foreman),
            _ => Err(format!("Invalid Position: {}", s)),
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

impl std::str::FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            _ => Err(format!("Invalid gender: {}", s)),
        }
    }
}

impl std::fmt::Display for FuelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuelType::Gasoline => write!(f, "Gasoline"),
            FuelType::Diesel => write!(f, "Diesel"),
            FuelType::Electric => write!(f, "Electric"),
            FuelType::Hybrid => write!(f, "Hybrid"),
        }
    }
}

impl std::str::FromStr for FuelType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gasoline" => Ok(FuelType::Gasoline),
            "diesel" => Ok(FuelType::Diesel),
            "electric" => Ok(FuelType::Electric),
            "hybrid" => Ok(FuelType::Hybrid),
            _ => Err(format!("Invalid FuelType: {}", s)),
        }
    }
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => std::str::FromStr::from_str(s).map_err(serde::de::Error::custom).map(Some),
    }
}