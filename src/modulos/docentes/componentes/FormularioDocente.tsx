import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

interface Docente {
  id_docente?: number;
  cedula: string;
  apellidos: string;
  nombres: string;
  especialidad: string;
  telefono?: string;
  correoelectronico?: string;
}

interface FormularioDocenteProps {
  docente?: Docente;
  onGuardar: () => void;
  onCancelar: () => void;
}

export function FormularioDocente({ docente, onGuardar, onCancelar }: FormularioDocenteProps) {
  const [formData, setFormData] = useState<Partial<Docente>>({
    cedula: '',
    apellidos: '',
    nombres: '',
    especialidad: '',
    telefono: '',
    correoelectronico: '',
  });
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { mostrarMensaje } = useMensajeGlobal();

  useEffect(() => {
    if (docente) {
      setFormData(docente);
    }
  }, [docente]);

  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => {
        setSuccess(null);
        onGuardar();
      }, 2000);
      return () => clearTimeout(timer);
    }
  }, [success]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: name === 'cedula' ? value.replace(/[^0-9]/g, '') : value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError(null);
    setSuccess(null);
    try {
      const cleanData = {
        cedula: formData.cedula ? Number(formData.cedula) : undefined,
        apellidos: formData.apellidos,
        nombres: formData.nombres,
        especialidad: formData.especialidad,
        telefono: formData.telefono || null,
        correoelectronico: formData.correoelectronico || null,
      };
      console.log('Enviando datos al backend:', cleanData);
      if (docente?.id_docente) {
        console.log('Actualizando docente con id:', docente.id_docente);
        const resp = await invoke('actualizar_docente', {
          idDocente: docente.id_docente,
          docente: cleanData,
        });
        console.log('Respuesta actualizar_docente:', resp);
        mostrarMensaje('Docente actualizado correctamente', 'exito');
        setSuccess('Docente actualizado correctamente');
      } else {
        console.log('Creando nuevo docente');
        const resp = await invoke('crear_docente', {
          docente: cleanData,
        });
        console.log('Respuesta crear_docente:', resp);
        mostrarMensaje('Docente creado correctamente', 'exito');
        setSuccess('Docente creado correctamente');
      }
    } catch (err) {
      console.error('Error al guardar docente:', err);
      mostrarMensaje('Error al guardar el docente. Por favor, intente nuevamente.', 'error');
      setError('Error al guardar el docente. Por favor, intente nuevamente.');
    }
    setIsSubmitting(false);
  };

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-6">
      {success && (
        <div className="mb-4 p-3 rounded-lg bg-green-100 border border-green-300 text-green-800 text-center font-semibold animate-fade-in">
          {success}
        </div>
      )}
      {error && (
        <div className="mb-4 p-3 rounded-lg bg-red-100 border border-red-300 text-red-800 text-center font-semibold">
          {error}
        </div>
      )}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Cédula</label>
          <input
            type="text"
            name="cedula"
            value={formData.cedula || ''}
            onChange={handleChange}
            required
            placeholder="Ej: 12345678"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Apellidos</label>
          <input
            type="text"
            name="apellidos"
            value={formData.apellidos || ''}
            onChange={handleChange}
            required
            placeholder="Ej: RAMOS ROJAS"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Nombres</label>
          <input
            type="text"
            name="nombres"
            value={formData.nombres || ''}
            onChange={handleChange}
            required
            placeholder="Ej: PEDRO JOSE"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Especialidad</label>
          <input
            type="text"
            name="especialidad"
            value={formData.especialidad || ''}
            onChange={handleChange}
            required
            placeholder="Ej: Educación Física"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Teléfono</label>
          <input
            type="text"
            name="telefono"
            value={formData.telefono || ''}
            onChange={handleChange}
            placeholder="Ej: 04141234567"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
        <div className="space-y-2">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Correo electrónico</label>
          <input
            type="email"
            name="correoelectronico"
            value={formData.correoelectronico || ''}
            onChange={handleChange}
            placeholder="Ej: docente@email.com"
            className="w-full px-4 py-2.5 rounded-lg border border-gray-300 dark:border-dark-600 bg-white dark:bg-dark-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-all"
          />
        </div>
      </div>
      <div className="flex justify-end gap-4 mt-6">
        <button
          type="button"
          onClick={onCancelar}
          className="px-5 py-2 rounded-lg bg-gray-200 text-gray-700 hover:bg-gray-300 font-medium"
        >
          Cancelar
        </button>
        <button
          type="submit"
          disabled={isSubmitting}
          className="px-5 py-2 rounded-lg bg-emerald-600 text-white hover:bg-emerald-700 font-semibold shadow flex items-center gap-2"
        >
          {isSubmitting ? 'Guardando…' : docente ? 'Actualizar' : 'Guardar'}
        </button>
      </div>
    </form>
  );
} 