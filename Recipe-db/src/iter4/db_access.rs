use super::errors::RecipeError;
use super::models::Recipe;
use sqlx::postgres::PgPool;

pub async fn get_recipes_from_user_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Recipe>, RecipeError> {
    // Prepare SQL statement
    let recipe_rows = sqlx::query!(
        "SELECT user_id, recipe_id, recipe_name, posted_time FROM yummyrecipe_c5 where user_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await?;
    // Extract result

    let recipes: Vec<Recipe> = recipe_rows
        .iter()
        .map(|recipe_row| Recipe {
            recipe_id: recipe_row.recipe_id,
            user_id: recipe_row.user_id,
            recipe_name: recipe_row.recipe_name.clone(),
            posted_time: Some(recipe_row.posted_time.unwrap()),
        })
        .collect();
    match recipes.len() {
        0 => Err(RecipeError::NotFound(
            "Recipe not found for tutor".into(),
        )),
        _ => Ok(recipes),
    }
}

//Return result

pub async fn get_recipe_details_db(pool: &PgPool, user_id: i32, recipe_id: i32) -> Result<Recipe, RecipeError> {
    // Prepare SQL statement
    let recipe_row = sqlx::query!(
        "SELECT user_id, recipe_id, recipe_name, posted_time FROM yummyrecipe_c5 where user_id = $1 and recipe_id = $2",
        user_id, recipe_id
    )
    .fetch_one(pool)
    .await;
    if let Ok(recipe_row) = recipe_row {     
    // Execute query
    Ok(Recipe {
        recipe_id: recipe_row.recipe_id,
        user_id: recipe_row.user_id,
        recipe_name: recipe_row.recipe_name.clone(),
        posted_time: Some(recipe_row.posted_time.unwrap()),
    })
} else {
    Err(RecipeError::NotFound("Recipe id not found".into()))
}
}

pub async fn post_new_recipe_db(pool: &PgPool, new_recipe: Recipe) -> Result<Recipe, RecipeError> {
    let recipe_row = sqlx::query!("insert into yummyrecipe_c5 (recipe_id, user_id, recipe_name) values ($1,$2,$3) returning user_id, recipe_id, recipe_name, posted_time", new_recipe.recipe_id, new_recipe.user_id, new_recipe.recipe_name)
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Recipe {
        recipe_id: recipe_row.recipe_id,
        user_id: recipe_row.user_id,
        recipe_name: recipe_row.recipe_name.clone(),
        posted_time: Some(recipe_row.posted_time.unwrap()),
    })
}