<template>
  <div class="marketplace-page">
    <MainNavigation>
      <template #status>
        <MarketplaceStatus v-if="isAuthenticated" />
      </template>
    </MainNavigation>

    <main class="page-content">
      <MarketplaceSettings @install="handlePackageInstall" />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import MainNavigation from '../components/MainNavigation.vue';
import MarketplaceSettings from '../components/MarketplaceSettings.vue';
import MarketplaceStatus from '../components/MarketplaceStatus.vue';
import type { MarketplacePackage } from '../services/marketplace-client';
import { useMarketplace } from '../composables/useMarketplace';

const router = useRouter();
const { isAuthenticated } = useMarketplace();

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
        path: '/edit',
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
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.page-content {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}
</style>
