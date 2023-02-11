use super::db_access::*;
use super::models::Recipe;
use super::state::AppState;


use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_recipes_from_user(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tuple = params.0;
    let user_id: i32 = tuple;
    let recipes = get_recipes_from_user_db(&app_state.db, user_id).await;
    HttpResponse::Ok().json(recipes)
}

pub async fn get_recipe_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse { /*
    let tuple = params;
    let user_id: i32 = tuple.0;
    let recipe_id: i32 = tuple.1; */
    let (user_id, recipe_id) = (params.0,params.1);
    let recipe = get_recipe_details_db(&app_state.db, user_id, recipe_id).await;
    HttpResponse::Ok().json(recipe)
}

/* curl -X POST localhost:3000/courses/ \
-H "Content-Type: application/json" \
 -d '{"user_id":1, "recipe_name":"Fries and coke"}'
*/
pub async fn post_new_recipe(
    new_recipe: web::Json<Recipe>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let recipe = post_new_recipe_db(&app_state.db, new_recipe.into()).await;

    HttpResponse::Ok().json(recipe)
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
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
        let user_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_recipes_from_user(app_state, user_id).await;
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
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_recipe_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

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
            posted_time: Some(NaiveDate::from_ymd(2023, 01, 18).and_hms(07, 40, 00)),
        };
        let recipe_param = web::Json(new_recipe_msg);
        let resp = post_new_recipe(recipe_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}