//! Banco (pasarela de pagos simulada).
//!
//! Recibe PaymentPreauth / PaymentCapture, registra la operacion y
//! responde PaymentAck. No modela saldos ni rechazos: el foco esta en la
//! comunicacion, no en logica bancaria.

use crate::types::TripId;
use std::collections::HashMap;

pub struct Bank {
    pub operaciones: HashMap<TripId, OperacionBanco>,
}

pub enum OperacionBanco {
    Preautorizada,
    Cobrada(u32),
}

pub fn run() {
    let _bank = Bank {
        operaciones: HashMap::new(),
    };
    println!("[bank] arrancando pasarela simulada en 127.0.0.1:9100");
    // En la implementacion: TcpListener::bind, por cada conexion leer un
    // mensaje, actualizar operaciones y responder PaymentAck.
}
