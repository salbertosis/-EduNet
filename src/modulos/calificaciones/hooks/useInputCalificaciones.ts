import { useState } from 'react';
import { useValidacionesCalificaciones } from './useValidacionesCalificaciones';
import { calcularNotaFinal } from '../../../utilidades/calculoNotas';

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
  setCalificaciones: ((calificaciones: CalificacionEstudiante[]) => void) | ((updater: (prev: CalificacionEstudiante[]) => CalificacionEstudiante[]) => void)
) {
  const { validarInput, actualizarError, errores, limpiarErrores } = useValidacionesCalificaciones();

  // Callback inmutable y profesional para actualizar calificaciones
  const handleInputChange = (id_asignatura: number, campo: keyof CalificacionEstudiante, valor: string | number) => {
    (setCalificaciones as (updater: (prev: CalificacionEstudiante[]) => CalificacionEstudiante[]) => void)((prev: CalificacionEstudiante[]) => {
      const idx = prev.findIndex((c: CalificacionEstudiante) => c.id_asignatura === id_asignatura);
      let updated = [...prev];
      let calif: CalificacionEstudiante = idx === -1 ? {
        id_asignatura,
        nombre_asignatura: asignaturas.find(a => a.id_asignatura === id_asignatura)?.nombre_asignatura || '',
      } as CalificacionEstudiante : { ...updated[idx] };

      // Convertir valor a string para validarInput
      const { valor: num, error } = validarInput(id_asignatura, campo, String(valor), calif);
      (calif as any)[campo] = num;

      // Si el campo modificado es un lapso o ajuste, recalcula nota_final
      if ([
        'lapso_1', 'lapso_2', 'lapso_3',
        'lapso_1_ajustado', 'lapso_2_ajustado', 'lapso_3_ajustado'
      ].includes(campo)) {
        calif.nota_final = calcularNotaFinal(calif);
      }

      if (idx === -1) {
        updated.push(calif);
      } else {
        updated[idx] = calif;
      }
      actualizarError(id_asignatura, campo, error);
      return updated;
    });
  };

  return {
    handleInputChange,
    errores,
    limpiarErrores
  };
} 