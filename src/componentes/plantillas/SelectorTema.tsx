import { useState, useEffect } from 'react';
import { Moon, Sun } from 'lucide-react';

export function SelectorTema() {
  const [tema, setTema] = useState<'claro' | 'oscuro'>(() => {
    if (typeof window !== 'undefined') {
      return document.documentElement.classList.contains('dark') ? 'oscuro' : 'claro';
    }
    return 'claro';
  });

  useEffect(() => {
    const root = document.documentElement;
    if (tema === 'oscuro') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
  }, [tema]);

  return (
    <button
      onClick={() => setTema(tema === 'claro' ? 'oscuro' : 'claro')}
      className="p-3 rounded-xl bg-gradient-to-r from-emerald-700 via-cyan-700 to-blue-700 border-2 border-emerald-400 shadow-lg shadow-emerald-900/20 hover:scale-105 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-emerald-400"
      aria-label={`Cambiar a tema ${tema === 'claro' ? 'oscuro' : 'claro'}`}
    >
      {tema === 'claro' ? (
        <Moon className="w-6 h-6 text-emerald-200 drop-shadow" />
      ) : (
        <Sun className="w-6 h-6 text-yellow-300 drop-shadow" />
      )}
    </button>
  );
} 