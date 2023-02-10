use super::state::AppState;
use actix_web::{web, HttpResponse};

use super::models::Recipe;
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_recipe(
    new_recipe: web::Json<Recipe>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new food recipe creation");
    let recipe_count_for_user = app_state
        .recipes
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|recipe| recipe.user_id == new_recipe.user_id)
       .count();
     
    let new_recipe = Recipe {
        user_id: new_recipe.user_id,
        recipe_id: Some((recipe_count_for_user + 1).try_into().unwrap()),
        recipe_name: new_recipe.recipe_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.recipes.lock().unwrap().push(new_recipe);
    HttpResponse::Ok().json("Added recipe!")
}

pub async fn get_recipes_from_user(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    
    let user_id: i32 = params.into_inner();

    let filtered_recipes = app_state
        .recipes
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|recipe| recipe.user_id == user_id)
        .collect::<Vec<Recipe>>();

    if filtered_recipes.len() > 0 {
        HttpResponse::Ok().json(filtered_recipes)
    } else {
        HttpResponse::Ok().json("No recipe found for user".to_string())
    }
}

pub async fn get_recipe_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (user_id, recipe_id) = params.into_inner();
    let selected_recipe = app_state
        .recipes
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.user_id == user_id && x.recipe_id == Some(recipe_id.try_into().unwrap()))
        .ok_or("Recipe not found");

    if let Ok(recipe) = selected_recipe {
        HttpResponse::Ok().json(recipe)
    } else {
        HttpResponse::Ok().json("Recipe not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_recipe_test() {
        let recipe = web::Json(Recipe{
            user_id: 1,
            recipe_name: "You have entered a recipe :)".into(),
            recipe_id: None,
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            recipes: Mutex::new(vec![]),
        });
        let resp = new_recipe(recipe, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_recipes_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            recipes: Mutex::new(vec![]),
        });
        let tutor_id: web::Path<i32> = web::Path::from(1);
        let resp = get_recipes_from_user(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_recipe_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            recipes: Mutex::new(vec![]),
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_recipe_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}