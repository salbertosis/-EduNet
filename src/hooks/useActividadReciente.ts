import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export interface Actividad {
  id: number;
  tipo_actividad: string;
  descripcion: string;
  usuario?: string;
  metadata?: string;
  timestamp: string;
  id_estudiante?: number;
  id_docente?: number;
  id_periodo?: number;
  tipo?: string; // Para compatibilidad con el componente
}

export interface NuevaActividad {
  tipo_actividad: string;
  descripcion: string;
  usuario?: string;
  metadata?: string;
  id_estudiante?: number;
  id_docente?: number;
  id_periodo?: number;
}

export function useActividadReciente() {
  const [actividades, setActividades] = useState<Actividad[]>([]);
  const [cargando, setCargando] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const obtenerActividades = async (limite: number = 10) => {
    try {
      setCargando(true);
      setError(null);
      const resultado = await invoke<Actividad[]>('obtener_actividad_reciente', { limite });
      setActividades(resultado);
    } catch (err) {
      setError(err as string);
      console.error('Error obteniendo actividades:', err);
    } finally {
      setCargando(false);
    }
  };

  const registrarActividad = async (nuevaActividad: NuevaActividad) => {
    try {
      setError(null);
      const resultado = await invoke<Actividad>('registrar_actividad', { nuevaActividad });
      
      // Agregar la nueva actividad al inicio de la lista
      setActividades(prev => [resultado, ...prev.slice(0, 9)]);
      
      return resultado;
    } catch (err) {
      setError(err as string);
      console.error('Error registrando actividad:', err);
      throw err;
    }
  };

  const limpiarActividadesAntiguas = async () => {
    try {
      setError(null);
      const resultado = await invoke<number>('limpiar_actividad_antigua');
      console.log(`Se limpiaron ${resultado} actividades antiguas`);
      return resultado;
    } catch (err) {
      setError(err as string);
      console.error('Error limpiando actividades antiguas:', err);
      throw err;
    }
  };

  // Función helper para registrar actividades comunes
  const registrarActividadComun = async (
    tipo: string,
    descripcion: string,
    usuario?: string,
    metadata?: string
  ) => {
    const nuevaActividad: NuevaActividad = {
      tipo_actividad: tipo,
      descripcion,
      usuario,
      metadata
    };
    
    return await registrarActividad(nuevaActividad);
  };

  // Función para registrar actividad de estudiante
  const registrarActividadEstudiante = async (
    tipo: string,
    descripcion: string,
    idEstudiante: number,
    usuario?: string,
    metadata?: string
  ) => {
    const nuevaActividad: NuevaActividad = {
      tipo_actividad: tipo,
      descripcion,
      usuario,
      metadata,
      id_estudiante: idEstudiante
    };
    
    return await registrarActividad(nuevaActividad);
  };

  // Función para registrar actividad de docente
  const registrarActividadDocente = async (
    tipo: string,
    descripcion: string,
    idDocente: number,
    usuario?: string,
    metadata?: string
  ) => {
    const nuevaActividad: NuevaActividad = {
      tipo_actividad: tipo,
      descripcion,
      usuario,
      metadata,
      id_docente: idDocente
    };
    
    return await registrarActividad(nuevaActividad);
  };

  // Función para registrar actividad de período
  const registrarActividadPeriodo = async (
    tipo: string,
    descripcion: string,
    idPeriodo: number,
    usuario?: string,
    metadata?: string
  ) => {
    const nuevaActividad: NuevaActividad = {
      tipo_actividad: tipo,
      descripcion,
      usuario,
      metadata,
      id_periodo: idPeriodo
    };
    
    return await registrarActividad(nuevaActividad);
  };

  return {
    actividades,
    cargando,
    error,
    obtenerActividades,
    registrarActividad,
    registrarActividadComun,
    registrarActividadEstudiante,
    registrarActividadDocente,
    registrarActividadPeriodo,
    limpiarActividadesAntiguas
  };
} 