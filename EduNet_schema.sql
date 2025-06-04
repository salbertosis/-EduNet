--
-- PostgreSQL database dump
--

-- Dumped from database version 17.4
-- Dumped by pg_dump version 17.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: estado_estudiante; Type: TYPE; Schema: public; Owner: Salbertosis
--

CREATE TYPE public.estado_estudiante AS ENUM (
    'activo',
    'retirado'
);


ALTER TYPE public.estado_estudiante OWNER TO "Salbertosis";

--
-- Name: asignar_id_grado_secciones(); Type: FUNCTION; Schema: public; Owner: Salbertosis
--

CREATE FUNCTION public.asignar_id_grado_secciones() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    -- Asignar el id_grado_secciones automáticamente sin validación
    NEW.id_grado_secciones = (
        SELECT id_grado_secciones
        FROM grado_secciones
        WHERE id_grado = NEW.id_grado
          AND id_seccion = NEW.id_seccion
          AND id_modalidad = NEW.id_modalidad
    );

    RETURN NEW;
END;
$$;


ALTER FUNCTION public.asignar_id_grado_secciones() OWNER TO "Salbertosis";

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: actas; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.actas (
    id integer NOT NULL,
    nombre_acta character varying(100) NOT NULL,
    id_asignatura integer NOT NULL,
    id_grado integer NOT NULL,
    id_seccion integer NOT NULL,
    id_modalidad integer NOT NULL,
    id_lapso integer NOT NULL,
    fecha_creacion timestamp without time zone DEFAULT now()
);


ALTER TABLE public.actas OWNER TO "Salbertosis";

--
-- Name: actas_id_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.actas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.actas_id_seq OWNER TO "Salbertosis";

--
-- Name: actas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.actas_id_seq OWNED BY public.actas.id;


--
-- Name: asignaturas; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.asignaturas (
    id_asignatura integer NOT NULL,
    nombre character varying(100)
);


ALTER TABLE public.asignaturas OWNER TO "Salbertosis";

--
-- Name: asignaturas_id_asignatura_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.asignaturas_id_asignatura_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.asignaturas_id_asignatura_seq OWNER TO "Salbertosis";

--
-- Name: asignaturas_id_asignatura_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.asignaturas_id_asignatura_seq OWNED BY public.asignaturas.id_asignatura;


--
-- Name: asignaturas_pendientes; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.asignaturas_pendientes (
    id_pendiente integer NOT NULL,
    id_estudiante integer NOT NULL,
    id_asignatura integer NOT NULL,
    id_periodo integer NOT NULL,
    grado character varying(20) NOT NULL,
    cal_momento1 integer,
    cal_momento2 integer,
    cal_momento3 integer,
    cal_momento4 integer,
    estado character varying(15) NOT NULL,
    fecha_registro timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    id_grado_secciones integer
);


ALTER TABLE public.asignaturas_pendientes OWNER TO "Salbertosis";

--
-- Name: asignaturas_pendientes_id_pendiente_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.asignaturas_pendientes_id_pendiente_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.asignaturas_pendientes_id_pendiente_seq OWNER TO "Salbertosis";

--
-- Name: asignaturas_pendientes_id_pendiente_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.asignaturas_pendientes_id_pendiente_seq OWNED BY public.asignaturas_pendientes.id_pendiente;


--
-- Name: calificaciones; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.calificaciones (
    id_calificacion integer NOT NULL,
    id_estudiante integer NOT NULL,
    id_asignatura integer NOT NULL,
    id_periodo integer NOT NULL,
    lapso_1 integer,
    lapso_2 integer,
    lapso_3 integer,
    revision integer,
    lapso_1_ajustado integer,
    lapso_2_ajustado integer,
    lapso_3_ajustado integer,
    nota_final integer,
    CONSTRAINT calificaciones_lapso_1_ajustado_check CHECK ((((lapso_1_ajustado >= 0) AND (lapso_1_ajustado <= 20)) OR (lapso_1_ajustado IS NULL))),
    CONSTRAINT calificaciones_lapso_2_ajustado_check CHECK ((((lapso_2_ajustado >= 0) AND (lapso_2_ajustado <= 20)) OR (lapso_2_ajustado IS NULL))),
    CONSTRAINT calificaciones_lapso_3_ajustado_check CHECK ((((lapso_3_ajustado >= 0) AND (lapso_3_ajustado <= 20)) OR (lapso_3_ajustado IS NULL)))
);


ALTER TABLE public.calificaciones OWNER TO "Salbertosis";

--
-- Name: calificaciones_extra_catedra; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.calificaciones_extra_catedra (
    cedula_estudiante bigint NOT NULL,
    id_extra_catedra integer NOT NULL,
    lapso character varying(50) NOT NULL,
    calificacion numeric(5,2),
    id bigint
);


ALTER TABLE public.calificaciones_extra_catedra OWNER TO "Salbertosis";

--
-- Name: calificaciones_nueva_id_calificacion_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.calificaciones_nueva_id_calificacion_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.calificaciones_nueva_id_calificacion_seq OWNER TO "Salbertosis";

--
-- Name: calificaciones_nueva_id_calificacion_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.calificaciones_nueva_id_calificacion_seq OWNED BY public.calificaciones.id_calificacion;


--
-- Name: docentes; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.docentes (
    id_docente integer NOT NULL,
    cedula bigint NOT NULL,
    nombres character varying(50) NOT NULL,
    apellidos character varying(50) NOT NULL,
    especialidad character varying(100),
    telefono character varying(20),
    correoelectronico character varying(250)
);


ALTER TABLE public.docentes OWNER TO "Salbertosis";

--
-- Name: docentes_id_docente_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.docentes_id_docente_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.docentes_id_docente_seq OWNER TO "Salbertosis";

--
-- Name: docentes_id_docente_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.docentes_id_docente_seq OWNED BY public.docentes.id_docente;


--
-- Name: est_consolidado; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.est_consolidado (
    id_consolidado integer NOT NULL,
    cedula integer NOT NULL,
    id_grado_asignatura integer NOT NULL,
    id bigint
);


ALTER TABLE public.est_consolidado OWNER TO "Salbertosis";

--
-- Name: est_consolidado_id_consolidado_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.est_consolidado_id_consolidado_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.est_consolidado_id_consolidado_seq OWNER TO "Salbertosis";

--
-- Name: est_consolidado_id_consolidado_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.est_consolidado_id_consolidado_seq OWNED BY public.est_consolidado.id_consolidado;


--
-- Name: estudiantes; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.estudiantes (
    cedula bigint NOT NULL,
    nombres character varying(50) NOT NULL,
    apellidos character varying(50) NOT NULL,
    genero character(1),
    fecha_nacimiento date NOT NULL,
    id_grado_secciones integer,
    fecha_ingreso date,
    municipionac character varying(50),
    paisnac character varying(50),
    entidadfed character varying(10),
    ciudadnac character varying(50),
    estadonac character varying(50),
    id_grado integer,
    id_seccion integer,
    id_modalidad integer,
    id_periodoactual integer,
    id integer NOT NULL,
    estado public.estado_estudiante DEFAULT 'activo'::public.estado_estudiante NOT NULL,
    fecha_retiro date,
    CONSTRAINT estudiantes_genero_check CHECK ((genero = ANY (ARRAY['M'::bpchar, 'F'::bpchar])))
);


ALTER TABLE public.estudiantes OWNER TO "Salbertosis";

--
-- Name: estudiantes_extra_catedra; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.estudiantes_extra_catedra (
    cedula_estudiante bigint NOT NULL,
    id_extra_catedra integer NOT NULL,
    id bigint,
    cedula bigint
);


ALTER TABLE public.estudiantes_extra_catedra OWNER TO "Salbertosis";

--
-- Name: estudiantes_id_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.estudiantes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.estudiantes_id_seq OWNER TO "Salbertosis";

--
-- Name: estudiantes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.estudiantes_id_seq OWNED BY public.estudiantes.id;


--
-- Name: extra_catedra; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.extra_catedra (
    id_extra_catedra integer NOT NULL,
    nombre character varying(255) NOT NULL
);


ALTER TABLE public.extra_catedra OWNER TO "Salbertosis";

--
-- Name: extra_catedra_id_extra_catedra_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.extra_catedra_id_extra_catedra_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.extra_catedra_id_extra_catedra_seq OWNER TO "Salbertosis";

--
-- Name: extra_catedra_id_extra_catedra_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.extra_catedra_id_extra_catedra_seq OWNED BY public.extra_catedra.id_extra_catedra;


--
-- Name: grado_modalidad_asignaturas; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.grado_modalidad_asignaturas (
    id integer NOT NULL,
    id_modalidad integer,
    id_grado integer,
    id_asignatura integer,
    orden integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.grado_modalidad_asignaturas OWNER TO "Salbertosis";

--
-- Name: grado_modalidad_asignaturas_id_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.grado_modalidad_asignaturas_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.grado_modalidad_asignaturas_id_seq OWNER TO "Salbertosis";

--
-- Name: grado_modalidad_asignaturas_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.grado_modalidad_asignaturas_id_seq OWNED BY public.grado_modalidad_asignaturas.id;


--
-- Name: grado_secciones; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.grado_secciones (
    id_grado_secciones integer NOT NULL,
    id_grado integer,
    id_seccion integer,
    id_modalidad integer,
    id_docente_guia integer
);


ALTER TABLE public.grado_secciones OWNER TO "Salbertosis";

--
-- Name: grado_secciones_id_grado_seccion_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.grado_secciones_id_grado_seccion_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.grado_secciones_id_grado_seccion_seq OWNER TO "Salbertosis";

--
-- Name: grado_secciones_id_grado_seccion_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.grado_secciones_id_grado_seccion_seq OWNED BY public.grado_secciones.id_grado_secciones;


--
-- Name: grados; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.grados (
    id_grado integer NOT NULL,
    nombre_grado character varying(10) NOT NULL
);


ALTER TABLE public.grados OWNER TO "Salbertosis";

--
-- Name: grados_asignaturas; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.grados_asignaturas (
    id_grado_asignatura integer NOT NULL,
    id_grado integer NOT NULL,
    id_asignatura integer NOT NULL
);


ALTER TABLE public.grados_asignaturas OWNER TO "Salbertosis";

--
-- Name: grados_asignaturas_id_grado_asignatura_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.grados_asignaturas_id_grado_asignatura_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.grados_asignaturas_id_grado_asignatura_seq OWNER TO "Salbertosis";

--
-- Name: grados_asignaturas_id_grado_asignatura_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.grados_asignaturas_id_grado_asignatura_seq OWNED BY public.grados_asignaturas.id_grado_asignatura;


--
-- Name: grados_id_grado_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.grados_id_grado_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.grados_id_grado_seq OWNER TO "Salbertosis";

--
-- Name: grados_id_grado_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.grados_id_grado_seq OWNED BY public.grados.id_grado;


--
-- Name: historial_academico; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.historial_academico (
    id_historial integer NOT NULL,
    id_estudiante integer NOT NULL,
    id_periodo integer NOT NULL,
    id_grado_secciones integer NOT NULL,
    promedio_anual numeric(5,2) NOT NULL,
    estatus character varying(20) NOT NULL,
    fecha_registro timestamp without time zone DEFAULT CURRENT_TIMESTAMP
);


ALTER TABLE public.historial_academico OWNER TO "Salbertosis";

--
-- Name: historial_academico_id_historial_seq1; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.historial_academico_id_historial_seq1
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.historial_academico_id_historial_seq1 OWNER TO "Salbertosis";

--
-- Name: historial_academico_id_historial_seq1; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.historial_academico_id_historial_seq1 OWNED BY public.historial_academico.id_historial;


--
-- Name: historial_grado_estudiantes; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.historial_grado_estudiantes (
    id_historial_grado integer NOT NULL,
    id_estudiante integer NOT NULL,
    id_grado_secciones integer NOT NULL,
    id_periodo integer NOT NULL,
    fecha_inicio date NOT NULL,
    fecha_fin date,
    es_actual boolean DEFAULT true
);


ALTER TABLE public.historial_grado_estudiantes OWNER TO "Salbertosis";

--
-- Name: historial_grado_estudiantes_id_historial_grado_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.historial_grado_estudiantes_id_historial_grado_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.historial_grado_estudiantes_id_historial_grado_seq OWNER TO "Salbertosis";

--
-- Name: historial_grado_estudiantes_id_historial_grado_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.historial_grado_estudiantes_id_historial_grado_seq OWNED BY public.historial_grado_estudiantes.id_historial_grado;


--
-- Name: lapsos; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.lapsos (
    id_lapso integer NOT NULL,
    id_periodo_escolar integer NOT NULL,
    nombre_lapso character varying(50) NOT NULL
);


ALTER TABLE public.lapsos OWNER TO "Salbertosis";

--
-- Name: lapsos_id_lapso_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.lapsos_id_lapso_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.lapsos_id_lapso_seq OWNER TO "Salbertosis";

--
-- Name: lapsos_id_lapso_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.lapsos_id_lapso_seq OWNED BY public.lapsos.id_lapso;


--
-- Name: modalidades; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.modalidades (
    id_modalidad integer NOT NULL,
    nombre_modalidad character varying(50) NOT NULL
);


ALTER TABLE public.modalidades OWNER TO "Salbertosis";

--
-- Name: modalidades_id_modalidad_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.modalidades_id_modalidad_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.modalidades_id_modalidad_seq OWNER TO "Salbertosis";

--
-- Name: modalidades_id_modalidad_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.modalidades_id_modalidad_seq OWNED BY public.modalidades.id_modalidad;


--
-- Name: periodos_escolares; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.periodos_escolares (
    id_periodo integer NOT NULL,
    periodo_escolar character varying(9) NOT NULL,
    fecha_inicio date,
    fecha_final date,
    activo boolean DEFAULT false
);


ALTER TABLE public.periodos_escolares OWNER TO "Salbertosis";

--
-- Name: periodos_escolares_id_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.periodos_escolares_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.periodos_escolares_id_seq OWNER TO "Salbertosis";

--
-- Name: periodos_escolares_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.periodos_escolares_id_seq OWNED BY public.periodos_escolares.id_periodo;


--
-- Name: secciones; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.secciones (
    id_seccion integer NOT NULL,
    nombre_seccion character varying(10) NOT NULL
);


ALTER TABLE public.secciones OWNER TO "Salbertosis";

--
-- Name: secciones_id_seccion_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.secciones_id_seccion_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.secciones_id_seccion_seq OWNER TO "Salbertosis";

--
-- Name: secciones_id_seccion_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.secciones_id_seccion_seq OWNED BY public.secciones.id_seccion;


--
-- Name: usuarios; Type: TABLE; Schema: public; Owner: Salbertosis
--

CREATE TABLE public.usuarios (
    id integer NOT NULL,
    username character varying(50) NOT NULL,
    password character varying(255) NOT NULL,
    role character varying(20) NOT NULL
);


ALTER TABLE public.usuarios OWNER TO "Salbertosis";

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: Salbertosis
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO "Salbertosis";

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: Salbertosis
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.usuarios.id;


--
-- Name: actas id; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas ALTER COLUMN id SET DEFAULT nextval('public.actas_id_seq'::regclass);


--
-- Name: asignaturas id_asignatura; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas ALTER COLUMN id_asignatura SET DEFAULT nextval('public.asignaturas_id_asignatura_seq'::regclass);


--
-- Name: asignaturas_pendientes id_pendiente; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes ALTER COLUMN id_pendiente SET DEFAULT nextval('public.asignaturas_pendientes_id_pendiente_seq'::regclass);


--
-- Name: calificaciones id_calificacion; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones ALTER COLUMN id_calificacion SET DEFAULT nextval('public.calificaciones_nueva_id_calificacion_seq'::regclass);


--
-- Name: docentes id_docente; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.docentes ALTER COLUMN id_docente SET DEFAULT nextval('public.docentes_id_docente_seq'::regclass);


--
-- Name: est_consolidado id_consolidado; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.est_consolidado ALTER COLUMN id_consolidado SET DEFAULT nextval('public.est_consolidado_id_consolidado_seq'::regclass);


--
-- Name: estudiantes id; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes ALTER COLUMN id SET DEFAULT nextval('public.estudiantes_id_seq'::regclass);


--
-- Name: extra_catedra id_extra_catedra; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.extra_catedra ALTER COLUMN id_extra_catedra SET DEFAULT nextval('public.extra_catedra_id_extra_catedra_seq'::regclass);


--
-- Name: grado_modalidad_asignaturas id; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas ALTER COLUMN id SET DEFAULT nextval('public.grado_modalidad_asignaturas_id_seq'::regclass);


--
-- Name: grado_secciones id_grado_secciones; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones ALTER COLUMN id_grado_secciones SET DEFAULT nextval('public.grado_secciones_id_grado_seccion_seq'::regclass);


--
-- Name: grados id_grado; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados ALTER COLUMN id_grado SET DEFAULT nextval('public.grados_id_grado_seq'::regclass);


--
-- Name: grados_asignaturas id_grado_asignatura; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados_asignaturas ALTER COLUMN id_grado_asignatura SET DEFAULT nextval('public.grados_asignaturas_id_grado_asignatura_seq'::regclass);


--
-- Name: historial_academico id_historial; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico ALTER COLUMN id_historial SET DEFAULT nextval('public.historial_academico_id_historial_seq1'::regclass);


--
-- Name: historial_grado_estudiantes id_historial_grado; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes ALTER COLUMN id_historial_grado SET DEFAULT nextval('public.historial_grado_estudiantes_id_historial_grado_seq'::regclass);


--
-- Name: lapsos id_lapso; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.lapsos ALTER COLUMN id_lapso SET DEFAULT nextval('public.lapsos_id_lapso_seq'::regclass);


--
-- Name: modalidades id_modalidad; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.modalidades ALTER COLUMN id_modalidad SET DEFAULT nextval('public.modalidades_id_modalidad_seq'::regclass);


--
-- Name: periodos_escolares id_periodo; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.periodos_escolares ALTER COLUMN id_periodo SET DEFAULT nextval('public.periodos_escolares_id_seq'::regclass);


--
-- Name: secciones id_seccion; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.secciones ALTER COLUMN id_seccion SET DEFAULT nextval('public.secciones_id_seccion_seq'::regclass);


--
-- Name: usuarios id; Type: DEFAULT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.usuarios ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: actas actas_nombre_acta_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_nombre_acta_key UNIQUE (nombre_acta);


--
-- Name: actas actas_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_pkey PRIMARY KEY (id);


--
-- Name: asignaturas_pendientes asignaturas_pendientes_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT asignaturas_pendientes_pkey PRIMARY KEY (id_pendiente);


--
-- Name: asignaturas_pendientes asignaturas_pendientes_unica; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT asignaturas_pendientes_unica UNIQUE (id_estudiante, id_asignatura, id_periodo);


--
-- Name: asignaturas asignaturas_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas
    ADD CONSTRAINT asignaturas_pkey PRIMARY KEY (id_asignatura);


--
-- Name: calificaciones_extra_catedra calificaciones_extra_catedra_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones_extra_catedra
    ADD CONSTRAINT calificaciones_extra_catedra_pkey PRIMARY KEY (cedula_estudiante, id_extra_catedra, lapso);


--
-- Name: calificaciones calificaciones_nueva_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones
    ADD CONSTRAINT calificaciones_nueva_pkey PRIMARY KEY (id_calificacion);


--
-- Name: docentes docentes_cedula_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.docentes
    ADD CONSTRAINT docentes_cedula_key UNIQUE (cedula);


--
-- Name: docentes docentes_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.docentes
    ADD CONSTRAINT docentes_pkey PRIMARY KEY (id_docente);


--
-- Name: est_consolidado est_consolidado_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.est_consolidado
    ADD CONSTRAINT est_consolidado_pkey PRIMARY KEY (id_consolidado);


--
-- Name: estudiantes_extra_catedra estudiantes_extra_catedra_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes_extra_catedra
    ADD CONSTRAINT estudiantes_extra_catedra_pkey PRIMARY KEY (cedula_estudiante, id_extra_catedra);


--
-- Name: estudiantes estudiantes_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT estudiantes_pkey PRIMARY KEY (id);


--
-- Name: extra_catedra extra_catedra_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.extra_catedra
    ADD CONSTRAINT extra_catedra_pkey PRIMARY KEY (id_extra_catedra);


--
-- Name: grado_modalidad_asignaturas grado_modalidad_asignaturas_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas
    ADD CONSTRAINT grado_modalidad_asignaturas_pkey PRIMARY KEY (id);


--
-- Name: grado_secciones grado_secciones_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT grado_secciones_pkey PRIMARY KEY (id_grado_secciones);


--
-- Name: grados_asignaturas grados_asignaturas_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados_asignaturas
    ADD CONSTRAINT grados_asignaturas_pkey PRIMARY KEY (id_grado_asignatura);


--
-- Name: grados grados_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados
    ADD CONSTRAINT grados_pkey PRIMARY KEY (id_grado);


--
-- Name: historial_academico historial_academico_estudiante_periodo_unique; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT historial_academico_estudiante_periodo_unique UNIQUE (id_estudiante, id_periodo);


--
-- Name: historial_academico historial_academico_pkey1; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT historial_academico_pkey1 PRIMARY KEY (id_historial);


--
-- Name: historial_grado_estudiantes historial_grado_estudiantes_id_estudiante_id_periodo_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes
    ADD CONSTRAINT historial_grado_estudiantes_id_estudiante_id_periodo_key UNIQUE (id_estudiante, id_periodo);


--
-- Name: historial_grado_estudiantes historial_grado_estudiantes_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes
    ADD CONSTRAINT historial_grado_estudiantes_pkey PRIMARY KEY (id_historial_grado);


--
-- Name: lapsos lapsos_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.lapsos
    ADD CONSTRAINT lapsos_pkey PRIMARY KEY (id_lapso);


--
-- Name: modalidades modalidades_nombre_modalidad_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.modalidades
    ADD CONSTRAINT modalidades_nombre_modalidad_key UNIQUE (nombre_modalidad);


--
-- Name: modalidades modalidades_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.modalidades
    ADD CONSTRAINT modalidades_pkey PRIMARY KEY (id_modalidad);


--
-- Name: periodos_escolares periodos_escolares_periodo_escolar_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.periodos_escolares
    ADD CONSTRAINT periodos_escolares_periodo_escolar_key UNIQUE (periodo_escolar);


--
-- Name: periodos_escolares periodos_escolares_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.periodos_escolares
    ADD CONSTRAINT periodos_escolares_pkey PRIMARY KEY (id_periodo);


--
-- Name: secciones secciones_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.secciones
    ADD CONSTRAINT secciones_pkey PRIMARY KEY (id_seccion);


--
-- Name: calificaciones unique_calificacion; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones
    ADD CONSTRAINT unique_calificacion UNIQUE (id_estudiante, id_asignatura, id_periodo);


--
-- Name: historial_academico unique_estudiante_periodo_seccion; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT unique_estudiante_periodo_seccion UNIQUE (id_estudiante, id_periodo, id_grado_secciones);


--
-- Name: grado_secciones unique_grado_seccion_modalidad; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT unique_grado_seccion_modalidad UNIQUE (id_grado, id_seccion, id_modalidad);


--
-- Name: grado_modalidad_asignaturas unique_modalidad_grado_asignatura; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas
    ADD CONSTRAINT unique_modalidad_grado_asignatura UNIQUE (id_modalidad, id_grado, id_asignatura);


--
-- Name: usuarios users_pkey; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: usuarios users_username_key; Type: CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.usuarios
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: idx_asignaturas_pendientes_est_periodo; Type: INDEX; Schema: public; Owner: Salbertosis
--

CREATE INDEX idx_asignaturas_pendientes_est_periodo ON public.asignaturas_pendientes USING btree (id_estudiante, id_periodo);


--
-- Name: idx_calificaciones_est_periodo; Type: INDEX; Schema: public; Owner: Salbertosis
--

CREATE INDEX idx_calificaciones_est_periodo ON public.calificaciones USING btree (id_estudiante, id_periodo);


--
-- Name: idx_cedula; Type: INDEX; Schema: public; Owner: Salbertosis
--

CREATE UNIQUE INDEX idx_cedula ON public.estudiantes USING btree (cedula);


--
-- Name: idx_estudiantes_grado_secciones; Type: INDEX; Schema: public; Owner: Salbertosis
--

CREATE INDEX idx_estudiantes_grado_secciones ON public.estudiantes USING btree (id_grado_secciones);


--
-- Name: idx_historial_academico_est_periodo; Type: INDEX; Schema: public; Owner: Salbertosis
--

CREATE INDEX idx_historial_academico_est_periodo ON public.historial_academico USING btree (id_estudiante, id_periodo);


--
-- Name: estudiantes trigger_asignar_grado_seccion; Type: TRIGGER; Schema: public; Owner: Salbertosis
--

CREATE TRIGGER trigger_asignar_grado_seccion BEFORE INSERT ON public.estudiantes FOR EACH ROW EXECUTE FUNCTION public.asignar_id_grado_secciones();


--
-- Name: actas actas_id_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_id_asignatura_fkey FOREIGN KEY (id_asignatura) REFERENCES public.asignaturas(id_asignatura);


--
-- Name: actas actas_id_grado_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_id_grado_fkey FOREIGN KEY (id_grado) REFERENCES public.grados(id_grado);


--
-- Name: actas actas_id_lapso_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_id_lapso_fkey FOREIGN KEY (id_lapso) REFERENCES public.lapsos(id_lapso);


--
-- Name: actas actas_id_modalidad_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_id_modalidad_fkey FOREIGN KEY (id_modalidad) REFERENCES public.modalidades(id_modalidad);


--
-- Name: actas actas_id_seccion_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.actas
    ADD CONSTRAINT actas_id_seccion_fkey FOREIGN KEY (id_seccion) REFERENCES public.secciones(id_seccion);


--
-- Name: asignaturas_pendientes asignaturas_pendientes_id_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT asignaturas_pendientes_id_asignatura_fkey FOREIGN KEY (id_asignatura) REFERENCES public.asignaturas(id_asignatura);


--
-- Name: asignaturas_pendientes asignaturas_pendientes_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT asignaturas_pendientes_id_fkey FOREIGN KEY (id_estudiante) REFERENCES public.estudiantes(id);


--
-- Name: asignaturas_pendientes asignaturas_pendientes_id_periodo_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT asignaturas_pendientes_id_periodo_fkey FOREIGN KEY (id_periodo) REFERENCES public.periodos_escolares(id_periodo);


--
-- Name: calificaciones_extra_catedra calificaciones_extra_catedra_id_extra_catedra_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones_extra_catedra
    ADD CONSTRAINT calificaciones_extra_catedra_id_extra_catedra_fkey FOREIGN KEY (id_extra_catedra) REFERENCES public.extra_catedra(id_extra_catedra);


--
-- Name: calificaciones calificaciones_nueva_id_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones
    ADD CONSTRAINT calificaciones_nueva_id_asignatura_fkey FOREIGN KEY (id_asignatura) REFERENCES public.asignaturas(id_asignatura);


--
-- Name: calificaciones calificaciones_nueva_id_estudiante_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones
    ADD CONSTRAINT calificaciones_nueva_id_estudiante_fkey FOREIGN KEY (id_estudiante) REFERENCES public.estudiantes(id);


--
-- Name: calificaciones calificaciones_nueva_id_periodo_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.calificaciones
    ADD CONSTRAINT calificaciones_nueva_id_periodo_fkey FOREIGN KEY (id_periodo) REFERENCES public.periodos_escolares(id_periodo);


--
-- Name: est_consolidado est_consolidado_id_grado_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.est_consolidado
    ADD CONSTRAINT est_consolidado_id_grado_asignatura_fkey FOREIGN KEY (id_grado_asignatura) REFERENCES public.grados_asignaturas(id_grado_asignatura) ON DELETE CASCADE;


--
-- Name: estudiantes_extra_catedra estudiantes_extra_catedra_id_extra_catedra_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes_extra_catedra
    ADD CONSTRAINT estudiantes_extra_catedra_id_extra_catedra_fkey FOREIGN KEY (id_extra_catedra) REFERENCES public.extra_catedra(id_extra_catedra);


--
-- Name: estudiantes estudiantes_id_grado_seccion_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT estudiantes_id_grado_seccion_fkey FOREIGN KEY (id_grado_secciones) REFERENCES public.grado_secciones(id_grado_secciones);


--
-- Name: asignaturas_pendientes fk_asignaturas_pendientes_grado_secciones; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.asignaturas_pendientes
    ADD CONSTRAINT fk_asignaturas_pendientes_grado_secciones FOREIGN KEY (id_grado_secciones) REFERENCES public.grado_secciones(id_grado_secciones) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- Name: grado_secciones fk_docente_guia; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT fk_docente_guia FOREIGN KEY (id_docente_guia) REFERENCES public.docentes(id_docente) ON DELETE SET NULL;


--
-- Name: grado_secciones fk_grado; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT fk_grado FOREIGN KEY (id_grado) REFERENCES public.grados(id_grado);


--
-- Name: estudiantes fk_grado; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT fk_grado FOREIGN KEY (id_grado) REFERENCES public.grados(id_grado);


--
-- Name: estudiantes fk_id_periodoactual; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT fk_id_periodoactual FOREIGN KEY (id_periodoactual) REFERENCES public.periodos_escolares(id_periodo) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: grado_secciones fk_modalidad; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT fk_modalidad FOREIGN KEY (id_modalidad) REFERENCES public.modalidades(id_modalidad);


--
-- Name: estudiantes fk_modalidad; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT fk_modalidad FOREIGN KEY (id_modalidad) REFERENCES public.modalidades(id_modalidad);


--
-- Name: lapsos fk_periodo_escolar; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.lapsos
    ADD CONSTRAINT fk_periodo_escolar FOREIGN KEY (id_periodo_escolar) REFERENCES public.periodos_escolares(id_periodo) ON DELETE CASCADE;


--
-- Name: grado_secciones fk_seccion; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_secciones
    ADD CONSTRAINT fk_seccion FOREIGN KEY (id_seccion) REFERENCES public.secciones(id_seccion);


--
-- Name: estudiantes fk_seccion; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.estudiantes
    ADD CONSTRAINT fk_seccion FOREIGN KEY (id_seccion) REFERENCES public.secciones(id_seccion);


--
-- Name: grado_modalidad_asignaturas grado_modalidad_asignaturas_id_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas
    ADD CONSTRAINT grado_modalidad_asignaturas_id_asignatura_fkey FOREIGN KEY (id_asignatura) REFERENCES public.asignaturas(id_asignatura);


--
-- Name: grado_modalidad_asignaturas grado_modalidad_asignaturas_id_grado_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas
    ADD CONSTRAINT grado_modalidad_asignaturas_id_grado_fkey FOREIGN KEY (id_grado) REFERENCES public.grados(id_grado);


--
-- Name: grado_modalidad_asignaturas grado_modalidad_asignaturas_id_modalidad_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grado_modalidad_asignaturas
    ADD CONSTRAINT grado_modalidad_asignaturas_id_modalidad_fkey FOREIGN KEY (id_modalidad) REFERENCES public.modalidades(id_modalidad);


--
-- Name: grados_asignaturas grados_asignaturas_id_asignatura_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados_asignaturas
    ADD CONSTRAINT grados_asignaturas_id_asignatura_fkey FOREIGN KEY (id_asignatura) REFERENCES public.asignaturas(id_asignatura) ON DELETE CASCADE;


--
-- Name: grados_asignaturas grados_asignaturas_id_grado_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.grados_asignaturas
    ADD CONSTRAINT grados_asignaturas_id_grado_fkey FOREIGN KEY (id_grado) REFERENCES public.grados(id_grado) ON DELETE CASCADE;


--
-- Name: historial_academico historial_academico_id_estudiante_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT historial_academico_id_estudiante_fkey FOREIGN KEY (id_estudiante) REFERENCES public.estudiantes(id);


--
-- Name: historial_academico historial_academico_id_grado_secciones_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT historial_academico_id_grado_secciones_fkey FOREIGN KEY (id_grado_secciones) REFERENCES public.grado_secciones(id_grado_secciones);


--
-- Name: historial_academico historial_academico_id_periodo_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_academico
    ADD CONSTRAINT historial_academico_id_periodo_fkey FOREIGN KEY (id_periodo) REFERENCES public.periodos_escolares(id_periodo);


--
-- Name: historial_grado_estudiantes historial_grado_estudiantes_id_estudiante_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes
    ADD CONSTRAINT historial_grado_estudiantes_id_estudiante_fkey FOREIGN KEY (id_estudiante) REFERENCES public.estudiantes(id) ON DELETE CASCADE;


--
-- Name: historial_grado_estudiantes historial_grado_estudiantes_id_grado_secciones_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes
    ADD CONSTRAINT historial_grado_estudiantes_id_grado_secciones_fkey FOREIGN KEY (id_grado_secciones) REFERENCES public.grado_secciones(id_grado_secciones);


--
-- Name: historial_grado_estudiantes historial_grado_estudiantes_id_periodo_fkey; Type: FK CONSTRAINT; Schema: public; Owner: Salbertosis
--

ALTER TABLE ONLY public.historial_grado_estudiantes
    ADD CONSTRAINT historial_grado_estudiantes_id_periodo_fkey FOREIGN KEY (id_periodo) REFERENCES public.periodos_escolares(id_periodo);


--
-- PostgreSQL database dump complete
--

