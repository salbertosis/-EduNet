import React, { useState, useEffect } from 'react';
import { Paginacion } from '../../../componentes/Paginacion';
import { useGrados, Grado } from '../hooks/useGrados';
import { GraduationCap, Users, User, Search, BookOpen, Award, Feather, ScrollText } from 'lucide-react';

interface FiltrosGrados {
    busqueda: string;
    grado: string;
    seccion: string;
    modalidad: string;
}

interface TarjetaCursoProps {
    grado: Grado;
}

const IconoGrado: React.FC<{ idGrado: number }> = ({ idGrado }) => {
    const IconComponent = (() => {
        switch (idGrado) {
            case 1: return GraduationCap;
            case 2: return BookOpen;
            case 3: return Award;
            case 4: return Feather;
            case 5: return ScrollText;
            default: return GraduationCap;
        }
    })();
    return <IconComponent className="w-7 h-7 text-white" />;
};

const TarjetaCurso: React.FC<TarjetaCursoProps> = ({ grado }) => (
    <div className="relative rounded-2xl border border-cyan-200 dark:border-cyan-800 shadow-2xl shadow-cyan-100/40 dark:shadow-cyan-900/40 bg-white dark:bg-[#232c3d] p-6 flex flex-col hover:scale-105 hover:shadow-emerald-400/30 transition-transform duration-200">
        {/* Gradiente superior */}
        <div className="h-2 w-full rounded-t-2xl bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 mb-3"></div>
        {/* Icono destacado */}
        <div className="flex items-center justify-center w-12 h-12 rounded-full bg-gradient-to-br from-cyan-400 to-blue-500 shadow-lg mb-2 mx-auto">
            <IconoGrado idGrado={grado.id_grado} />
        </div>
        {/* Encabezado */}
        <span className="font-extrabold text-2xl text-gray-900 dark:text-white mb-1 text-center">{grado.nombre_grado} Año {grado.nombre_seccion}</span>
        {/* Modalidad */}
        <span className="text-sm text-cyan-600 dark:text-cyan-300 mb-2 text-center">{grado.nombre_modalidad}</span>
        {/* Docente guía */}
        <span className="text-xs text-gray-500 dark:text-gray-400 mb-2 text-center">Docente Guía: <b className="text-cyan-600 dark:text-cyan-300">{grado.docente_guia}</b></span>
        {/* Estadísticas */}
        <div className="flex items-center justify-center gap-4 my-2">
            <span className="flex items-center gap-1 text-blue-600 dark:text-blue-400 text-sm bg-blue-50 dark:bg-blue-900/30 px-2 py-1 rounded-full font-medium"><Users className="w-4 h-4" /> {grado.total_estudiantes}</span>
            <span className="flex items-center gap-1 text-pink-500 dark:text-pink-400 text-sm bg-pink-50 dark:bg-pink-900/30 px-2 py-1 rounded-full font-medium"><User className="w-4 h-4" /> {grado.estudiantes_femeninos}</span>
            <span className="flex items-center gap-1 text-blue-700 dark:text-blue-500 text-sm bg-blue-100 dark:bg-blue-900/30 px-2 py-1 rounded-full font-medium"><User className="w-4 h-4" /> {grado.estudiantes_masculinos}</span>
        </div>
        {/* Botón de detalles */}
        <div className="flex justify-end mt-4">
            <button className="flex items-center gap-2 px-4 py-2 rounded-lg bg-blue-100 text-blue-700 border border-blue-200 shadow hover:bg-blue-200 transition dark:bg-blue-900 dark:text-blue-200 dark:border-blue-700 dark:hover:bg-blue-800">
                <Search className="w-5 h-5" />
                Ver detalles
            </button>
        </div>
    </div>
);

const AÑOS = ["1er Año", "2do Año", "3er Año", "4to Año", "5to Año"];
const MODALIDADES = [
    { value: 1, label: "Ciencias" },
    { value: 2, label: "Telemática" },
];

// Función para normalizar strings (quitar tildes y pasar a minúsculas)
function normalizar(str: string) {
    return str.normalize('NFD').replace(/[00-\u036f]/g, '').toLowerCase();
}

export const ListaGrados: React.FC = () => {
    const [añoActivo, setAñoActivo] = useState(0);
    const [modalidad, setModalidad] = useState(1);

    const { 
        grados, 
        cargando, 
        error,
        obtenerGrados
    } = useGrados();

    useEffect(() => {
        obtenerGrados();
    }, []);

    // Filtrar tarjetas según año y modalidad seleccionada (por id_modalidad)
    const gradosFiltrados = grados.filter(
        (g) =>
            g.id_grado === añoActivo + 1 &&
            g.id_modalidad === modalidad &&
            (añoActivo !== 0 || ["A","B","C","D","E","F","G"].includes(g.nombre_seccion))
    ).sort((a, b) => {
        if (añoActivo === 0) {
            // Ordenar secciones de la A a la G
            const orden = ["A","B","C","D","E","F","G"];
            return orden.indexOf(a.nombre_seccion) - orden.indexOf(b.nombre_seccion);
        }
        return 0;
    });

    return (
        <div className="p-6">
            {/* Tabs y selector de modalidad */}
            <div className="flex items-center justify-between mb-4">
                <div className="flex gap-2">
                    {AÑOS.map((año, idx) => (
                        <button
                            key={año}
                            className={`px-4 py-2 rounded-t-lg font-semibold transition-all duration-200 focus:outline-none
                                ${añoActivo === idx
                                    ? "bg-gradient-to-r from-teal-400 to-blue-400 text-white shadow"
                                    : "bg-gray-100 text-gray-600 dark:bg-dark-700 dark:text-gray-300 hover:bg-emerald-900/30 hover:text-emerald-300"}
                            `}
                            onClick={() => setAñoActivo(idx)}
                        >
                            {año}
                        </button>
                    ))}
                </div>
                <select
                    className="ml-4 px-3 py-2 rounded bg-white text-gray-800 border border-cyan-300 dark:bg-gray-700 dark:text-white dark:border-cyan-800 shadow"
                    value={modalidad}
                    onChange={e => setModalidad(Number(e.target.value))}
                >
                    {MODALIDADES.map(m => (
                        <option key={m.value} value={m.value}>{m.label}</option>
                    ))}
                </select>
            </div>
            {/* Tarjetas filtradas con animación */}
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6 transition-all duration-300">
                {gradosFiltrados.map((grado) => (
                    <TarjetaCurso key={grado.id_grado_secciones} grado={grado} />
                ))}
                {gradosFiltrados.length === 0 && (
                    <div className="col-span-full text-center text-gray-400 py-12">No hay cursos para este año y modalidad.</div>
                )}
            </div>
        </div>
    );
}; 