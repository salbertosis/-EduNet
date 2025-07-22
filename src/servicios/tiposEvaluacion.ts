import { invoke } from '@tauri-apps/api/tauri';

export interface TipoEvaluacion {
  id: number;
  codigo: string;
  nombre: string;
}

export interface RespuestaTiposEvaluacion {
  exito: boolean;
  mensaje: string;
  tipos_evaluacion?: TipoEvaluacion[];
}

export const obtenerTiposEvaluacion = async (): Promise<RespuestaTiposEvaluacion> => {
  try {
    console.log('🟡 [SERVICIO] Obteniendo tipos de evaluación...');
    
    const respuesta = await invoke<RespuestaTiposEvaluacion>('obtener_tipos_evaluacion');
    
    console.log('🟡 [SERVICIO] Tipos de evaluación obtenidos:', respuesta);
    return respuesta;
  } catch (error) {
    console.error('💥 [SERVICIO] Error al obtener tipos de evaluación:', error);
    return {
      exito: false,
      mensaje: 'Error al obtener tipos de evaluación',
    };
  }
}; 