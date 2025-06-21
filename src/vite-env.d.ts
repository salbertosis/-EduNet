/// <reference types="vite/client" />

declare global {
  interface Window {
    __TAURI__: {
      invoke: (command: string, args?: any) => Promise<any>;
      convertFileSrc: (src: string, protocol?: string) => string;
    };
  }
}

export {};
