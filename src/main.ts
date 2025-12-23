import { createApp } from 'vue'
import App from './App.vue'

// Minimal runtime probe to detect if we're actually running inside Tauri.
// This is intentionally safe: it won't throw if Tauri isn't present.
try {
  const hasTauri = typeof window !== 'undefined' && ('__TAURI__' in window || '__TAURI_IPC__' in window)
  // eslint-disable-next-line no-console
  console.log('[runtime] hasTauriGlobals=', hasTauri)
  // eslint-disable-next-line no-console
  console.log('[runtime] location=', window?.location?.href)
  // eslint-disable-next-line no-console
  console.log('[runtime] userAgent=', navigator?.userAgent)

  // Try a best-effort invoke probe (will only work in Tauri).
  // We import dynamically so browser mode doesn't bundle-crash.
  ;(async () => {
    if (!hasTauri) return
    try {
      const { invoke } = await import('@tauri-apps/api/core')
      // call something that exists in core permissions; this should succeed if IPC works.
      // `getVersion` is a core API, but via invoke probe we just check invoke existence.
      // eslint-disable-next-line no-console
      console.log('[runtime] tauri invoke typeof=', typeof invoke)
    } catch (e) {
      // eslint-disable-next-line no-console
      console.error('[runtime] tauri invoke probe failed:', e)
    }
  })()
} catch (e) {
  // eslint-disable-next-line no-console
  console.error('[runtime] probe error:', e)
}

createApp(App).mount('#app')
