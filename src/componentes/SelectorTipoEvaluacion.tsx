import React, { useState, useEffect } from 'react';
import { obtenerTiposEvaluacion, TipoEvaluacion } from '../servicios/tiposEvaluacion';

interface SelectorTipoEvaluacionProps {
  value: number | null;
  onChange: (value: number | null) => void;
  className?: string;
}

export function SelectorTipoEvaluacion({ value, onChange, className = '' }: SelectorTipoEvaluacionProps) {
  const [tiposEvaluacion, setTiposEvaluacion] = useState<TipoEvaluacion[]>([]);
  const [cargando, setCargando] = useState(false);

  useEffect(() => {
    const cargarTiposEvaluacion = async () => {
      try {
        setCargando(true);
        const respuesta = await obtenerTiposEvaluacion();
        
        if (respuesta.exito && respuesta.tipos_evaluacion) {
          setTiposEvaluacion(respuesta.tipos_evaluacion);
        } else {
          console.error('Error al cargar tipos de evaluación:', respuesta.mensaje);
          setTiposEvaluacion([]);
        }
      } catch (error) {
        console.error('Error al cargar tipos de evaluación:', error);
        setTiposEvaluacion([]);
      } finally {
        setCargando(false);
      }
    };

    cargarTiposEvaluacion();
  }, []);

  return (
    <select
      value={value || ''}
      onChange={(e) => onChange(e.target.value ? Number(e.target.value) : null)}
      className={`px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:border-emerald-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 ${className}`}
      disabled={cargando}
    >
      <option value="">Seleccionar tipo de evaluación</option>
      {tiposEvaluacion.map((tipo) => (
        <option key={tipo.id} value={tipo.id}>
          {tipo.nombre}
        </option>
      ))}
    </select>
  );
} 