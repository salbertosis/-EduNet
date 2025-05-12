import { HashRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Layout } from './componentes/plantillas/Layout';
import { Dashboard } from './modulos/dashboard/paginas/Dashboard';
import { ListaEstudiantes } from './modulos/estudiantes/paginas/ListaEstudiantes';

export function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Navigate to="/dashboard" replace />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="estudiantes" element={<ListaEstudiantes />} />
          <Route path="*" element={<Navigate to="/dashboard" replace />} />
        </Route>
      </Routes>
    </Router>
  );
}

export default App;
