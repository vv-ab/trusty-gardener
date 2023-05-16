use futures::TryFutureExt;
use reqwasm::http::{Request, Response};

use trusty_gardener_model::{Plant, PlantWateringHistory};

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum ApiError {

    #[error("Unknown plant: {plant}")]
    UnknownPlant {
        plant: String
    },

    #[error("ParsingFailure: {message}")]
    ParsingFailure { message: String },

    #[error("Failed to request")]
    RequestFailure
}

pub async fn get_plants() -> Result<Vec<Plant>, ApiError> {
    let plants_endpoint = "/api/plant";
    let response = Request::get(&plants_endpoint)
        .send()
        .map_err(|_| ApiError::RequestFailure)
        .await?;
    match response.status() {
        200 => {
            response
                .json::<Vec<Plant>>()
                .map_err(|cause| { ApiError::ParsingFailure { message: cause.to_string() } })
                .await
        }
        _ => {
            Err(ApiError::RequestFailure)
        }
    }
}

pub async fn clear_watering_history(name: String) -> Result<(), ApiError> {
    let plants_endpoint = format!("/api/plant/{}/watering_history", name);
    let response = Request::delete(&plants_endpoint)
        .send()
        .map_err(|_| ApiError::RequestFailure)
        .await?;
    match response.status() {
        200 => {
            Ok(())
        }
        _ => {
            Err(ApiError::UnknownPlant {
                plant: name
            })
        },
    }
}

pub async fn do_watering(name: String) -> Result<(), ApiError> {
    let plants_endpoint = format!("/api/plant/{}/watering", name);
    let response = Request::post(&plants_endpoint)
        .send()
        .map_err(|_| ApiError::RequestFailure)
        .await?;
    match response.status() {
        200 => {
            Ok(())
        }
        _ => {
            Err(ApiError::UnknownPlant {
                plant: name
            })
        },
    }
}

pub async fn get_watering_history(name: String) -> Result<PlantWateringHistory, ApiError> {
    let plants_endpoint = format!("/api/plant/{}/watering_history", name);
    let response: Response = Request::get(&plants_endpoint)
        .send()
        .map_err(|_| ApiError::RequestFailure)
        .await?;
    match response.status() {
        200 => {
            response
                .json::<PlantWateringHistory>()
                .map_err(|cause| { ApiError::ParsingFailure { message: cause.to_string() } })
                .await
        },
        404 => {
            Err(ApiError::UnknownPlant {
                plant: name
            })
        }
        _ => {
            Err(ApiError::RequestFailure)
        }
    }
}
