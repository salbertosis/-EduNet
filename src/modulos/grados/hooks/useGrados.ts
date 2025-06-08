import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export interface Grado {
    id_grado_secciones: number;
    id_grado: number;
    id_seccion: number;
    id_modalidad: number;
    nombre_grado: string;
    nombre_seccion: string;
    nombre_modalidad: string;
    docente_guia: string;
    total_estudiantes: number;
    estudiantes_femeninos: number;
    estudiantes_masculinos: number;
}

interface PaginacionInfo {
    paginaActual: number;
    totalPaginas: number;
    totalRegistros: number;
    registrosPorPagina: number;
}

interface FiltrosGrados {
    busqueda: string;
    grado: string;
    seccion: string;
}

export const useGrados = () => {
    const [grados, setGrados] = useState<Grado[]>([]);
    const [cargando, setCargando] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [paginacion, setPaginacion] = useState<PaginacionInfo>({
        paginaActual: 1,
        totalPaginas: 1,
        totalRegistros: 0,
        registrosPorPagina: 12
    });

    const obtenerGrados = useCallback(async () => {
        setCargando(true);
        setError(null);
        try {
            const data = await invoke<Grado[]>('obtener_tarjetas_cursos');
            setGrados(data);
        } catch (e: any) {
            setError(e.toString());
        } finally {
            setCargando(false);
        }
    }, []);

    const cambiarPagina = useCallback((nuevaPagina: number) => {
        setPaginacion(prev => ({
            ...prev,
            paginaActual: nuevaPagina
        }));
    }, []);

    const cambiarRegistrosPorPagina = useCallback((nuevosRegistros: number) => {
        setPaginacion(prev => ({
            ...prev,
            registrosPorPagina: nuevosRegistros,
            paginaActual: 1
        }));
    }, []);

    return {
        grados,
        cargando,
        error,
        paginacion,
        obtenerGrados,
        cambiarPagina,
        cambiarRegistrosPorPagina
    };
}; 