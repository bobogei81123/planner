
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] _pg_pool: PgPool) -> shuttle_axum::ShuttleAxum {
    todo!()
    // dotenv().ok();
    //
    // sqlx::migrate!()
    //     .run(&pg_pool)
    //     .await
    //     .expect("Failed to run migration");
    //
    // let app = planner_backend::build_app(pg_pool).await;
    // Ok(app.into())
}
