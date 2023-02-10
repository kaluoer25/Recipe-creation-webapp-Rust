use super::db_access::*;
use super::errors::RecipeError;
use super::models::Recipe;
use super::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, RecipeError> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Ok(HttpResponse::Ok().json(&response))
}

pub async fn get_recipes_from_user(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, RecipeError> {
    let user_id = path.into_inner();
    get_recipes_from_user_db(&app_state.db, user_id)
        .await
        .map(|recipes| HttpResponse::Ok().json(recipes))
}

pub async fn get_recipe_details(
    app_state: web::Data<AppState>,
    path:  web::Path<(i32, i32)>,
) -> Result<HttpResponse, RecipeError> {
    let (user_id, recipe_id) = path.into_inner();
    get_recipe_details_db(&app_state.db, user_id, recipe_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

/* curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
 -d '{"tutor_id":1, "course_name":"Course 1"}'
*/
pub async fn post_new_recipe(
    new_recipe: web::Json<Recipe>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, RecipeError> {
    post_new_recipe_db(&app_state.db, new_recipe.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::{NaiveDate};
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_recipes_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let user_id: web::Path<i32> = web::Path::from(1);
        let resp = get_recipes_from_user(app_state, user_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_recipe_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_recipe_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    ///
    #[ignore]
    #[actix_rt::test]
    async fn post_recipe_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_recipe_msg = Recipe {
            recipe_id: 3,
            user_id: 1,
            recipe_name: "Third course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(7, 01, 11)),
        };
        let recipe_param = web::Json(new_recipe_msg);
        let resp = post_new_recipe(recipe_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}