import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

interface Grado {
    id: number;
    grado: number;
    seccion: string;
    totalEstudiantes: number;
    estudiantesFemeninos: number;
    estudiantesMasculinos: number;
    docenteGuia: string;
    asignaturas: Array<{
        id: number;
        nombre: string;
        docente: string;
    }>;
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

    const obtenerGrados = useCallback(async (filtros: FiltrosGrados) => {
        setCargando(true);
        setError(null);

        // DATOS DE EJEMPLO
        const ejemplo: Grado[] = [
            {
                id: 1,
                grado: 1,
                seccion: 'A',
                totalEstudiantes: 32,
                estudiantesFemeninos: 18,
                estudiantesMasculinos: 14,
                docenteGuia: 'María Pérez',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'María Pérez' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 2,
                grado: 1,
                seccion: 'B',
                totalEstudiantes: 29,
                estudiantesFemeninos: 15,
                estudiantesMasculinos: 14,
                docenteGuia: 'José Gómez',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'José Gómez' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 3,
                grado: 1,
                seccion: 'C',
                totalEstudiantes: 30,
                estudiantesFemeninos: 16,
                estudiantesMasculinos: 14,
                docenteGuia: 'Ana Torres',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'Ana Torres' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 4,
                grado: 1,
                seccion: 'D',
                totalEstudiantes: 28,
                estudiantesFemeninos: 14,
                estudiantesMasculinos: 14,
                docenteGuia: 'Carlos Ruiz',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'Carlos Ruiz' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 5,
                grado: 1,
                seccion: 'E',
                totalEstudiantes: 31,
                estudiantesFemeninos: 17,
                estudiantesMasculinos: 14,
                docenteGuia: 'Laura Sánchez',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'Laura Sánchez' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 6,
                grado: 1,
                seccion: 'F',
                totalEstudiantes: 27,
                estudiantesFemeninos: 13,
                estudiantesMasculinos: 14,
                docenteGuia: 'Pedro López',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'Pedro López' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            },
            {
                id: 7,
                grado: 1,
                seccion: 'G',
                totalEstudiantes: 30,
                estudiantesFemeninos: 16,
                estudiantesMasculinos: 14,
                docenteGuia: 'Carmen Rodríguez',
                asignaturas: [
                    { id: 1, nombre: 'Lengua y Literatura', docente: 'Carmen Rodríguez' },
                    { id: 2, nombre: 'Matemática', docente: 'Carlos Ruiz' },
                    { id: 3, nombre: 'Ciencias de la Tierra', docente: 'Ana Torres' },
                ]
            }
        ];

        setTimeout(() => {
            setGrados(ejemplo);
            setPaginacion({
                paginaActual: 1,
                totalPaginas: 1,
                totalRegistros: ejemplo.length,
                registrosPorPagina: 12
            });
            setCargando(false);
        }, 500); // Simula carga
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