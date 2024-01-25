use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use lm_sensors::Initializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SensorData {
    chip: String,
    path: String,
    features: HashMap<String, Vec<SubFeatureData>>,
}

#[derive(Serialize, Deserialize)]
struct SubFeatureData {
    name: String,
    value: String,
}

async fn get_temperatures() -> impl Responder {
    let sensors = Initializer::default().initialize().unwrap();
    let mut data = Vec::new();

    for chip in sensors.chip_iter(None) {
        let chip_name = chip.name().unwrap_or_else(|_| "N/A".to_string());
        let chip_path = chip.path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "N/A".to_string());

        let mut feature_map = HashMap::new();

        for feature in chip.feature_iter() {
            let feature_name = feature.name().transpose().unwrap_or(Some("N/A"))
                .unwrap_or("N/A").to_string();

            let mut sub_features = Vec::new();
            for sub_feature in feature.sub_feature_iter() {
                let sub_feature_name = sub_feature.name()
                    .unwrap_or(Ok("N/A"))
                    .unwrap_or("N/A")
                    .to_string();
                let value = sub_feature.value().map_or("N/A".to_string(), |v| v.to_string());

                sub_features.push(SubFeatureData {
                    name: sub_feature_name,
                    value,
                });
            }

            feature_map.insert(feature_name, sub_features);
        }

        data.push(SensorData {
            chip: chip_name,
            path: chip_path,
            features: feature_map,
        });
    }

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/temperatures", web::get().to(get_temperatures))
    })
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
