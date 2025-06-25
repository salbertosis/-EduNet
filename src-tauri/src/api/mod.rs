pub mod calificaciones;
pub mod catalogo;
pub mod estudiantes;
pub mod historial;
pub mod pendiente;
pub mod docente;
pub mod plantillas;
pub mod actas_masivas;
pub mod acta_resumen;
pub mod migracion;
pub mod pdf_estudiantes;
pub mod pdf_simple;

pub mod pgcrp_simple;
pub mod estudiante_pgcrp;
pub mod resumen_excel;
pub mod crear_plantilla_excel;
pub mod analizar_plantilla;
pub mod agregar_marcadores;

pub use pdf_estudiantes::*;
pub use pdf_estudiantes::generar_pdf_estudiantes_curso; 