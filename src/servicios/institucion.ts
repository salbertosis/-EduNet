import { invoke } from '@tauri-apps/api/tauri';

export interface DatosInstitucion {
  codigo: string;
  denominacion: string;
  direccion: string;
  telefono?: string;
  municipio: string;
  entidad_federal: string;
  cdcee?: string;
  director: string;
  cedula_director: string;
}

export interface RespuestaInstitucion {
  exito: boolean;
  mensaje: string;
  datos?: DatosInstitucion;
}

export const obtenerDatosInstitucion = async (): Promise<RespuestaInstitucion> => {
  try {
    const respuesta = await invoke<RespuestaInstitucion>('obtener_datos_institucion');
    return respuesta;
  } catch (error) {
    console.error('Error obteniendo datos de la institución:', error);
    return {
      exito: false,
      mensaje: 'Error al obtener los datos de la institución',
    };
  }
};

export const guardarDatosInstitucion = async (datos: DatosInstitucion): Promise<RespuestaInstitucion> => {
  try {
    console.log('🟡 [SERVICIO] Iniciando llamada a Tauri...');
    console.log('🟡 [SERVICIO] Datos enviados:', datos);
    console.log('🟡 [SERVICIO] Tipo de datos:', typeof datos);
    console.log('🟡 [SERVICIO] Comando a invocar: guardar_datos_institucion');
    
    const respuesta = await invoke<RespuestaInstitucion>('guardar_datos_institucion', {
      institucion: datos
    });
    
    console.log('🟡 [SERVICIO] Respuesta de Tauri recibida:', respuesta);
    console.log('🟡 [SERVICIO] Tipo de respuesta:', typeof respuesta);
    
    if (!respuesta) {
      console.log('❌ [SERVICIO] Respuesta es undefined');
      return {
        exito: false,
        mensaje: 'No se recibió respuesta del servidor',
      };
    }
    
    return respuesta;
  } catch (error) {
    console.error('💥 [SERVICIO] Error en llamada a Tauri:', error);
    console.error('💥 [SERVICIO] Tipo de error:', typeof error);
    console.error('💥 [SERVICIO] Mensaje de error:', error instanceof Error ? error.message : String(error));
    return {
      exito: false,
      mensaje: 'Error al guardar los datos de la institución',
    };
  }
}; 