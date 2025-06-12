import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../componentes/MensajeGlobalContext';

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
      mostrarMensaje(`Período escolar "${periodoNombre}" creado exitosamente con ID: ${idPeriodo}`, 'exito');
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
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Crear Nuevo Período Escolar</h1>
      <form onSubmit={handleSubmit} className="bg-white dark:bg-dark-800 p-6 rounded-lg shadow-md">
        <div className="mb-4">
          <label htmlFor="periodoNombre" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">
            Nombre del Período:
          </label>
          <input
            type="text"
            id="periodoNombre"
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
            value={periodoNombre}
            onChange={(e) => setPeriodoNombre(e.target.value)}
            required
          />
        </div>
        <div className="mb-4">
          <label htmlFor="fechaInicio" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">
            Fecha de Inicio:
          </label>
          <input
            type="date"
            id="fechaInicio"
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
            value={fechaInicio}
            onChange={(e) => setFechaInicio(e.target.value)}
            required
          />
        </div>
        <div className="mb-6">
          <label htmlFor="fechaFin" className="block text-gray-700 dark:text-gray-300 text-sm font-bold mb-2">
            Fecha de Fin:
          </label>
          <input
            type="date"
            id="fechaFin"
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline bg-gray-100 dark:bg-dark-700 border-gray-300 dark:border-dark-600"
            value={fechaFin}
            onChange={(e) => setFechaFin(e.target.value)}
            required
          />
        </div>
        <button
          type="submit"
          className="bg-emerald-600 hover:bg-emerald-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
        >
          Crear Período
        </button>
      </form>
    </div>
  );
};

export default CrearPeriodoEscolar; 