import React, { createContext, useContext, useState, useEffect } from 'react';

type TipoMensaje = 'exito' | 'error' | 'info' | 'advertencia';

interface MensajeGlobal {
  abierto: boolean;
  mensaje: string;
  tipo: TipoMensaje;
}

interface MensajeGlobalContextType {
  mostrarMensaje: (mensaje: string, tipo: TipoMensaje) => void;
  cerrarMensaje: () => void;
  mensaje: MensajeGlobal;
}

const MensajeGlobalContext = createContext<MensajeGlobalContextType | undefined>(undefined);

export function MensajeGlobalProvider({ children }: { children: React.ReactNode }) {
  const [mensaje, setMensaje] = useState<MensajeGlobal>({
    abierto: false,
    mensaje: '',
    tipo: 'info'
  });

  useEffect(() => {
    let timeout: NodeJS.Timeout | undefined;
    if (mensaje.abierto && mensaje.tipo === 'exito') {
      timeout = setTimeout(() => {
        setMensaje(prev => ({ ...prev, abierto: false }));
      }, 2500);
    }
    return () => {
      if (timeout) clearTimeout(timeout);
    };
  }, [mensaje.abierto, mensaje.tipo]);

  const mostrarMensaje = (mensaje: string, tipo: TipoMensaje) => {
    setMensaje({ abierto: true, mensaje, tipo });
  };

  const cerrarMensaje = () => {
    setMensaje(prev => ({ ...prev, abierto: false }));
  };

  return (
    <MensajeGlobalContext.Provider value={{ mostrarMensaje, cerrarMensaje, mensaje }}>
      {children}
    </MensajeGlobalContext.Provider>
  );
}

export function useMensajeGlobal() {
  const context = useContext(MensajeGlobalContext);
  if (context === undefined) {
    throw new Error('useMensajeGlobal debe ser usado dentro de un MensajeGlobalProvider');
  }
  return context;
} 