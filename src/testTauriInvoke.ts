import { invoke } from '@tauri-apps/api/tauri';

export async function testInvokeTauri() {
    console.log('[TEST] Iniciando testInvokeTauri...');
    // Prueba: snake_case correcto con id 4
    try {
        const res1 = await invoke('obtener_asignaturas_pendientes_estudiante_v2', { id_estudiante: 4 });
        console.log('[TEST] snake_case OK:', res1);
    } catch (e) {
        console.error('[TEST] snake_case ERROR:', e);
    }
}

// Puedes llamar a testInvokeTauri() desde App.tsx o desde la consola para probar. 