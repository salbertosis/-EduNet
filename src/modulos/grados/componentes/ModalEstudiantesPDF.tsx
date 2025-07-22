import React, { useState } from 'react';
import { X, Loader2, FileText } from 'lucide-react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';
import { ModalPDF } from '../../../componentes/ModalPDF';

interface ModalEstudiantesPDFProps {
  isOpen: boolean;
  onClose: () => void;
  idGradoSecciones: number;
  nombreGrado: string;
  nombreSeccion: string;
}

export const ModalEstudiantesPDF: React.FC<ModalEstudiantesPDFProps> = ({
  isOpen,
  onClose,
  idGradoSecciones,
  nombreGrado,
  nombreSeccion
}) => {
  const { mostrarMensaje } = useMensajeGlobal();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);
  const [showPdf, setShowPdf] = useState(false);

  const generarPDF = async () => {
    if (!isOpen) return;
    
    setLoading(true);
    setError(null);
    setPdfUrl(null);
    setShowPdf(false);

    try {
      console.log('[ModalEstudiantesPDF] Generando PDF para:', {
        idGradoSecciones,
        nombreGrado,
        nombreSeccion
      });

      const pdfBase64 = await invoke<string>('generar_pdf_estudiantes_curso', {
        idGradoSecciones: idGradoSecciones
      });

      console.log('[ModalEstudiantesPDF] PDF generado exitosamente');
      
      // Convertir base64 a blob URL
      const pdfBytes = Uint8Array.from(atob(pdfBase64), c => c.charCodeAt(0));
      const blob = new Blob([pdfBytes], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      
      setPdfUrl(url);
      setShowPdf(true);
      mostrarMensaje('PDF de estudiantes generado exitosamente', 'exito');
      
    } catch (err: any) {
      console.error('[ModalEstudiantesPDF] Error generando PDF:', err);
      const errorMsg = err?.message || 'Error al generar el PDF de estudiantes';
      setError(errorMsg);
      mostrarMensaje(errorMsg, 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleClose = () => {
    if (pdfUrl) {
      URL.revokeObjectURL(pdfUrl);
      setPdfUrl(null);
    }
    setShowPdf(false);
    setError(null);
    onClose();
  };

  // Generar PDF automáticamente cuando se abre el modal
  React.useEffect(() => {
    if (isOpen && !loading && !pdfUrl) {
      generarPDF();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <>
      {/* Modal de confirmación */}
      <div className="fixed inset-0 flex items-center justify-center bg-black/50 backdrop-blur-sm z-50 p-4">
        <div className="bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-md">
          <div className="flex items-center justify-between p-6 border-b border-gray-200 dark:border-slate-700">
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
              Generar PDF de Estudiantes
            </h2>
            <button
              type="button"
              onClick={handleClose}
              className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors"
              aria-label="Cerrar modal"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
          
          <div className="p-6">
            <div className="text-center space-y-4">
              <div className="flex items-center justify-center w-16 h-16 mx-auto bg-blue-100 dark:bg-blue-900/30 rounded-full">
                <FileText className="w-8 h-8 text-blue-600 dark:text-blue-400" />
              </div>
              
              <div>
                <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
                  {nombreGrado} Año {nombreSeccion}
                </h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Se generará un PDF con el listado completo de estudiantes de esta sección.
                </p>
              </div>

              {loading && (
                <div className="flex items-center justify-center gap-2 text-blue-600 dark:text-blue-400">
                  <Loader2 className="w-5 h-5 animate-spin" />
                  <span>Generando PDF...</span>
                </div>
              )}

              {error && (
                <div className="text-center py-4">
                  <div className="text-red-600 dark:text-red-400 text-sm mb-2">
                    {error}
                  </div>
                  <button
                    onClick={generarPDF}
                    className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
                  >
                    Reintentar
                  </button>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Modal del PDF */}
      <ModalPDF
        open={showPdf}
        onClose={handleClose}
        pdfUrl={pdfUrl}
        loading={loading}
        error={error}
      />
    </>
  );
}; 