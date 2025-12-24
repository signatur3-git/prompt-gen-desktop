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

const router = useRouter();

/**
 * Handle package installation from marketplace
 */
async function handlePackageInstall(pkg: MarketplacePackage, version: string) {
  try {
    // Download package YAML
    const yamlContent = await downloadPackage(pkg, version);

    // Install to library using the library service
    const { installPackageToLibrary } = await import('../services/package-library.service');

    // Create a package object for the library
    const packageToInstall = {
      id: `${pkg.namespace}.${pkg.name}`,
      version: version,
      metadata: {
        name: pkg.name,
        description: pkg.description || '',
        authors: pkg.author ? [pkg.author] : [],
      },
      namespaces: {} // Will be populated when package is loaded from library
    };

    // Install to library
    const entry = await installPackageToLibrary(packageToInstall, yamlContent, 'marketplace');

    console.log('✅ Package installed to library:', entry);

    // Show success with options
    const openInEditor = confirm(
      `✅ Package installed successfully!\n\n` +
      `${pkg.namespace}/${pkg.name} v${version}\n\n` +
      `Would you like to open it in the editor now?`
    );

    if (openInEditor) {
      // Navigate to editor and load from library
      router.push({
        path: '/',
        query: { loadLibraryPackage: `${entry.id}@${entry.version}` }
      });
    } else {
      // Ask if they want to view library
      const viewLibrary = confirm('View in Library instead?');
      if (viewLibrary) {
        router.push('/library');
      }
    }
  } catch (error) {
    console.error('❌ Package installation failed:', error);
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
