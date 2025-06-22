import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

const GeneradorPDFSimple: React.FC = () => {
    const [loading, setLoading] = useState(false);
    const [idGradoSecciones, setIdGradoSecciones] = useState(1);
    const [mensaje, setMensaje] = useState('');

    const handleGenerarPDF = async () => {
        setLoading(true);
        setMensaje('');
        
        try {
            const resultado = await invoke<string>('generar_acta_pdf_simple', {
                idGradoSecciones: idGradoSecciones
            });

            // Convertir el base64 a blob y descargar
            const byteCharacters = atob(resultado);
            const byteNumbers = new Array(byteCharacters.length);
            for (let i = 0; i < byteCharacters.length; i++) {
                byteNumbers[i] = byteCharacters.charCodeAt(i);
            }
            const byteArray = new Uint8Array(byteNumbers);
            const blob = new Blob([byteArray], { type: 'application/pdf' });

            // Crear enlace de descarga
            const url = URL.createObjectURL(blob);
            const link = document.createElement('a');
            link.href = url;
            link.download = `acta_resumen_${new Date().getTime()}.pdf`;
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);
            URL.revokeObjectURL(url);

            setMensaje('¡PDF generado y descargado exitosamente! 🎉');
        } catch (error) {
            console.error('Error generando PDF:', error);
            setMensaje(`Error generando PDF: ${error}`);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
            <h2>🚀 Generador de PDF Simplificado</h2>
            <p style={{ color: '#666', marginBottom: '20px' }}>
                Nueva implementación más rápida y eficiente para generar actas de resumen estudiantil.
            </p>

            <div style={{ marginBottom: '20px' }}>
                <label style={{ display: 'block', marginBottom: '8px', fontWeight: 'bold' }}>
                    ID Grado Secciones:
                </label>
                <input
                    type="number"
                    value={idGradoSecciones}
                    onChange={(e) => setIdGradoSecciones(Number(e.target.value))}
                    style={{
                        width: '100%',
                        padding: '8px',
                        border: '1px solid #ddd',
                        borderRadius: '4px',
                        fontSize: '14px'
                    }}
                    disabled={loading}
                />
            </div>

            <button
                onClick={handleGenerarPDF}
                disabled={loading}
                style={{
                    background: loading ? '#ccc' : 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
                    color: 'white',
                    border: 'none',
                    padding: '12px 24px',
                    borderRadius: '8px',
                    fontSize: '16px',
                    fontWeight: 'bold',
                    cursor: loading ? 'not-allowed' : 'pointer',
                    width: '100%',
                    marginBottom: '20px',
                    transition: 'all 0.3s ease'
                }}
            >
                {loading ? '⏳ Generando PDF...' : '📄 Generar Acta PDF Simple'}
            </button>

            {mensaje && (
                <div style={{
                    padding: '12px',
                    borderRadius: '6px',
                    backgroundColor: mensaje.includes('Error') ? '#ffebee' : '#e8f5e8',
                    color: mensaje.includes('Error') ? '#c62828' : '#2e7d32',
                    border: `1px solid ${mensaje.includes('Error') ? '#ef5350' : '#4caf50'}`,
                    fontSize: '14px'
                }}>
                    {mensaje}
                </div>
            )}

            <div style={{ 
                marginTop: '30px', 
                padding: '15px', 
                backgroundColor: '#f8f9fa', 
                borderRadius: '8px',
                border: '1px solid #e9ecef'
            }}>
                <h3 style={{ margin: '0 0 10px 0', color: '#495057' }}>✨ Ventajas de esta implementación:</h3>
                <ul style={{ margin: 0, paddingLeft: '20px', color: '#6c757d' }}>
                    <li>🎯 <strong>Más simple:</strong> Usa HTML + CSS estándar</li>
                    <li>⚡ <strong>Más rápida:</strong> Menos código Rust manual</li>
                    <li>🎨 <strong>Más flexible:</strong> Fácil modificar estilos</li>
                    <li>🔧 <strong>Más mantenible:</strong> HTML es más legible que código PDF manual</li>
                    <li>📱 <strong>Responsive:</strong> Se adapta mejor a diferentes tamaños</li>
                </ul>
            </div>

            <div style={{ 
                marginTop: '20px', 
                padding: '15px', 
                backgroundColor: '#fff3cd', 
                borderRadius: '8px',
                border: '1px solid #ffeaa7'
            }}>
                <h4 style={{ margin: '0 0 10px 0', color: '#856404' }}>💡 Para usar en producción:</h4>
                <ol style={{ margin: 0, paddingLeft: '20px', color: '#856404', fontSize: '14px' }}>
                    <li>Conectar con datos reales de la base de datos</li>
                    <li>Agregar el logo oficial de la institución</li>
                    <li>Ajustar colores y estilos según necesidades</li>
                    <li>Configurar Chrome/Chromium en el servidor</li>
                </ol>
            </div>
        </div>
    );
};

export default GeneradorPDFSimple; 