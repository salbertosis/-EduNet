//! Generadores de contenido para el resumen final

pub mod tablas_generator;
pub mod estudiantes_generator;
pub mod paginas_generator;

// Re-exports
pub use tablas_generator::TablasGenerator;
pub use estudiantes_generator::EstudiantesGenerator;
pub use paginas_generator::PaginasGenerator; 