use crate::general::{SiteType, RiskLevel, Gender, Profession, Qualification, Position};

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

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Meidum => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
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

impl std::fmt::Display for Qualification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Qualification::Technician => write!(f, "Technician"),
            Qualification::Technologist => write!(f, "Technologist"),
            Qualification::Engineer => write!(f, "Engineer"),
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

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}