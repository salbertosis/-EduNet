import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from './MensajeGlobalContext';

const CrearPeriodoEscolar = () => {
  const [periodoNombre, setPeriodoNombre] = useState('');
  const [fechaInicio, setFechaInicio] = useState('');
  const [fechaFin, setFechaFin] = useState('');
  const { mostrarMensaje } = useMensajeGlobal();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      // Llamar a la función Rust para crear el período escolar
      const idPeriodo: number = await invoke('crear_periodo_escolar', {
        periodoEscolar: periodoNombre,
        fechaInicio,
        fechaFinal: fechaFin,
      });

      // Establecer el nuevo período como activo
      await invoke('establecer_periodo_activo', { idPeriodo });

      mostrarMensaje(`Período escolar "${periodoNombre}" creado exitosamente y establecido como activo. ID: ${idPeriodo}`, 'exito');
      // Limpiar el formulario
      setPeriodoNombre('');
      setFechaInicio('');
      setFechaFin('');
    } catch (error) {
      console.error('Error al crear el período escolar:', error);
      mostrarMensaje(`Error al crear el período escolar: ${error}`, 'error');
    }
  };

  return (
    <div className="container mx-auto p-4 max-w-2xl">
      <h1 className="text-3xl font-extrabold text-gray-900 dark:text-white mb-6 text-center">Crear Nuevo Período Escolar</h1>
      <form onSubmit={handleSubmit} className="bg-white dark:bg-dark-800 p-8 rounded-xl shadow-lg border border-gray-200 dark:border-dark-700 space-y-6">
        <div className="space-y-2">
          <label htmlFor="periodoNombre" className="block text-gray-700 dark:text-gray-300 text-sm font-medium">
            Nombre del Período:
          </label>
          <input
            type="text"
            id="periodoNombre"
            placeholder="2024-2025"
            className="w-1/2 px-1 py-0.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
            value={periodoNombre}
            onChange={(e) => setPeriodoNombre(e.target.value)}
            required
          />
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="space-y-2">
            <label htmlFor="fechaInicio" className="block text-gray-700 dark:text-gray-300 text-sm font-medium">
              Fecha de Inicio:
            </label>
            <input
              type="date"
              id="fechaInicio"
              className="w-full px-1 py-0.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
              value={fechaInicio}
              onChange={(e) => setFechaInicio(e.target.value)}
              required
            />
          </div>
          <div className="space-y-2">
            <label htmlFor="fechaFin" className="block text-gray-700 dark:text-gray-300 text-sm font-medium">
              Fecha de Fin:
            </label>
            <input
              type="date"
              id="fechaFin"
              className="w-full px-1 py-0.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-emerald-500 focus:border-transparent transition-all"
              value={fechaFin}
              onChange={(e) => setFechaFin(e.target.value)}
              required
            />
          </div>
        </div>
        <div className="flex justify-center">
          <button
            type="submit"
            className="w-1/3 bg-gradient-to-r from-emerald-500 to-teal-600 hover:from-emerald-600 hover:to-teal-700 text-white font-bold py-0.5 px-1 rounded-lg shadow-lg transform transition-all duration-200 ease-in-out hover:scale-105 focus:outline-none focus:ring-2 focus:ring-emerald-500 focus:ring-offset-2 dark:focus:ring-offset-dark-800"
          >
            Crear Período
          </button>
        </div>
      </form>
    </div>
  );
};

export default CrearPeriodoEscolar; 