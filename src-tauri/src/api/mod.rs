pub mod calificaciones;
pub mod catalogo;
pub mod estudiantes;
pub mod historial;
pub mod institucion;
pub mod pendiente;
pub mod docente;
pub mod plantillas;
pub mod actas_masivas;
pub mod acta_resumen;
pub mod migracion;
pub mod pdf_estudiantes;

pub mod pgcrp_simple;
pub mod estudiante_pgcrp;
pub mod resumen_final;
pub mod resumen_final_bridge;
pub mod tipos_evaluacion;

pub use pdf_estudiantes::generar_pdf_estudiantes_curso; 