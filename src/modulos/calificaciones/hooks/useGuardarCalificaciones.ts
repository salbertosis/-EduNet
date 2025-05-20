import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { CalificacionEstudiante } from '../types';
import { calcularNotaFinal } from '../../../utilidades/calculoNotas';
import { useMensajeGlobal } from '../../../componentes/MensajeGlobalContext';

export function useGuardarCalificaciones() {
    const [loading, setLoading] = useState(false);
    const [exito, setExito] = useState(false);
    const { mostrarMensaje } = useMensajeGlobal();

    const guardarCalificaciones = async (calificaciones: CalificacionEstudiante[], estudianteId: number, periodoId: number) => {
        setLoading(true);
        setExito(false);
        try {
            for (const calif of calificaciones) {
                const calificacionBackend = {
                    ...calif,
                    id_estudiante: estudianteId,
                    id_periodo: periodoId,
                    nota_final: calcularNotaFinal(calif),
                };
                await invoke('guardar_calificacion', { calificacion: calificacionBackend });
            }
            setExito(true);
            mostrarMensaje('Calificaciones guardadas correctamente.', 'exito');
        } catch (err: any) {
            mostrarMensaje(err?.message || 'Error al guardar calificaciones', 'error');
            setExito(false);
        } finally {
            setLoading(false);
        }
    };

    return { guardarCalificaciones, loading, exito };
} 