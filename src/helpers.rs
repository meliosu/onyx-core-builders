use crate::database::Database;

pub fn router() -> axum::Router<Database> {
    axum::Router::new()
}
