//! Estacion: entidad central del sistema.
//!
//! Administra las bicicletas que tiene fisicamente, valida retiros y
//! devoluciones contra su estado local, y sincroniza con el lider de zona
//! y con el banco. Internamente se va a implementar con modelo de actores
//! (ver README), pero en este esqueleto solo se deja el struct de estado
//! y un `run` que arranca el proceso.

use crate::config::StationConfig;
use crate::types::*;
use std::collections::HashMap;

pub struct Station {
    pub config: StationConfig,
    pub bicicletas: HashMap<BikeId, Bicicleta>,
    pub viajes_activos: HashMap<TripId, Viaje>,
    pub operaciones_pendientes: Vec<OperacionPendiente>,
    pub lider_actual: Option<StationId>,
}

impl Station {
    pub fn new(config: StationConfig) -> Self {
        Self {
            config,
            bicicletas: HashMap::new(),
            viajes_activos: HashMap::new(),
            operaciones_pendientes: Vec::new(),
            lider_actual: None,
        }
    }
}

pub fn run(config: StationConfig) {
    let station = Station::new(config);
    println!(
        "[station {}] arrancando en {}:{} (zona {}, vecinos {:?})",
        station.config.id,
        station.config.ip,
        station.config.puerto,
        station.config.zona,
        station.config.vecinos
    );
    // En la implementacion: levantar listener TCP/UDP, arrancar los actores
    // internos (inventario, red, eleccion, mutex, pagos, persistencia) y
    // entrar al loop principal.
}
