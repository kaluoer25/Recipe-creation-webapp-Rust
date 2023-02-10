use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;
#[derive(Debug)]
pub struct Recipe{
   pub recipe_id: i32,
   pub user_id: i32,
   pub recipe_name: String,
   pub posted_time: Option<NaiveDateTime>,
}
#[actix_rt::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect(
  "DATABASE_URL is not set in .env file");
  let db_pool = PgPool::connect(&database_url).await.unwrap();
   let recipe_rows = sqlx::query!(
       r#"select recipe_id, user_id, recipe_name, posted_time from
       yummyrecipe_c4 where recipe_id = $1"#,
       1
   )
  .fetch_all(&db_pool)
  .await
  .unwrap();
   let mut recipes_list = vec![];
   for recipe_row in recipe_rows {
    recipes_list.push(Recipe {
           recipe_id: recipe_row.recipe_id,
           user_id: recipe_row.user_id,
           recipe_name: recipe_row.recipe_name,
           posted_time: Some(chrono::NaiveDateTime::from(
           recipe_row.posted_time.unwrap())),
       })
   }
   println!("Recipes = {:?}", recipes_list);
  Ok(())
}