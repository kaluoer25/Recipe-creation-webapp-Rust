use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
   cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/recipes")
      .route("/", web::post().to(post_new_recipe))
      .route("/{tutor_id}", web::get().to(get_recipes_from_user))
      .route("/{tutor_id}/{course_id}", web::get().to(
      get_recipe_details)),
   );
}