import { useState } from 'react';
import ActasEvaluacion from './componentes/ActasEvaluacion';
import ActaResumen from './componentes/ActaResumen';

const TABS = [
  { key: 'actas', label: 'Actas de Evaluación' },
  { key: 'actas_resumen', label: 'Planilla de Calificaciones' },
  { key: 'boletines', label: 'Boletines' },
  { key: 'resumen', label: 'Resumen Final' },
  { key: 'certificaciones', label: 'Certificaciones' },
  { key: 'titulos', label: 'Títulos' },
];

export default function Reportes() {
  const [tab, setTab] = useState('actas');

  return (
    <div className="max-w-5xl mx-auto p-4 md:p-8 bg-white dark:bg-dark-800 rounded-2xl shadow-lg mt-8">
      <h2 className="text-2xl font-bold mb-6 text-emerald-700 dark:text-cyan-200">Reportes</h2>
      <div className="flex gap-2 border-b-2 border-emerald-400 dark:border-emerald-700 mb-8">
        {TABS.map(t => (
          <button
            key={t.key}
            onClick={() => setTab(t.key)}
            className={`px-4 py-2 font-semibold rounded-t-lg transition-all duration-200 focus:outline-none text-base tracking-wide shadow-sm ${tab === t.key ? 'bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 text-white shadow-emerald-800/30' : 'bg-gray-100 dark:bg-dark-700 text-gray-600 dark:text-gray-300 hover:bg-emerald-900/30 hover:text-emerald-300'}`}
            style={{marginBottom: '-2px'}}>
            {t.label}
          </button>
        ))}
      </div>
      <div className="mt-6">
        {tab === 'actas' && <ActasEvaluacion />}
        {tab === 'actas_resumen' && <ActaResumen />}
        {tab === 'boletines' && <div>Boletines (próximamente)</div>}
        {tab === 'resumen' && <div>Resumen Final (próximamente)</div>}
        {tab === 'certificaciones' && <div>Certificaciones (próximamente)</div>}
        {tab === 'titulos' && <div>Títulos (próximamente)</div>}
      </div>
    </div>
  );
} 