use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};

use axum::{Json, response::IntoResponse, Router, routing::{get, post}};
use axum::body::{Body, boxed};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use chrono::DateTime;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use trusty_gardener_model::{Plant, PlantWateringHistory, WateringEvent};

#[derive(Clone)]
struct AppState {
    plants: HashMap<String, Plant>,
    watering_histories: HashMap<String, PlantWateringHistory>
}

type SharedAppState = State<Arc<RwLock<AppState>>>;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(AppState {
        plants: HashMap::new(),
        watering_histories: HashMap::new(),
    }));
    let app = Router::new()
        .route(
            "/api/plant",
            get(plants_handler)
                .put(create_plant_handler)
        )
        .route("/api/plant/:name/watering", post(watering_handler))
        .route(
            "/api/plant/:name/watering_history",
            get(watering_history_handler)
                .delete(delete_watering_history_handler)
        )
        .fallback_service(get(|req| async move {
            match ServeDir::new(String::from("dist")).oneshot(req).await {
                Ok(res) => res.map(boxed),
                Err(err) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Body::from(format!("error: {err}"))))
                    .expect("error response"),
            }
        }))
        .with_state(state);

    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9090);



    println!("listening on http://{}", socket_address);

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}

async fn plants_handler(State(state): SharedAppState) -> Json<Vec<Plant>> {
    let state = state.read().unwrap();

    Json(state.plants.values().cloned().collect())
}

async fn create_plant_handler(State(state): SharedAppState, Json(plant): Json<Plant>) -> Response {
    println!("Created new plant: {:?}", plant.name);
    let mut state = state.write().unwrap();
    state.watering_histories.insert(Clone::clone(&plant.name), PlantWateringHistory::default());
    state.plants.insert(Clone::clone(&plant.name), plant);
    StatusCode::CREATED.into_response()
}

async fn watering_handler(Path(name): Path<String>, State(state): SharedAppState) {
    println!("Watering {:?}", name);
    let mut state = state.write().unwrap();
    let event = WateringEvent {
        timestamp: DateTime::default()
    };
    match state.watering_histories.get_mut(&name) {
        None => {
            state.watering_histories.insert(name, PlantWateringHistory {
                history: vec![event]
            });
        }
        Some(watering_history) => {
            watering_history.history.push(event)
        }
    }
}

async fn watering_history_handler(Path(name): Path<String>, State(state): SharedAppState) -> Response {
    let state = state.read().unwrap();
    match state.watering_histories.get(&name) {
        None => {
            (StatusCode::NOT_FOUND, format!("Unknown Plant: {name}")).into_response()
        }
        Some(watering_history) => {
            (StatusCode::OK, Json(watering_history)).into_response()
        }
    }
}

async fn delete_watering_history_handler(Path(name): Path<String>, State(state): SharedAppState) -> Response {
    let mut state = state.write().unwrap();
    match state.watering_histories.get_mut(&name) {
        None => {
            (StatusCode::NOT_FOUND, format!("Unknown Plant: {name}")).into_response()
        }
        Some(watering_history) => {
            watering_history.history.clear();
            StatusCode::OK.into_response()
        }
    }
}
