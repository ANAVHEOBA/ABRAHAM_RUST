use actix_web::web;
use crate::controllers::user_controller::UserController;
use crate::repositories::user_repository::UserRepository;
use crate::services::jwt_service::JwtService;
use mongodb::sync::Client;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let user_repository = UserRepository::new(client);
    let jwt_service = JwtService::new("your_jwt_secret".to_string());

    let controller = UserController::new(user_repository, jwt_service);

    cfg.service(
        web::resource("/users")
            .route(web::post().to(controller.create_user)),
    );
}
