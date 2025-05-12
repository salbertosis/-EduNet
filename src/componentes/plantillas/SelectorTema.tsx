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
      className="p-2 rounded-lg bg-gray-100 dark:bg-dark-800 hover:bg-gray-200 dark:hover:bg-dark-700 transition-colors"
      aria-label={`Cambiar a tema ${tema === 'claro' ? 'oscuro' : 'claro'}`}
    >
      {tema === 'claro' ? (
        <Moon className="w-5 h-5 text-gray-700" />
      ) : (
        <Sun className="w-5 h-5 text-yellow-400" />
      )}
    </button>
  );
} 