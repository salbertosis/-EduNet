import React from 'react';

interface CalificacionesActualesProps {
  // Aquí irán las props necesarias, como asignaturas, calificaciones, handlers, etc.
}

export const CalificacionesActuales: React.FC<CalificacionesActualesProps> = (props) => {
  return (
    <section>
      <h3 className="text-xl font-semibold mb-4 text-emerald-400">Calificaciones del Año Actual</h3>
      {/* Aquí irá la tabla de calificaciones y los controles de guardado */}
    </section>
  );
}; 