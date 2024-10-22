use crate::models::user_model::{User, Secret};
use mongodb::{bson::doc, sync::Client};
use crate::services::custom_errors::{BadRequest, NotFound};
use bcrypt::hash;
use mongodb::bson::{oid::ObjectId};

pub struct UserRepository {
    client: Client,
}

impl UserRepository {
    pub fn new(client: Client) -> Self {
        UserRepository { client }
    }

    pub fn create_user(&self, user_data: &User, plain_password: &str) -> Result<User, BadRequest> {
        let collection = self.client.database("mydb").collection::<User>("users");
        let secret_collection = self.client.database("mydb").collection::<Secret>("secrets");

        let hashed_password = hash(plain_password, 4).unwrap();

        let mut user = user_data.clone();
        user.verified = false;

        let insert_result = collection.insert_one(user.clone(), None);
        if let Ok(result) = insert_result {
            let user_id = result.inserted_id.as_object_id().unwrap();
            let secret = Secret {
                password: hashed_password,
                user_id,
            };

            let secret_result = secret_collection.insert_one(secret, None);
            if secret_result.is_err() {
                return Err(BadRequest("Failed to create user secret".to_string()));
            }
        } else {
            return Err(BadRequest("Failed to create user".to_string()));
        }

        Ok(user)
    }

    pub fn find_user_by_email(&self, email: &str) -> Result<User, NotFound> {
        let collection = self.client.database("mydb").collection::<User>("users");
        let filter = doc! { "email": email };
        let user = collection.find_one(filter, None).unwrap();

        if let Some(user) = user {
            return Ok(user);
        }

        Err(NotFound("User not found".to_string()))
    }

    // Add more repository functions like updating users, verifying users, etc.
}
