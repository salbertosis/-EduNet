import { invoke } from '@tauri-apps/api/tauri';

export interface Credenciales {
  username: string;
  password: string;
}

export interface UsuarioDB {
  id: number;
  username: string;
  role: string;
}

export async function autenticarUsuario(credenciales: Credenciales): Promise<{ usuario: UsuarioDB; token: string } | null> {
  try {
    // Llama al comando Tauri
    const usuario = await invoke<null | UsuarioDB>('autenticar_usuario_tauri', { credenciales });
    if (!usuario) return null;
    // Simula un token local (puedes mejorarlo en Rust si lo deseas)
    return {
      usuario,
      token: 'token-local-tauri'
    };
  } catch (error) {
    console.error('Error en autenticación:', error);
    return null;
  }
} 