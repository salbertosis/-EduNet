import { useState } from 'react';
import { useValidacionesCalificaciones } from './useValidacionesCalificaciones';

interface CalificacionEstudiante {
  id_calificacion?: number;
  id_asignatura: number;
  nombre_asignatura: string;
  lapso_1?: number;
  lapso_1_ajustado?: number;
  lapso_2?: number;
  lapso_2_ajustado?: number;
  lapso_3?: number;
  lapso_3_ajustado?: number;
  nota_final?: number;
  revision?: string;
}

interface Asignatura {
  id_asignatura: number;
  nombre_asignatura: string;
  id_grado: number;
  id_modalidad: number;
}

export function useInputCalificaciones(
  asignaturas: Asignatura[],
  calificaciones: CalificacionEstudiante[],
  setCalificaciones: (calificaciones: CalificacionEstudiante[]) => void
) {
  const { validarInput, actualizarError, errores, limpiarErrores } = useValidacionesCalificaciones();

  const handleInputChange = (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string) => {
    const idx = calificaciones.findIndex(c => c.id_asignatura === id_asignatura);
    let updated = [...calificaciones];
    let calif = idx === -1 ? {
      id_asignatura,
      nombre_asignatura: asignaturas.find(a => a.id_asignatura === id_asignatura)?.nombre_asignatura || '',
    } as CalificacionEstudiante : { ...updated[idx] };

    const { valor: num, error } = validarInput(id_asignatura, campo, valor, calif);
    
    // Asignar el valor solo si es number o undefined
    (calif as any)[campo] = num;
    
    if (idx === -1) {
      updated.push(calif);
    } else {
      updated[idx] = calif;
    }

    actualizarError(id_asignatura, campo, error);
    setCalificaciones(updated);
  };

  return {
    handleInputChange,
    errores,
    limpiarErrores
  };
} 