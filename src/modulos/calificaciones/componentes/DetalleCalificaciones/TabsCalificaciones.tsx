import React from 'react';

interface Tab {
  key: string;
  label: string;
}

interface TabsCalificacionesProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (key: string) => void;
}

export const TabsCalificaciones: React.FC<TabsCalificacionesProps> = ({ tabs, activeTab, onTabChange }) => {
  return (
    <div className="flex gap-2 border-b-2 border-emerald-400 dark:border-emerald-700 mb-8">
      {tabs.map(t => (
        <button
          key={t.key}
          onClick={() => onTabChange(t.key)}
          className={`px-4 py-2 font-semibold rounded-t-lg transition-all duration-200 focus:outline-none text-base tracking-wide shadow-sm ${activeTab === t.key ? 'bg-gradient-to-r from-emerald-400 via-cyan-400 to-blue-400 text-white shadow-emerald-800/30' : 'bg-gray-100 dark:bg-dark-700 text-gray-600 dark:text-gray-300 hover:bg-emerald-900/30 hover:text-emerald-300'}`}
          style={{marginBottom: '-2px'}}>
          {t.label}
        </button>
      ))}
    </div>
  );
}; 