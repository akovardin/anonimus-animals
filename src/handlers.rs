use axum::{
    Json, 
    extract::State, 
    http::StatusCode, 
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use sqlx::{Pool, Sqlite, any};
use chrono::{Utc, Duration};

use crate::{generator::AnimalNameGenerator, models::Animal};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub generator: AnimalNameGenerator,
}

pub async fn home() -> &'static str {
    "Anonimus Animals"
}

#[derive(Deserialize, Serialize)]
pub struct UpdateRequest {
    pub id: i64
}

pub async fn update(State(state): State<AppState>, Json(json): Json<UpdateRequest>,) -> impl IntoResponse {
    let res = sqlx::query("UPDATE animals SET updated_at = $1 WHERE id = $2")
        .bind(Utc::now())
        .bind(json.id)
        .execute(&state.db)
        .await;

    match res {
        Ok(res) => {
            let animal = sqlx::query_as::<_, Animal>("SELECT id, name, image, created_at, updated_at FROM animals WHERE id = $1")
                .bind(json.id)
                .fetch_one(&state.db)
                .await;

            match animal {
                Ok(animal) => {
                    return (StatusCode::OK, Json(json!(animal))).into_response();
                }
                Err(animal) => {
                    println!("{}", animal);

                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", animal)).into_response();
                }
            }

        }
        Err(res) => {
            println!("{}", res);

            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", res)).into_response();
        }
    }
}

pub async fn animals(State(state): State<AppState>) -> impl IntoResponse {
    let duration = Utc::now() -  Duration::minutes(1);

    let query = "SELECT id, name, image, created_at, updated_at FROM animals WHERE updated_at > $1 LIMIT 5";
    let res = sqlx::query_as::<_, Animal>(query)
        .bind(duration)
        .fetch_all(&state.db)
        .await;

    match res {
        Ok(res) => {
            return (StatusCode::OK, Json(json!(res))).into_response();
        }
        Err(res) => {
            println!("{}", res);

            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", res)).into_response();
        }
    }
}

pub async fn add(State(state): State<AppState>) -> impl IntoResponse {
    let name = state.generator.generate();
    let parts: Vec<&str> = name.split_whitespace().collect();
    let image = format!("{}_.png", parts[1]);

    let res = sqlx::query("INSERT INTO animals (name, image, created_at, updated_at) VALUES ($1, $2, $3, $4)")
        .bind(name)
        .bind(image)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&state.db)
        .await;

    match res {
        Ok(res) => {
            let id = res.last_insert_rowid();

            let animal = sqlx::query_as::<_, Animal>("SELECT id, name, image, created_at, updated_at FROM animals WHERE id = ?")
                .bind(id)
                .fetch_one(&state.db)
                .await;

            match animal {
                Ok(animal) => {
                    return (StatusCode::OK, Json(json!(animal))).into_response();
                }
                Err(animal) => {
                    println!("{}", animal);

                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", animal)).into_response();
                }
            }
        }
        Err(res) => {
            println!("{}", res);

            return (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", res)).into_response();
        }
    }
}
