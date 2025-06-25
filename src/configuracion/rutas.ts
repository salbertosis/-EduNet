export const RUTAS = {
  INICIO: '/',
  AUTENTICACION: {
    INICIO_SESION: '/iniciar-sesion',
    REGISTRO: '/registro',
    RECUPERAR_CONTRASENA: '/recuperar-contrasena'
  },
  DASHBOARD: {
    PRINCIPAL: '/dashboard',
    PERFIL: '/dashboard/perfil',
    CONFIGURACION: '/dashboard/configuracion'
  },
  ESTUDIANTES: {
    LISTA: '/estudiantes',
    NUEVO: '/estudiantes/nuevo',
    DETALLE: (id: string) => `/estudiantes/${id}`,
    EDITAR: (id: string) => `/estudiantes/${id}/editar`
  },
  DOCENTES: {
    LISTA: '/docentes',
    NUEVO: '/docentes/nuevo',
    DETALLE: (id: string) => `/docentes/${id}`,
    EDITAR: (id: string) => `/docentes/${id}/editar`
  },
  CURSOS: {
    LISTA: '/cursos',
    NUEVO: '/cursos/nuevo',
    DETALLE: (id: string) => `/cursos/${id}`,
    EDITAR: (id: string) => `/cursos/${id}/editar`
  },
  CALIFICACIONES: {
    LISTA: '/calificaciones',
    NUEVO: '/calificaciones/nuevo',
    DETALLE: (id: string) => `/calificaciones/${id}`,
    EDITAR: (id: string) => `/calificaciones/${id}/editar`
  },
  REPORTES: {
    ANALIZADOR_EXCEL: '/reportes/analizador-excel',
    RESUMEN_FINAL: '/reportes/resumen-final',
    ACTAS: '/reportes/actas'
  }
} as const; 