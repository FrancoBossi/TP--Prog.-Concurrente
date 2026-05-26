//! Tipos del dominio compartidos por todas las entidades.

use std::time::SystemTime;

pub type StationId = String;
pub type BikeId = String;
pub type UserId = String;
pub type TripId = String;
pub type Zone = String;
pub type Coord = (f32, f32);

pub enum EstadoBicicleta {
    DisponibleEnEstacion,
    EnViaje,
    DevueltaPendienteDeSync,
    PosiblementePerdida,
}

pub enum EstadoCobro {
    Pendiente,
    Enviado,
    Confirmado,
}

pub enum EstadoSync {
    Sincronizado,
    Pendiente,
}

pub struct Bicicleta {
    pub id: BikeId,
    pub estado: EstadoBicicleta,
}

pub struct Viaje {
    pub id: TripId,
    pub id_bicicleta: BikeId,
    pub estacion_origen: StationId,
    pub usuario: UserId,
    pub horario_inicio: SystemTime,
    pub estado_cobro: EstadoCobro,
    pub estado_sync: EstadoSync,
}

pub struct StationInfo {
    pub id: StationId,
    pub ip: String,
    pub puerto: u16,
    pub ubicacion: Coord,
    pub zona: Zone,
    pub capacidad_max: u32,
}

pub enum OperacionPendiente {
    Retiro { trip_id: TripId, bike_id: BikeId, user_id: UserId, ts: SystemTime },
    Devolucion { trip_id: TripId, bike_id: BikeId, ts: SystemTime },
    Cobro { trip_id: TripId, user_id: UserId, monto: u32 },
}
