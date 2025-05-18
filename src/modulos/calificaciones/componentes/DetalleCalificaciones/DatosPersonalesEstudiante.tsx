import React from 'react';

interface DatosPersonalesEstudianteProps {
  estudiante: {
    cedula?: string;
    apellidos?: string;
    nombres?: string;
    nombre_grado?: string;
    nombre_seccion?: string;
    nombre_modalidad?: string;
  };
}

export const DatosPersonalesEstudiante: React.FC<DatosPersonalesEstudianteProps> = ({ estudiante }) => {
  return (
    <section className="space-y-4">
      <h3 className="text-xl font-semibold mb-4">Datos Personales</h3>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div><span className="font-bold">Cédula:</span> {estudiante?.cedula}</div>
        <div><span className="font-bold">Apellidos:</span> {estudiante?.apellidos}</div>
        <div><span className="font-bold">Nombres:</span> {estudiante?.nombres}</div>
        <div><span className="font-bold">Grado:</span> {estudiante?.nombre_grado}</div>
        <div><span className="font-bold">Sección:</span> {estudiante?.nombre_seccion}</div>
        <div><span className="font-bold">Modalidad:</span> {estudiante?.nombre_modalidad}</div>
      </div>
    </section>
  );
}; 