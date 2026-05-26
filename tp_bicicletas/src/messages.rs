//! Mensajes que viajan por la red entre procesos.
//!
//! Cada struct es el payload de un mensaje. El transporte (TCP / UDP) se
//! detalla en el README. El formato de aplicacion es texto plano simple
//! (`KIND|k=v|k=v\n`), sin crates externos.

use crate::types::*;
use std::time::SystemTime;

// App <-> Estacion

pub struct DiscoverRequest {
    pub ubicacion_usuario: Coord,
    pub radio: f32,
}

pub struct DiscoverResponse {
    pub estaciones: Vec<StationInfo>,
}

pub struct RentRequest {
    pub usuario: UserId,
    pub estacion: StationId,
}

pub struct RentAccepted {
    pub trip_id: TripId,
    pub bike_id: BikeId,
}

pub struct RentRejected {
    pub motivo: String,
}

pub struct ReturnRequest {
    pub usuario: UserId,
    pub bike_id: BikeId,
    pub estacion: StationId,
}

pub struct ReturnAccepted {
    pub trip_id: TripId,
}

pub struct ReturnRejected {
    pub motivo: String,
}

// Estacion <-> Lider y entre estaciones de la misma zona

pub struct TripStarted {
    pub trip_id: TripId,
    pub bike_id: BikeId,
    pub estacion_origen: StationId,
    pub usuario: UserId,
    pub timestamp: SystemTime,
}

pub struct TripCompleted {
    pub trip_id: TripId,
    pub bike_id: BikeId,
    pub estacion_destino: StationId,
    pub timestamp: SystemTime,
}

pub struct SyncPending {
    pub estacion: StationId,
    pub operaciones: Vec<OperacionPendiente>,
}

pub struct RegionalStateUpdate {
    pub zona: Zone,
    pub disponibilidad: Vec<(StationId, u32, u32)>, // (estacion, bicis, libres)
}

pub struct Heartbeat {
    pub estacion: StationId,
}

// Exclusion mutua centralizada (lider como coordinador)

pub struct MutexRequest {
    pub recurso: String,
    pub solicitante: StationId,
}

pub struct MutexOk {
    pub recurso: String,
}

pub struct MutexRelease {
    pub recurso: String,
    pub solicitante: StationId,
}

// Eleccion de lider (Bully)

pub struct Election {
    pub emisor: StationId,
}

pub struct OkElection {
    pub emisor: StationId,
}

pub struct Coordinator {
    pub lider: StationId,
}

// Banco

pub struct PaymentPreauth {
    pub trip_id: TripId,
    pub usuario: UserId,
    pub monto: u32,
    pub timestamp: SystemTime,
}

pub struct PaymentCapture {
    pub trip_id: TripId,
    pub usuario: UserId,
    pub monto: u32,
    pub timestamp: SystemTime,
}

pub struct PaymentAck {
    pub trip_id: TripId,
    pub exito: bool,
}
