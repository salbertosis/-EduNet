//! Módulo de generación de resumen final del rendimiento estudiantil
//! 
//! Este módulo maneja la generación de reportes de resumen final según las 
//! especificaciones del MPPE de Venezuela.

// Re-exports públicos
pub use service::ResumenFinalService;

// Módulos internos
mod service;
pub mod generators;
pub mod processors;
pub mod models;
pub mod utils;

// Re-exports específicos para conveniencia
pub use generators::*;
pub use processors::*;
pub use models::*;
pub use utils::constants::*; 