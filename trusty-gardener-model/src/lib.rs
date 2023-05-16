use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Plant {
    pub name: String,
    pub species: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PlantWateringHistory {
    pub history: Vec<WateringEvent>
}

impl Default for PlantWateringHistory {
    fn default() -> Self {
        PlantWateringHistory {
            history: Vec::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WateringEvent {
    pub timestamp: DateTime<Utc>
}
