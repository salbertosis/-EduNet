import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { Sesion, Usuario } from '../tipos';

interface EstadoAplicacion {
  sesion: Sesion | null;
  tema: 'claro' | 'oscuro';
  cargando: boolean;
  error: string | null;
  
  // Acciones
  iniciarSesion: (sesion: Sesion) => void;
  cerrarSesion: () => void;
  establecerTema: (tema: 'claro' | 'oscuro') => void;
  establecerCargando: (cargando: boolean) => void;
  establecerError: (error: string | null) => void;
}

export const useStore = create<EstadoAplicacion>()(
  persist(
    (set) => ({
      sesion: null,
      tema: 'claro',
      cargando: false,
      error: null,

      iniciarSesion: (sesion) => set({ sesion, error: null }),
      cerrarSesion: () => set({ sesion: null }),
      establecerTema: (tema) => set({ tema }),
      establecerCargando: (cargando) => set({ cargando }),
      establecerError: (error) => set({ error }),
    }),
    {
      name: 'edunet-storage',
      partialize: (state) => ({
        tema: state.tema,
        sesion: state.sesion,
      }),
    }
  )
); 