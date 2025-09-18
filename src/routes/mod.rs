use axum::Router;
pub mod init;

pub fn init() -> Router {
    init::_routes()
}
