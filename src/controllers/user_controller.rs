use actix_web::{web, HttpResponse, Responder};
use crate::repositories::user_repository::UserRepository;
use crate::models::user_model::User;
use crate::services::jwt_service::JwtService;
use crate::schema::user_schema::{CreateUserInput, LoginUserInput};

pub struct UserController {
    repository: UserRepository,
    jwt_service: JwtService,
}

impl UserController {
    pub fn new(repository: UserRepository, jwt_service: JwtService) -> Self {
        UserController { repository, jwt_service }
    }

    pub async fn create_user(user_data: web::Json<CreateUserInput>) -> impl Responder {
        // Business logic for creating a user
        let user = User {
            id: None,
            email: user_data.email.clone(),
            first_name: user_data.first_name.clone(),
            last_name: user_data.last_name.clone(),
            bio: None,
            contact: None,
            date_of_birth: None,
            gender: None,
            occupation: None,
            picture: None,
            verified: false,
        };

        // Save to repository
        let result = self.repository.create_user(&user, &user_data.password);
        match result {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::BadRequest().json(err),
        }
    }

    // Add more controller functions for authentication, user updates, etc.
}
