import { useState } from 'react';

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

export function useValidacionesCalificaciones() {
  const [errores, setErrores] = useState<Record<string, string>>({});

  const validarInput = (
    id_asignatura: number,
    campo: keyof CalificacionEstudiante,
    valor: string,
    calif: CalificacionEstudiante
  ): { valor: number | undefined; error: string } => {
    let num: number | undefined = valor === '' ? undefined : Number(valor);
    let error = '';

    // Validar campos numéricos
    if (['lapso_1', 'lapso_2', 'lapso_3', 'lapso_1_ajustado', 'lapso_2_ajustado', 'lapso_3_ajustado'].includes(campo) && num !== undefined) {
      if (num > 20) {
        num = 20;
        error = 'La calificación máxima es 20';
      }
      if (num < 0) {
        num = 0;
        error = 'La calificación mínima es 0';
      }
    }

    // Validar ajustes
    if (campo.includes('ajustado')) {
      const lapsoCampo = campo.replace('_ajustado', '') as keyof CalificacionEstudiante;
      const lapsoValorRaw = calif[lapsoCampo];
      const lapsoValor = typeof lapsoValorRaw === 'number' ? lapsoValorRaw : 0;
      
      if (num !== undefined) {
        if (num < lapsoValor) {
          error = 'El ajuste no puede ser menor que la nota original';
          num = lapsoValor;
        }
        if (num > lapsoValor + 2) {
          error = 'El ajuste no puede ser mayor que la nota original +2';
          num = lapsoValor + 2;
        }
      }
    }

    return { valor: num, error };
  };

  const actualizarError = (id_asignatura: number, campo: keyof CalificacionEstudiante, error: string) => {
    setErrores(prev => ({
      ...prev,
      [`${id_asignatura}_${campo}`]: error
    }));
  };

  const limpiarErrores = () => {
    setErrores({});
  };

  const tieneErrores = () => {
    return Object.keys(errores).length > 0;
  };

  return {
    errores,
    validarInput,
    actualizarError,
    limpiarErrores,
    tieneErrores
  };
} 