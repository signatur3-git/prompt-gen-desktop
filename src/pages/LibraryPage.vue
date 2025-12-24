<template>
  <div class="library-page">
    <header class="page-header">
      <div class="header-content">
        <button @click="$router.push('/')" class="back-button">
          ← Back to Editor
        </button>
        <h1>📚 Package Library</h1>
      </div>
      <div class="header-actions">
        <button @click="handleRefresh" class="btn-secondary" :disabled="loading">
          🔄 Refresh
        </button>
        <button @click="handleImport" class="btn-secondary">
          📥 Import Package
        </button>
      </div>
    </header>

    <main class="page-content">
      <PackageLibraryBrowser
        :packages="packages"
        :loading="loading"
        :error="error"
        @load="handleLoad"
        @delete="handleDelete"
        @export="handleExport"
      />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import PackageLibraryBrowser from '../components/PackageLibraryBrowser.vue';
import { listLibraryPackages, removePackageFromLibrary, refreshLibrary, type LibraryEntry } from '../services/package-library.service';
import { open } from '@tauri-apps/plugin-dialog';

const router = useRouter();
const packages = ref<LibraryEntry[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);

onMounted(async () => {
  await loadPackages();
});

async function loadPackages() {
  try {
    loading.value = true;
    error.value = null;
    packages.value = await listLibraryPackages();
  } catch (e) {
    error.value = (e as Error).message;
    console.error('Failed to load library packages:', e);
  } finally {
    loading.value = false;
  }
}

async function handleRefresh() {
  try {
    loading.value = true;
    error.value = null;
    packages.value = await refreshLibrary();
  } catch (e) {
    error.value = (e as Error).message;
    console.error('Failed to refresh library:', e);
  } finally {
    loading.value = false;
  }
}

async function handleLoad(entry: LibraryEntry) {
  // Navigate to editor with package loaded
  router.push({
    path: '/',
    query: { loadLibraryPackage: `${entry.id}@${entry.version}` }
  });
}

async function handleDelete(entry: LibraryEntry) {
  const confirmed = confirm(
    `Are you sure you want to delete ${entry.name} v${entry.version}?\n\nThis will permanently remove the package from your library.`
  );

  if (!confirmed) return;

  try {
    await removePackageFromLibrary(entry.id, entry.version);
    await loadPackages(); // Reload list
    alert(`Package ${entry.name} deleted successfully.`);
  } catch (e) {
    error.value = (e as Error).message;
    console.error('Failed to delete package:', e);
    alert(`Failed to delete package: ${(e as Error).message}`);
  }
}

async function handleExport(entry: LibraryEntry) {
  // TODO: Implement export functionality
  alert('Export feature coming soon!');
}

async function handleImport() {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Package Files',
        extensions: ['yaml', 'yml']
      }]
    });

    if (!selected) return;

    // Read the file content
    const { readTextFile } = await import('@tauri-apps/plugin-fs');
    const yamlContent = await readTextFile(selected);

    // Parse the package to get metadata
    const { invoke } = await import('@tauri-apps/api/core');
    const pkg = await invoke('load_package', { path: selected });

    // Install to library as 'imported' source
    const { installPackageToLibrary } = await import('../services/package-library.service');
    const entry = await installPackageToLibrary(pkg, yamlContent, 'imported');

    // Reload the library to show the new package
    await loadPackages();

    // Show success message
    alert(`✅ Package imported successfully!\n\n${entry.name} v${entry.version}\n\nThe package is now available in your library.`);
  } catch (e) {
    console.error('Failed to import package:', e);
    alert(`❌ Failed to import package: ${(e as Error).message}`);
  }
}
</script>

<style scoped>
.library-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.header-content {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.back-button {
  padding: 0.5rem 1rem;
  background-color: var(--button-bg);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
}

.back-button:hover {
  background-color: var(--button-hover-bg);
  border-color: var(--border-hover);
}

.page-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-secondary {
  padding: 0.5rem 1rem;
  background-color: var(--button-secondary-bg);
  color: var(--text-primary);
  border: none;
  border-radius: 6px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover:not(:disabled) {
  background-color: var(--button-secondary-hover);
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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

