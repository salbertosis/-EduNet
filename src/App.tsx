import { HashRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './componentes/plantillas/Layout';
import { Dashboard } from './modulos/dashboard/paginas/Dashboard';
import { ListaEstudiantes } from './modulos/estudiantes/paginas/ListaEstudiantes';
import { MensajeGlobalProvider, useMensajeGlobal } from "./componentes/MensajeGlobalContext";
import { ModalMensaje } from "./componentes/ModalMensaje";
import { DetalleCalificacionesPage } from './modulos/calificaciones/paginas/DetalleCalificacionesPage';
import { ListaDocentes } from './modulos/docentes/paginas/ListaDocentes';
import { ListaCalificaciones } from './modulos/calificaciones/paginas/ListaCalificaciones';
import Reportes from './modulos/reportes';
import { ListaGrados } from './modulos/grados/paginas/ListaGrados';
import CrearPeriodoEscolar from './componentes/CrearPeriodoEscolar';

function MensajeGlobalModal() {
  const { mensaje, cerrarMensaje } = useMensajeGlobal();
  if (!mensaje.abierto) return null;
  return (
    <ModalMensaje mensaje={mensaje.mensaje} tipo={mensaje.tipo} onClose={cerrarMensaje} />
  );
}

export function App() {
  return (
    <MensajeGlobalProvider>
    <Router>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="/cursos" element={<ListaGrados />} />
          <Route path="estudiantes" element={<ListaEstudiantes />} />
          <Route path="estudiantes/:id/calificaciones" element={<DetalleCalificacionesPage />} />
          <Route path="docentes" element={<ListaDocentes />} />
          <Route path="calificaciones" element={<ListaCalificaciones />} />
          <Route path="reportes" element={<Reportes />} />
          <Route path="nuevo-periodo" element={<CrearPeriodoEscolar />} />
          <Route path="*" element={<Navigate to="/dashboard" replace />} />
        </Route>
      </Routes>
    </Router>
      <MensajeGlobalModal />
    </MensajeGlobalProvider>
  );
}

export default App;
