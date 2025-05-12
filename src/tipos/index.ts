export interface Usuario {
  id: number;
  username: string;
  role: string;
}

export interface Estudiante extends Usuario {
  matricula: string;
  fechaNacimiento: Date;
  grado: string;
  seccion: string;
  tutor?: {
    nombre: string;
    telefono: string;
    email: string;
  };
}

export interface Profesor extends Usuario {
  especialidad: string;
  titulo: string;
  cursos: string[]; // IDs de los cursos
}

export interface Curso {
  id: string;
  nombre: string;
  codigo: string;
  descripcion: string;
  profesorId: string;
  grado: string;
  seccion: string;
  activo: boolean;
  fechaInicio: Date;
  fechaFin: Date;
}

export interface Calificacion {
  id: string;
  estudianteId: string;
  cursoId: string;
  periodo: string;
  valor: number;
  tipo: 'EXAMEN' | 'TAREA' | 'PROYECTO' | 'PARTICIPACION';
  fecha: Date;
  observaciones?: string;
}

export interface Sesion {
  usuario: Usuario;
  token: string;
  expiracion: Date;
} 