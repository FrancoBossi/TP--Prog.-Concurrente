//! App de usuario.
//!
//! Reenvia intenciones del usuario (consulta, retiro, devolucion) a una
//! estacion cercana. No es fuente de verdad: la estacion es quien valida
//! y registra.

use crate::config::AppConfig;
use crate::types::*;

pub struct App {
    pub config: AppConfig,
    pub usuario: UserId,
    pub ubicacion: Coord,
    pub viaje_en_curso: Option<(TripId, BikeId, StationId)>,
}

pub fn run(config: AppConfig) {
    let app = App {
        config,
        usuario: "u-demo".to_string(),
        ubicacion: (0.0, 0.0),
        viaje_en_curso: None,
    };
    println!(
        "[app] arrancando. usuario={} estaciones_conocidas={}",
        app.usuario,
        app.config.estaciones.len()
    );
    // En la implementacion: leer input del usuario (stdin) y enviar
    // DiscoverRequest / RentRequest / ReturnRequest por TCP a la estacion
    // mas cercana del radio.
}
