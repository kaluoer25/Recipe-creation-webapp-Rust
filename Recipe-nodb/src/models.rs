use actix_web:: web; 
use chrono::NaiveDateTime; 
use serde:: {Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Recipe { 
    pub user_id:i32, 
    pub recipe_id:Option<i32>, 
    pub recipe_name: String, 
    pub posted_time:Option<NaiveDateTime>, 
}

impl From<web::Json<Recipe>> for Recipe { 
    fn from (recipe:web::Json<Recipe>) -> Self { 
        Recipe { 
            recipe_id: recipe.recipe_id, 
            user_id: recipe.user_id,
            recipe_name: recipe.recipe_name.clone(), 
            posted_time: recipe.posted_time, 

        }
    }

}