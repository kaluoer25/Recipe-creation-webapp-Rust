use super::models::Recipe;
use sqlx::postgres::PgPool;

pub async fn get_recipes_from_user_db(pool: &PgPool, user_id: i32) -> Vec<Recipe> {
    // Prepare SQL statement
    let recipe_rows = sqlx::query!(
        "SELECT user_id, recipe_id, recipe_name, posted_time FROM yummyrecipe_c4 where user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await
    .unwrap();
    // Extract result
    recipe_rows
        .iter()
        .map(|recipe_row| Recipe {
            recipe_id: recipe_row.recipe_id,
            user_id: recipe_row.user_id,
            recipe_name: recipe_row.recipe_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(recipe_row.posted_time.unwrap())),
        })
        .collect()
}

//Return result

pub async fn get_recipe_details_db(pool: &PgPool, user_id: i32, recipe_id: i32) -> Recipe {
    // Prepare SQL statement
    let recipe_row = sqlx::query!(
        "SELECT user_id, recipe_id, recipe_name, posted_time FROM yummyrecipe_c4 where user_id = $1 and recipe_id = $2",
        user_id, recipe_id
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // Execute query
    Recipe{
        recipe_id: recipe_row.recipe_id,
        user_id: recipe_row.user_id,
        recipe_name: recipe_row.recipe_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(recipe_row.posted_time.unwrap())),
    }
}

pub async fn post_new_recipe_db(pool: &PgPool, new_recipe: Recipe) -> Recipe {
    let recipe_row = sqlx::query!(
        "insert into yummyrecipe_c4 (recipe_id,user_id, recipe_name)  
                                  values ($1,$2,$3) returning 
                                  user_id, 
                                  recipe_id, 
                                  recipe_name,  
                                  posted_time",
        new_recipe.recipe_id,
        new_recipe.user_id,
        new_recipe.recipe_name
    )
    .fetch_one(pool)
    .await
    .unwrap();
    
    Recipe{
        recipe_id: recipe_row.recipe_id,
        user_id: recipe_row.user_id,
        recipe_name: recipe_row.recipe_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(recipe_row.posted_time.unwrap())),
    }
}