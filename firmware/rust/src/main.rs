// Create Web API for my Raspberry Pi Gesture Recognition project

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_cors::Cors;
use serde::{Serialize};
use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use log::{debug, error};
use std::fs::File;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[get("/start_gesture_recognition")]
async fn start_gesture_recognition() -> impl Responder {
    
    // Call the gesture recognition script
    let output = std::process::Command::new("sudo")
        .arg("python")
        .arg("~/Documents/Projects/Raspberry-Pi-Gesture-Controlled-Led-System/firmware/python/gesture_recognition/gesture_recognition.py")
        .output()
        .expect("failed to execute process");
    
    // Check if the gesture recognition script was executed successfully
    if output.status.success() {
        debug!("Gesture Recognition Started");
    } else {
        error!("Gesture Recognition Failed");
    }

    let response = Response {
        message: "Gesture Recognition Started".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Instantiate Logger */
    match CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            File::create("gesture_recognition.log").unwrap(),
        ),
    ]) {
        Ok(_) => debug!("Logger initialized"),
        Err(e) => debug!("Logger failed to initialize: {}", e),
    }
        HttpServer::new(move || {
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header();
    
            App::new()
                .wrap(cors)
                .service(healthcheck)
                .service(start_gesture_recognition)
                .default_service(web::route().to(not_found))
        })
        .bind(("127.0.0.1", 8084))?
        .run()
        .await
    
}