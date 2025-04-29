mod entities;

use entities::{prelude::*, *};

use axum_macros::debug_handler;
use axum::{
    extract::{Path, State},
    http::{HeaderValue, Method, StatusCode}, 
    routing::{get, post}, Error, 
    Router,
    Json,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use sea_orm::*;
use serde::Deserialize;
use migration::{Migrator, MigratorTrait};

const DATABASE_URL: &str = "sqlite:./budget.db?mode=rwc";

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    tracing_subscriber::fmt::init();

    // Setup "database"
    let db = Database::connect(DATABASE_URL).await.expect("Database connection failed");
    Migrator::up(&db, None).await.unwrap();
    
    // Build app
    let app = Router::new()
        .route("/categories", get(get_category).post(add_new_category))
        .route("/categories/", get(get_category).post(add_new_category))
        .route("/purchases", get(get_purchase).post(add_new_purchase))
        .route("/purchases/", get(get_purchase).post(add_new_purchase))
        .route("/categories/{id}", get(get_category_by_id))
        .route("/purchases/{id}", get(get_purchase_by_id))
        .with_state(db)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_headers(vec![axum::http::header::CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    // Bind to address/port and run
    let listener = tokio::net::TcpListener::bind("localhost:5000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_category(
    State(db): State<DatabaseConnection>
) -> (StatusCode, Json<Vec<category::Model>>) {
    let categories: Vec<category::Model> = Category::find()
        .all(&db)
        .await
        .unwrap();

    (StatusCode::OK, Json(categories))
}

async fn get_purchase(
    State(db): State<DatabaseConnection>
) -> (StatusCode, Json<Vec<purchase::Model>>) {
    let purchases: Vec<purchase::Model> = Purchase::find()
        .order_by_desc(purchase::Column::Date)
        .order_by_desc(purchase::Column::Id)
        .all(&db)
        .await
        .unwrap();

    (StatusCode::OK, Json(purchases))
}

async fn get_category_by_id(
    Path(category_id): Path<i32>,
    State(db): State<DatabaseConnection>
) -> Result<(StatusCode, Json<category::Model>), StatusCode> {
    let category = Category::find_by_id(category_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn get_purchase_by_id(
    Path(purchase_id): Path<i32>,
    State(db): State<DatabaseConnection>
) -> Result<(StatusCode, Json<purchase::Model>), StatusCode> {
    let purchase = Purchase::find_by_id(purchase_id)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(purchase)))
}

#[debug_handler]
async fn add_new_category(
    State(db): State<DatabaseConnection>,
    form: Json<NewCategory>
) -> (StatusCode, Json<category::Model>) {
    let new_category = category::ActiveModel {
        name: ActiveValue::Set(form.name.to_owned()),
        budget: ActiveValue::Set(form.budget),
        ..Default::default()
    };

    let res = Category::insert(new_category).exec(&db).await.unwrap();

    let inserted = Category::find_by_id(res.last_insert_id)
        .one(&db)
        .await.unwrap().unwrap();
    
    (StatusCode::OK, Json(inserted))
}

async fn add_new_purchase(
    State(db): State<DatabaseConnection>,
    form: Json<NewPurchase>
) -> (StatusCode, Json<purchase::Model>) {
    let cat = Category::find()
        .filter(category::Column::Name.eq(&form.category))
        .one(&db)
        .await.unwrap().unwrap();

    let new_purchase = purchase::ActiveModel {
        desc: ActiveValue::Set(form.desc.to_owned()),
        amount: ActiveValue::Set(form.amount),
        date: ActiveValue::Set(form.date.to_owned()),
        cat_id: ActiveValue::Set(cat.id),
        ..Default::default()
    };

    let res = Purchase::insert(new_purchase).exec(&db).await.unwrap();

    let inserted = Purchase::find_by_id(res.last_insert_id)
        .one(&db)
        .await.unwrap().unwrap();
    
    (StatusCode::OK, Json(inserted))
}

#[derive(Deserialize)]
struct NewCategory {
    name: String,
    budget: Option<i32>,
}

#[derive(Deserialize)]
struct NewPurchase {
    desc: String,
    amount: i32,
    date: String,
    category: String,
}
