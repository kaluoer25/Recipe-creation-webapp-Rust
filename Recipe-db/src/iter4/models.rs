use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Recipe {
    pub recipe_id: i32,
    pub user_id: i32,
    pub recipe_name: String,
    pub posted_time: Option<NaiveDateTime>,
}
impl From<web::Json<Recipe>> for Recipe {
    fn from(msg: web::Json<Recipe>) -> Self {
        Recipe {
            recipe_id: msg.recipe_id,
            user_id: msg.user_id,
            recipe_name: msg.recipe_name.clone(),
            posted_time: msg.posted_time,
        }
    }
}