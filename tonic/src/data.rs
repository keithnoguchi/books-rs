//! RouteGuideService data deserializer.
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

pub fn load() -> Result<Vec<crate::Feature>, Box<dyn std::error::Error>> {
    let file = File::open("./data/route_guide_db.json")?;
    let decoded: Vec<Feature> = serde_json::from_reader(&file)?;
    Ok(decoded
        .into_iter()
        .map(|feature| crate::Feature {
            name: feature.name,
            location: Some(crate::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect())
}
