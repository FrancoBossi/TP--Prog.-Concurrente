//! Carga de configuracion desde archivos planos `key = value`.

use crate::types::*;
use std::fs;
use std::io;

pub struct StationConfig {
    pub id: StationId,
    pub ip: String,
    pub puerto: u16,
    pub ubicacion: Coord,
    pub zona: Zone,
    pub capacidad_max: u32,
    pub vecinos: Vec<StationId>,
    pub banco: (String, u16),
}

pub struct AppConfig {
    pub estaciones: Vec<StationInfo>,
    pub banco: (String, u16),
}

impl StationConfig {
    pub fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(parse_station(&content).unwrap_or_else(Self::default_demo))
    }

    pub fn default_demo() -> Self {
        Self {
            id: "S1".to_string(),
            ip: "127.0.0.1".to_string(),
            puerto: 9001,
            ubicacion: (0.0, 0.0),
            zona: "Z1".to_string(),
            capacidad_max: 10,
            vecinos: vec!["S2".to_string(), "S3".to_string()],
            banco: ("127.0.0.1".to_string(), 9100),
        }
    }
}

impl AppConfig {
    pub fn from_file(_path: &str) -> io::Result<Self> {
        Ok(Self::default_demo())
    }

    pub fn default_demo() -> Self {
        Self {
            estaciones: vec![StationInfo {
                id: "S1".to_string(),
                ip: "127.0.0.1".to_string(),
                puerto: 9001,
                ubicacion: (0.0, 0.0),
                zona: "Z1".to_string(),
                capacidad_max: 10,
            }],
            banco: ("127.0.0.1".to_string(), 9100),
        }
    }
}

fn parse_station(content: &str) -> Option<StationConfig> {
    let mut id = None;
    let mut ip = None;
    let mut puerto = None;
    let mut x = 0.0_f32;
    let mut y = 0.0_f32;
    let mut zona = None;
    let mut cap = None;
    let mut vecinos = Vec::new();
    let mut banco_host = "127.0.0.1".to_string();
    let mut banco_port = 9100_u16;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line.split_once('=')?;
        let k = k.trim();
        let v = v.trim();
        match k {
            "id" => id = Some(v.to_string()),
            "ip" => ip = Some(v.to_string()),
            "puerto" => puerto = v.parse().ok(),
            "ubicacion_x" => x = v.parse().unwrap_or(0.0),
            "ubicacion_y" => y = v.parse().unwrap_or(0.0),
            "zona" => zona = Some(v.to_string()),
            "capacidad_max" => cap = v.parse().ok(),
            "vecinos" => vecinos = v.split(',').map(|s| s.trim().to_string()).collect(),
            "banco_host" => banco_host = v.to_string(),
            "banco_puerto" => banco_port = v.parse().unwrap_or(9100),
            _ => {}
        }
    }

    Some(StationConfig {
        id: id?,
        ip: ip?,
        puerto: puerto?,
        ubicacion: (x, y),
        zona: zona?,
        capacidad_max: cap?,
        vecinos,
        banco: (banco_host, banco_port),
    })
}
