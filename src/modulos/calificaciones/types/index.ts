export interface CalificacionEstudiante {
    id_calificacion?: number;
    id_asignatura: number;
    nombre_asignatura: string;
    lapso_1?: number;
    lapso_1_ajustado?: number;
    lapso_2?: number;
    lapso_2_ajustado?: number;
    lapso_3?: number;
    lapso_3_ajustado?: number;
    nota_final?: number;
    revision?: string;
}

export interface Asignatura {
    id_asignatura: number;
    nombre_asignatura: string;
    id_grado?: number;
    id_modalidad?: number;
} 