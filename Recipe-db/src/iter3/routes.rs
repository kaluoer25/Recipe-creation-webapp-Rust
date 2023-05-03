use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
   cfg.route("/health", web::get().to(health_check_handler));
}

pub fn recipe_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipes")
      .route("/", web::post().to(post_new_recipe))
      .route("/{user_id}", web::get().to(get_recipes_from_user))
      .route("/{user_id}/{recipe_id}", web::get().to(
      get_recipe_details)),
   );
}