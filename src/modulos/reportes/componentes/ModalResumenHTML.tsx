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

            const htmlContent = await invoke<string>('generar_resumen_final_html_directo', {
                idGradoSecciones,
                idTipoEvaluacion
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

                const respuesta = await invoke<RespuestaResumenFinal>('generar_resumen_final_pdf_directo', {
                    idGradoSecciones,
                    idTipoEvaluacion,
                    rutaSalida
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
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div className="bg-white rounded-lg shadow-xl w-11/12 h-5/6 flex flex-col">
                {/* Header */}
                <div className="flex justify-between items-center p-4 border-b">
                    <h2 className="text-xl font-semibold text-gray-800">
                        Resumen Final Rendimiento Estudiantil
                    </h2>
                    <button
                        onClick={handleClose}
                        className="text-gray-500 hover:text-gray-700 text-2xl"
                    >
                        ×
                    </button>
                </div>

                {/* Content */}
                <div className="flex-1 p-4 overflow-hidden">
                    {isLoading && (
                        <div className="flex items-center justify-center h-full">
                            <div className="text-center">
                                <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
                                <p className="text-gray-600">Generando resumen HTML...</p>
                            </div>
                        </div>
                    )}

                    {error && (
                        <div className="flex items-center justify-center h-full">
                            <div className="text-center">
                                <div className="text-red-500 text-6xl mb-4">⚠️</div>
                                <p className="text-red-600 font-semibold mb-2">Error</p>
                                <p className="text-gray-600">{error}</p>
                                <button
                                    onClick={generarResumenHTML}
                                    className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                                >
                                    Reintentar
                                </button>
                            </div>
                        </div>
                    )}

                    {htmlContent && !isLoading && !error && (
                        <div className="h-full flex flex-col">
                            <div className="flex justify-between items-center mb-4">
                                <div className="text-sm text-gray-600">
                                    <span>Página {paginaActual + 1} de {paginasHTML.length} - {htmlContent.length} caracteres</span>
                                </div>
                                <div className="flex gap-2">
                                    {/* Navegación entre páginas */}
                                    {paginasHTML.length > 1 && (
                                        <>
                                            <button
                                                onClick={() => cambiarPagina(paginaActual - 1)}
                                                disabled={paginaActual === 0}
                                                className="px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700 disabled:opacity-50"
                                            >
                                                ← Anterior
                                            </button>
                                            <button
                                                onClick={() => cambiarPagina(paginaActual + 1)}
                                                disabled={paginaActual === paginasHTML.length - 1}
                                                className="px-3 py-1 bg-gray-600 text-white rounded text-sm hover:bg-gray-700 disabled:opacity-50"
                                            >
                                                Siguiente →
                                            </button>
                                        </>
                                    )}
                                    
                                    {/* Botones de acción */}
                                    <button
                                        onClick={imprimirPagina}
                                        className="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700"
                                    >
                                        🖨️ Imprimir
                                    </button>
                                    <button
                                        onClick={guardarPDF}
                                        className="px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
                                    >
                                        📄 Guardar PDF
                                    </button>
                                </div>
                            </div>
                            
                            {/* Iframe para mostrar el HTML */}
                            <div className="flex-1 border rounded overflow-hidden">
                                <iframe
                                    srcDoc={htmlContent}
                                    className="w-full h-full"
                                    title="Resumen Final HTML"
                                    sandbox="allow-same-origin allow-scripts"
                                    style={{ border: 'none' }}
                                />
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default ModalResumenHTML; 