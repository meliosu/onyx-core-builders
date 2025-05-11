#![allow(warnings)]

pub mod database;

mod general;
mod departments;
mod areas;
mod sites;
mod workers;
mod technical_personnel;
mod equipment;
mod clients;
mod brigades;
mod tasks;
mod materials;
mod helpers;
mod impls;
mod utils;

pub fn router() -> axum::Router<database::Database> {
    axum::Router::new()
        .merge(general::router())
        .merge(departments::router())
        .merge(areas::router())
        .merge(sites::router())
        .merge(workers::router())
        .merge(technical_personnel::router())
        .merge(equipment::router())
        .merge(clients::router())
        .merge(brigades::router())
        .merge(tasks::router())
        .merge(materials::router())
        .merge(helpers::router())
}