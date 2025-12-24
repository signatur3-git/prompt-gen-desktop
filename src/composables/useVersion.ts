/**
 * Version composable - automatically synced with package.json
 * This ensures we only maintain version in one place
 */

// @ts-ignore - Vite provides this at build time
import packageJson from '../../package.json'

export function useVersion() {
  const version = packageJson.version as string

  return {
    version,
    versionDisplay: `v${version}`,
  }
}

