import React, { useState, useEffect } from 'react';
import { Paginacion } from '../../../componentes/Paginacion';
import { useGrados, Grado } from '../hooks/useGrados';
import { GraduationCap, Users, User, Search } from 'lucide-react';

interface FiltrosGrados {
    busqueda: string;
    grado: string;
    seccion: string;
    modalidad: string;
}

interface TarjetaCursoProps {
    grado: Grado;
}

const TarjetaCurso: React.FC<TarjetaCursoProps> = ({ grado }) => (
    <div className="relative rounded-lg shadow p-4 bg-white dark:bg-[#2a2a40]/90 transition-all duration-200 flex flex-col hover:shadow-md hover:scale-105">
        {/* Encabezado */}
        <div className="flex items-center mb-2">
            <GraduationCap className="w-5 h-5 mr-2 text-blue-400" strokeWidth={1.5} />
            <span className="card-title font-extrabold text-xl text-[#232b36] dark:text-white">
                {grado.nombre_grado} Año {grado.nombre_seccion}
            </span>
        </div>
        {/* Modalidad */}
        <div className="flex items-center mb-2">
            <span className="text-xs text-gray-500 dark:text-gray-400">{grado.nombre_modalidad}</span>
        </div>
        {/* Docente guía */}
        <div className="flex items-center mb-2">
            <User className="w-4 h-4 mr-1 text-cyan-400" strokeWidth={1.5} />
            <span className="subtitle text-xs text-gray-700 dark:text-[#a0aec0]">
                Docente Guía: <b className="text-cyan-600 dark:text-cyan-300">{grado.docente_guia}</b>
            </span>
        </div>
        {/* Estadísticas */}
        <div className="flex items-center gap-3 mb-2">
            <Users className="w-4 h-4 text-blue-400" strokeWidth={1.5} />
            <span className="text-xs text-blue-600 dark:text-blue-400 font-medium">{grado.total_estudiantes} estudiantes</span>
            <User className="w-4 h-4 text-pink-400 ml-2" strokeWidth={1.5} />
            <span className="text-xs text-pink-500 dark:text-pink-400 font-medium">{grado.estudiantes_femeninos}</span>
            <User className="w-4 h-4 text-blue-500 ml-2" strokeWidth={1.5} />
            <span className="text-xs text-blue-700 dark:text-blue-500 font-medium">{grado.estudiantes_masculinos}</span>
        </div>
        {/* Botón de detalles */}
        <div className="flex justify-end mt-3">
            <button className="flex items-center gap-2 px-3 py-1.5 rounded bg-blue-600 text-white text-sm shadow hover:bg-blue-700 transition">
                <Search className="w-4 h-4" strokeWidth={1.5} />
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
                                ${añoActivo === idx ? "bg-gradient-to-r from-teal-400 to-blue-400 text-white shadow" : "bg-gray-700 text-gray-200"}
                            `}
                            onClick={() => setAñoActivo(idx)}
                        >
                            {año}
                        </button>
                    ))}
                </div>
                <select
                    className="ml-4 px-3 py-2 rounded bg-gray-700 text-white"
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