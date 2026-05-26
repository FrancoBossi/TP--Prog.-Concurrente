//! Punto de entrada. Un solo binario que se lanza como Estacion, App o Banco
//! segun el primer argumento de linea de comandos.
//!
//! Uso:
//!   tp_bicicletas station <config>
//!   tp_bicicletas app     <config>
//!   tp_bicicletas bank

#![allow(dead_code)]

mod bank;
mod config;
mod messages;
mod station;
mod types;
mod user_app;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("uso: {} <station|app|bank> [config]", args[0]);
        process::exit(1);
    }

    match args[1].as_str() {
        "station" => {
            let path = args.get(2).map(String::as_str).unwrap_or("config/station.cfg");
            let cfg = config::StationConfig::from_file(path)
                .unwrap_or_else(|_| config::StationConfig::default_demo());
            station::run(cfg);
        }
        "app" => {
            let path = args.get(2).map(String::as_str).unwrap_or("config/app.cfg");
            let cfg = config::AppConfig::from_file(path)
                .unwrap_or_else(|_| config::AppConfig::default_demo());
            user_app::run(cfg);
        }
        "bank" => bank::run(),
        other => {
            eprintln!("entidad desconocida: {}", other);
            process::exit(1);
        }
    }
}
