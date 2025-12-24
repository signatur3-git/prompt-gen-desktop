import { invoke } from '@tauri-apps/api/core';

/**
 * Small helper to retrieve useful debug info from the Rust side in release builds
 * (where DevTools may not be available).
 */
export async function getDebugInfo(): Promise<string> {
  try {
    return await invoke<string>('debug_info');
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    return `debug_info unavailable: ${msg}`;
  }
}

