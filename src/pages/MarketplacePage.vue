<template>
  <div class="marketplace-page">
    <header class="page-header">
      <button @click="$router.push('/')" class="back-button">
        ← Back to Editor
      </button>
      <h1>📦 Marketplace</h1>
    </header>

    <main class="page-content">
      <MarketplaceSettings @install="handlePackageInstall" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import MarketplaceSettings from '../components/MarketplaceSettings.vue';
import type { MarketplacePackage } from '../services/marketplace-client';
import { writeTextFile, mkdir } from '@tauri-apps/plugin-fs';
import { join, appDataDir } from '@tauri-apps/api/path';

const router = useRouter();

/**
 * Handle package installation from marketplace
 */
async function handlePackageInstall(pkg: MarketplacePackage, version: string) {
  try {
    // Download package YAML
    const yamlContent = await downloadPackage(pkg, version);

    // Save to app data directory
    const appDir = await appDataDir();
    const packagesDir = await join(appDir, 'packages');

    // Create packages directory if it doesn't exist
    try {
      await mkdir(packagesDir, { recursive: true });
    } catch (error) {
      console.log('Packages directory exists or created');
    }

    const fileName = `${pkg.namespace}.${pkg.name}.yaml`;
    const filePath = await join(packagesDir, fileName);

    // Save package file
    await writeTextFile(filePath, yamlContent);

    // Show success message
    alert(
      `Package ${pkg.namespace}/${pkg.name} v${version} installed successfully!\nLocation: ${filePath}`
    );

    // Optionally navigate to editor and load
    const shouldLoad = confirm('Would you like to open the installed package in the editor?');
    if (shouldLoad) {
      // Navigate to editor with package path as query param
      router.push({ path: '/', query: { loadPackage: filePath } });
    }
  } catch (error) {
    console.error('Package installation failed:', error);
    alert(`Failed to install package: ${(error as Error).message}`);
  }
}

/**
 * Download package YAML from marketplace
 */
async function downloadPackage(pkg: MarketplacePackage, version: string): Promise<string> {
  // Use marketplace client to download
  const { marketplaceClient } = await import('../services/marketplace-client');
  return marketplaceClient.downloadPackage(pkg.namespace, pkg.name, version);
}
</script>

<style scoped>
.marketplace-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: inherit;
  color: inherit;
}

.page-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  background-color: var(--bg-secondary, #f5f5f5);
  border-bottom: 1px solid var(--border-color, #e0e0e0);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.back-button {
  padding: 0.5rem 1rem;
  background-color: var(--button-bg, #ffffff);
  color: var(--text-primary, #1a1a1a);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
}

.back-button:hover {
  background-color: var(--button-hover-bg, #f0f0f0);
  border-color: var(--border-hover, #c0c0c0);
}

.page-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary, #1a1a1a);
}

.page-content {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
  background-color: inherit;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .marketplace-page {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2a2a2a;
    --text-primary: #e0e0e0;
    --text-secondary: #a0a0a0;
    --border-color: #404040;
    --button-bg: #2a2a2a;
    --button-hover-bg: #353535;
    --border-hover: #505050;
  }

  .page-header {
    background-color: var(--bg-secondary);
    border-bottom-color: var(--border-color);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .back-button {
    background-color: var(--button-bg);
    color: var(--text-primary);
    border-color: var(--border-color);
  }

  .back-button:hover {
    background-color: var(--button-hover-bg);
    border-color: var(--border-hover);
  }

  .page-header h1 {
    color: var(--text-primary);
  }
}
</style>

