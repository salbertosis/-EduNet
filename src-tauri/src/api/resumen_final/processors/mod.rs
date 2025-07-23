//! Procesadores para el resumen final

pub mod html_processor;
pub mod pdf_processor;
pub mod data_processor;

// Re-exports
pub use html_processor::HtmlProcessor;
pub use pdf_processor::PdfProcessor;
pub use data_processor::DataProcessor; 