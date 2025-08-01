# React + TypeScript + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## Expanding the ESLint configuration

If you are developing a production application, we recommend updating the configuration to enable type-aware lint rules:

# EduNet - Sistema de Gestión Educativa

EduNet es una aplicación de escritorio moderna y completa para la gestión académica, construida con Tauri, React, TypeScript y Vite. Proporciona una solución robusta y eficiente para la administración de instituciones educativas, cubriendo desde la gestión de estudiantes y calificaciones hasta la generación de reportes detallados.

## ✨ Características Principales

- **Gestión de Estudiantes:** Registro, actualización y consulta de información detallada de los estudiantes.
- **Control de Calificaciones:** Ingreso y seguimiento de calificaciones por asignatura, lapso y tipo de evaluación.
- **Generación de Reportes:** Creación de resúmenes finales, actas y otros documentos oficiales en formato HTML y PDF.
- **Interfaz de Usuario Moderna:** Interfaz rápida y amigable desarrollada con React y Tailwind CSS.
- **Backend Potente en Rust:** Lógica de negocio segura y de alto rendimiento implementada en Rust.
- **Base de Datos PostgreSQL:** Almacenamiento de datos robusto y escalable.
- **Aplicación de Escritorio Multiplataforma:** Gracias a Tauri, EduNet puede compilarse para Windows, macOS y Linux.

## 🛠️ Tecnologías Utilizadas

- **Frontend:**
  - [React](https://reactjs.org/)
  - [TypeScript](https://www.typescriptlang.org/)
  - [Vite](https://vitejs.dev/)
  - [Tailwind CSS](https://tailwindcss.com/)
- **Backend:**
  - [Rust](https://www.rust-lang.org/)
  - [Tauri](https://tauri.app/)
- **Base de Datos:**
  - [PostgreSQL](https://www.postgresql.org/)

## 🚀 Instalación y Puesta en Marcha

Sigue estos pasos para configurar el entorno de desarrollo y ejecutar la aplicación.

### Prerrequisitos

- [Node.js](https://nodejs.org/) (versión 18 o superior)
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

### Pasos de Instalación

1. **Clonar el repositorio:**
   ```bash
   git clone https://github.com/tu-usuario/edunet-tauri.git
   cd edunet-tauri
   ```

2. **Instalar dependencias del frontend:**
   ```bash
   npm install
   ```

3. **Configurar la base de datos:**
   - Asegúrate de que PostgreSQL esté en ejecución.
   - Crea una base de datos para la aplicación.
   - Importa el esquema inicial desde `EduNet_schema.sql` y `EduNet_schema_actualizado.sql`.

4. **Configurar variables de entorno:**
   - Crea un archivo `.env` en la raíz del proyecto si es necesario para la conexión a la base de datos u otras configuraciones.

### Ejecutar la Aplicación

Para iniciar la aplicación en modo de desarrollo, ejecuta:

```bash
npm run tauri dev
```

Esto iniciará el servidor de desarrollo de Vite y la aplicación de escritorio de Tauri.

## 📂 Estructura del Proyecto

El proyecto está organizado de la siguiente manera:

- `src/`: Contiene el código fuente del frontend en React y TypeScript.
  - `componentes/`: Componentes reutilizables de la interfaz.
  - `modulos/`: Lógica de negocio y componentes específicos de cada módulo (calificaciones, estudiantes, etc.).
  - `servicios/`: Funciones para interactuar con el backend de Tauri.
- `src-tauri/`: Contiene el código fuente del backend en Rust.
  - `src/api/`: Módulos que definen los comandos y la lógica de negocio del backend.
  - `plantillas/`: Plantillas HTML para la generación de reportes.
- `EduNet_schema.sql`: Esquema inicial de la base de datos.

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Si deseas mejorar el proyecto, por favor sigue estos pasos:

1. Haz un fork del repositorio.
2. Crea una nueva rama para tu característica (`git checkout -b feature/nueva-caracteristica`).
3. Realiza tus cambios y haz commit (`git commit -m 'Añadir nueva característica'`).
4. Envía tus cambios a la rama (`git push origin feature/nueva-caracteristica`).
5. Abre un Pull Request.

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.


You can also install [eslint-plugin-react-x](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-x) and [eslint-plugin-react-dom](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-dom) for React-specific lint rules:

```js
// eslint.config.js
import reactX from 'eslint-plugin-react-x'
import reactDom from 'eslint-plugin-react-dom'

export default tseslint.config({
  plugins: {
    // Add the react-x and react-dom plugins
    'react-x': reactX,
    'react-dom': reactDom,
  },
  rules: {
    // other rules...
    // Enable its recommended typescript rules
    ...reactX.configs['recommended-typescript'].rules,
    ...reactDom.configs.recommended.rules,
  },
})
```
