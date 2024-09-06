use actix_web::web;

pub mod errors;
pub mod health;
pub mod signin;
pub mod signup;
pub mod users;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health::init());
    cfg.service(users::init());
    cfg.service(signup::init());
    cfg.service(signin::init());
}
