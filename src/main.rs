use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use actix_files::{Files, NamedFile};
use actix_web::{HttpRequest, Result, HttpServer, App, web, HttpResponse, FromRequest};
use actix_web::web::Data;
use serde_derive::{Deserialize, Serialize};

async fn index() -> Result<NamedFile>{
    Ok(NamedFile::open("static/index.html")?)
}

async fn form() -> Result<NamedFile> {
    Ok(NamedFile::open("static/form.html")?)
}

async fn mcu_finder(form: web::Form<Vec<Vec<String>>>, mcus: Data<Mutex<Vec<Mcu>>>) -> HttpResponse {
    // Parse the form data from the request
    let mut mcu_data = HashMap::new();
    println!("{:?}", form.0);
    for vector in form.0 {
        if vector.contains(&("low".to_string())) {
            mcu_data.insert("lowEnergy".to_string(), true);
        } else if !mcu_data.contains_key("lowEnergy") {
            mcu_data.insert("lowEnergy".to_string(), false);
        }
        if vector.contains(&("bluetooth".to_string())) {
            mcu_data.insert("bluetooth".to_string(), true);
        } else if !mcu_data.contains_key("bluetooth") {
            mcu_data.insert("bluetooth".to_string(), false);
        }
        if vector.contains(&("wifi".to_string())) {
            mcu_data.insert("wifi".to_string(), true);
        } else if !mcu_data.contains_key("wifi") {
            mcu_data.insert("wifi".to_string(), false);
        }
        if vector.contains(&("audio".to_string())) {
            mcu_data.insert("audio".to_string(), true);
        } else if !mcu_data.contains_key("audio") {
            mcu_data.insert("audio".to_string(), false);
        }
        if vector.contains(&("multiport".to_string())) {
            mcu_data.insert("multiplePorts".to_string(), true);
        } else if !mcu_data.contains_key("multiplePorts") {
            mcu_data.insert("multiplePorts".to_string(), false);
        }
        if vector.contains(&("highspeed".to_string())) {
            mcu_data.insert("highComputationPower".to_string(), true);
        } else if !mcu_data.contains_key("highComputationPower") {
            mcu_data.insert("highComputationPower".to_string(), false);
        }
    }
    println!("{:?}", &mcu_data);
    let mcus = mcus.lock().unwrap().clone();
    println!("{:?}", &mcus);
    let mcu = find_matching_mcu(&mcus, &mcu_data).unwrap();

    // Return an HTTP response
    HttpResponse::Ok().body(mcu.name.clone())
}

async fn beginner() -> Result<NamedFile> {
    Ok(NamedFile::open("static/beginner.html")?)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Mcu {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "lowEnergy")]
    pub low_energy: bool,
    pub bluetooth: bool,
    pub wifi: bool,
    pub audio: bool,
    #[serde(rename = "multiplePorts")]
    pub multiple_ports: bool,
    #[serde(rename = "highComputationalPower")]
    pub high_computational_power: bool,
}

fn find_matching_mcu(mcus: &[Mcu], requirements: &HashMap<String, bool>) -> Option<Mcu> {
    let mut exact_match: Option<Mcu> = None;
    let mut best_mcu: Option<Mcu> = None;
    let mut least_additional_features = std::usize::MAX;

    for mcu in mcus {
        let mut additional_features = 0;
        let mut all_requirements_matched = true;

        for (requirement, &value) in requirements {
            let mcu_feature = match requirement.as_str() {
                "lowEnergy" => mcu.low_energy,
                "bluetooth" => mcu.bluetooth,
                "wifi" => mcu.wifi,
                "audio" => mcu.audio,
                "multiplePorts" => mcu.multiple_ports,
                "highComputationPower" => mcu.high_computational_power,
                _ => false,
            };

            if mcu_feature == value {
                additional_features += 1;
            } else if value {
                all_requirements_matched = false;
                break;
            }
        }

        if all_requirements_matched {
            if additional_features < least_additional_features {
                least_additional_features = additional_features;
                best_mcu = Some(mcu.clone());
            }
        }

        if additional_features == 6 {
            // Exact match found
            exact_match = Some(mcu.clone());
            break;
        }
    }

    exact_match.or(best_mcu)
}

async fn low() -> Result<NamedFile> {
    Ok(NamedFile::open("static/low.html")?)
}

async fn moderate() -> Result<NamedFile> {
    Ok(NamedFile::open("static/moderate.html")?)
}

async fn high() -> Result<NamedFile> {
    Ok(NamedFile::open("static/high.html")?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file = File::open("mcus.json")?;
    let reader = BufReader::new(file);

    let mcus: Vec<Mcu> = serde_json::from_reader(reader).unwrap();
    let mcus = Data::new(Mutex::new(mcus));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&mcus))
            .service(Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/form", web::get().to(form))
            .route("/mcus/find", web::post().to(mcu_finder))
            .route("/mcus/beginner", web::get().to(beginner))
            .route("/mcus/low", web::get().to(low))
            .route("/mcus/moderate", web::get().to(moderate))
            .route("/mcus/high", web::get().to(high))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}