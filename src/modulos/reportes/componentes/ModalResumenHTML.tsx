import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { save } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/shell';

interface ModalResumenHTMLProps {
    isOpen: boolean;
    onClose: () => void;
    idGradoSecciones: number;
    idTipoEvaluacion: number;
}

interface RespuestaResumenFinal {
    exito: boolean;
    mensaje: string;
    archivo_generado?: string;
    estudiantes_procesados: number;
}

const ModalResumenHTML: React.FC<ModalResumenHTMLProps> = ({ 
    isOpen, 
    onClose, 
    idGradoSecciones,
    idTipoEvaluacion
}) => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [htmlContent, setHtmlContent] = useState<string | null>(null);
    const [paginasHTML, setPaginasHTML] = useState<string[]>([]);
    const [paginaActual, setPaginaActual] = useState(0);

    const generarResumenHTML = async () => {
        if (!idGradoSecciones) {
            setError('ID de grado secciones no válido');
            return;
        }

        setIsLoading(true);
        setError(null);

        try {
            console.log('🔄 Generando resumen final HTML...');
            console.log('📋 Parámetros:', { idGradoSecciones });

            const htmlContent = await invoke<string>('generar_resumen_final_html_directo_v2', {
                params: {
                    idGradoSecciones: idGradoSecciones,
                    idTipoEvaluacion: idTipoEvaluacion
                }
            });

            console.log('✅ HTML generado exitosamente');
            console.log('📄 Tamaño del HTML:', htmlContent.length, 'caracteres');

            // Separar las páginas por el marcador de salto de página
            const paginas = htmlContent.split('<div style=\'page-break-before: always;\'></div>');
            setPaginasHTML(paginas);
            setPaginaActual(0);
            setHtmlContent(paginas[0] || htmlContent);
            
        } catch (err) {
            const errorMsg = err instanceof Error ? err.message : 'Error desconocido';
            setError(`Error al generar resumen: ${errorMsg}`);
            console.error('❌ Error:', err);
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        if (isOpen && idGradoSecciones) {
            generarResumenHTML();
        }
    }, [isOpen, idGradoSecciones]);

    const handleClose = () => {
        setHtmlContent(null);
        setPaginasHTML([]);
        setPaginaActual(0);
        setError(null);
        onClose();
    };

    const cambiarPagina = (nuevaPagina: number) => {
        if (nuevaPagina >= 0 && nuevaPagina < paginasHTML.length) {
            setPaginaActual(nuevaPagina);
            setHtmlContent(paginasHTML[nuevaPagina]);
        }
    };

    const imprimirPagina = () => {
        try {
            // Crear una nueva ventana con el contenido HTML
            const nuevaVentana = window.open('', '_blank', 'width=800,height=600');
            if (nuevaVentana) {
                // Escribir el contenido HTML completo
                nuevaVentana.document.write(htmlContent || '');
                nuevaVentana.document.close();
                
                // Esperar a que se cargue completamente el contenido
                setTimeout(() => {
                    try {
                        nuevaVentana.print();
                        // Cerrar la ventana después de un tiempo
                        setTimeout(() => {
                            nuevaVentana.close();
                        }, 2000);
                    } catch (printError) {
                        console.error('Error al imprimir:', printError);
                        alert('Error al abrir el diálogo de impresión');
                        nuevaVentana.close();
                    }
                }, 1000);
            } else {
                alert('No se pudo abrir la ventana de impresión. Verifica que el bloqueador de ventanas emergentes esté desactivado.');
            }
        } catch (error) {
            console.error('Error al crear ventana de impresión:', error);
            alert('Error al abrir el diálogo de impresión');
        }
    };

    const guardarPDF = async () => {
        try {
            const rutaSalida = await save({
                title: 'Guardar Resumen Final PDF',
                filters: [{
                    name: 'Archivos PDF',
                    extensions: ['pdf']
                }],
                defaultPath: `resumen_final_rendimiento_estudiantil_${idGradoSecciones}.pdf`
            });

            if (rutaSalida) {
                console.log('🔄 Generando PDF...');
                console.log('📋 Parámetros:', { idGradoSecciones, rutaSalida });

                const respuesta = await invoke<RespuestaResumenFinal>('generar_resumen_final_pdf_directo_v2', {
                    params: {
                        idGradoSecciones: idGradoSecciones,
                        idTipoEvaluacion: idTipoEvaluacion,
                        rutaSalida: rutaSalida
                    }
                });

                console.log('✅ Respuesta del backend:', respuesta);

                if (respuesta.exito) {
                    console.log('✅ PDF generado exitosamente');
                    alert(`¡PDF generado exitosamente! ${respuesta.estudiantes_procesados} estudiantes procesados.`);
                } else {
                    console.error('❌ Error del backend:', respuesta.mensaje);
                    alert(`Error: ${respuesta.mensaje}`);
                }
            }
        } catch (err) {
            const errorMsg = err instanceof Error ? err.message : 'Error desconocido';
            console.error('❌ Error guardando PDF:', err);
            alert(`Error al generar PDF: ${errorMsg}`);
        }
    };

    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 bg-black bg-opacity-60 flex items-center justify-center z-50 transition-opacity duration-300">
            <div className="bg-gray-50 dark:bg-gray-800 rounded-xl shadow-2xl w-10/12 h-[90%] flex flex-col max-w-7xl">
                {/* Header */}
                <div className="flex justify-between items-center p-4 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
                    <h2 className="text-xl font-bold text-gray-800 dark:text-gray-100">
                        Vista Previa: Resumen Final
                    </h2>
                    <button
                        onClick={handleClose}
                        className="text-gray-400 dark:text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors rounded-full p-1"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                {/* Content */}
                <div className="flex-1 p-1 sm:p-4 overflow-y-auto bg-white dark:bg-gray-900">
                    {isLoading && (
                        <div className="flex items-center justify-center h-full">
                            <div className="text-center text-gray-500 dark:text-gray-400">
                                <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
                                <p className="text-lg font-semibold">Generando resumen...</p>
                                <p>Por favor, espere un momento.</p>
                            </div>
                        </div>
                    )}

                    {error && (
                        <div className="flex items-center justify-center h-full">
                            <div className="text-center bg-red-50 dark:bg-red-900/20 p-8 rounded-lg">
                                <div className="text-red-500 text-6xl mb-4">⚠️</div>
                                <p className="text-red-700 dark:text-red-300 text-xl font-bold mb-2">Error al Generar</p>
                                <p className="text-gray-600 dark:text-gray-400 max-w-md mx-auto">{error}</p>
                                <button
                                    onClick={generarResumenHTML}
                                    className="mt-6 px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-all duration-300 ease-in-out transform hover:scale-105"
                                >
                                    Reintentar
                                </button>
                            </div>
                        </div>
                    )}

                    {!isLoading && !error && htmlContent && (
                         <iframe
                            srcDoc={htmlContent}
                            title="Resumen Final"
                            className="w-full h-full border-0"
                        />
                    )}
                </div>

                {/* Footer */}
                <div className="p-4 bg-gray-100 dark:bg-gray-800/50 border-t border-gray-200 dark:border-gray-700 flex justify-between items-center flex-shrink-0">
                    {/* Pagination */}
                    <div className="flex items-center gap-2 sm:gap-4">
                        <button
                            onClick={() => cambiarPagina(paginaActual - 1)}
                            disabled={paginaActual === 0 || paginasHTML.length <= 1}
                            className="px-4 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fillRule="evenodd" d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z" clipRule="evenodd" /></svg>
                            <span className="hidden sm:inline">Anterior</span>
                        </button>
                        <span className="text-sm text-gray-600 dark:text-gray-400 font-medium">
                            Página {paginaActual + 1} de {paginasHTML.length}
                        </span>
                        <button
                            onClick={() => cambiarPagina(paginaActual + 1)}
                            disabled={paginaActual >= paginasHTML.length - 1}
                            className="px-4 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-200 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                        >
                            <span className="hidden sm:inline">Siguiente</span>
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fillRule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clipRule="evenodd" /></svg>
                        </button>
                    </div>

                    {/* Action Buttons */}
                    <div className="flex items-center gap-3">
                        <button
                            onClick={imprimirPagina}
                            className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50 flex items-center gap-2 font-semibold transition-colors"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path fillRule="evenodd" d="M5 4v3H4a2 2 0 00-2 2v6a2 2 0 002 2h12a2 2 0 002-2V9a2 2 0 00-2-2h-1V4a2 2 0 00-2-2H7a2 2 0 00-2 2zm8 0H7v3h6V4zm0 8H7v4h6v-4z" clipRule="evenodd" /></svg>
                            <span className="hidden sm:inline">Imprimir</span>
                        </button>
                        <button
                            onClick={guardarPDF}
                            className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 disabled:opacity-50 flex items-center gap-2 font-semibold transition-colors"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"><path d="M10.894 2.894a1 1 0 00-1.414 0l-6 6a1 1 0 000 1.414l6 6a1 1 0 001.414-1.414L6.414 10l4.48-4.48a1 1 0 000-1.414z" clipRule="evenodd" /><path fillRule="evenodd" d="M4 2a2 2 0 00-2 2v12a2 2 0 002 2h12a2 2 0 002-2V4a2 2 0 00-2-2H4zm1 4a1 1 0 100 2h8a1 1 0 100-2H5z" clipRule="evenodd" /></svg>
                            <span className="hidden sm:inline">Guardar PDF</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default ModalResumenHTML;