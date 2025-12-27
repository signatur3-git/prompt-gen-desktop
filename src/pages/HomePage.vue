<template>
  <div class="home-page">
    <MainNavigation>
      <template #status>
        <MarketplaceStatus v-if="isAuthenticated" />
      </template>
    </MainNavigation>

    <div class="home-content">
      <header class="home-header">
        <h1>Random Prompt Generator</h1>
        <p class="subtitle">Desktop Application v{{ APP_VERSION }}</p>
      </header>

      <div class="navigation-cards">
        <router-link to="/generate" class="nav-card">
          <div class="nav-icon">⚡</div>
          <h2>Generate</h2>
          <p>Create prompts from your package library</p>
        </router-link>

        <router-link to="/edit" class="nav-card">
          <div class="nav-icon">✏️</div>
          <h2>Edit</h2>
          <p>Create and edit package files</p>
        </router-link>

        <router-link to="/library" class="nav-card">
          <div class="nav-icon">📚</div>
          <h2>Library</h2>
          <p>Browse and manage your packages</p>
        </router-link>

        <router-link to="/marketplace" class="nav-card">
          <div class="nav-icon">📦</div>
          <h2>Marketplace</h2>
          <p>Download packages from the community</p>
        </router-link>
      </div>

      <div class="quick-actions">
        <h3>Quick Actions</h3>
        <div class="action-buttons">
          <button @click="handleNewPackage" class="btn-action">
            <span class="icon">📄</span>
            New Package
          </button>
          <button @click="handleOpenPackage" class="btn-action">
            <span class="icon">📂</span>
            Open Package
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { open } from '@tauri-apps/plugin-dialog';
import MainNavigation from '../components/MainNavigation.vue';
import MarketplaceStatus from '../components/MarketplaceStatus.vue';
import { useMarketplace } from '../composables/useMarketplace';

const router = useRouter();
const APP_VERSION = '1.0.1';
const { isAuthenticated } = useMarketplace();

async function handleNewPackage() {
  // Navigate to edit page which will show the new package dialog
  router.push('/edit?action=new');
}

async function handleOpenPackage() {
  try {
    const selected = await open({
      title: 'Open Package File',
      multiple: false,
      filters: [
        {
          name: 'YAML',
          extensions: ['yaml', 'yml'],
        },
        {
          name: 'All Files',
          extensions: ['*'],
        },
      ],
    });

    if (selected) {
      // Navigate to edit page with the loaded package
      router.push({
        path: '/edit',
        query: { action: 'load', path: selected }
      });
    }
  } catch (error) {
    console.error('Error opening package:', error);
    alert(`Failed to open package: ${error}`);
  }
}
</script>

<style scoped>
.home-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.home-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start; /* Changed from center to prevent overlap */
  max-width: 1200px;
  width: 100%;
  margin: 0 auto;
  padding: 3rem 2rem; /* Increased top padding */
  overflow-y: auto;
  background: linear-gradient(135deg, var(--bg-primary) 0%, var(--bg-secondary) 100%);
}

.home-header {
  text-align: center;
  margin-bottom: 3rem;
  flex-shrink: 0; /* Prevent header from shrinking */
}

.home-header h1 {
  font-size: 3rem;
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
}

.subtitle {
  font-size: 1.25rem;
  color: var(--text-secondary);
  margin: 0;
}

.navigation-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin-bottom: 3rem;
  width: 100%; /* Ensure cards take full width */
  flex-shrink: 0; /* Prevent cards from shrinking */
}

.nav-card {
  background-color: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 12px;
  padding: 2rem;
  text-align: center;
  text-decoration: none;
  color: var(--text-primary);
  transition: all 0.3s;
  cursor: pointer;
}

.nav-card:hover {
  border-color: var(--accent-color);
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
}

.nav-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.nav-card h2 {
  font-size: 1.5rem;
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.nav-card p {
  font-size: 1rem;
  color: var(--text-secondary);
  margin: 0;
}

.quick-actions {
  text-align: center;
  padding: 2rem;
  background-color: var(--bg-secondary);
  border-radius: 12px;
  border: 1px solid var(--border-color);
  width: 100%;
  flex-shrink: 0; /* Prevent quick actions from shrinking */
}

.quick-actions h3 {
  margin: 0 0 1.5rem 0;
  color: var(--text-primary);
  font-size: 1.25rem;
}

.action-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
}

.btn-action {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background-color: var(--button-bg);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-action:hover {
  background-color: var(--button-hover-bg);
  border-color: var(--accent-color);
}

.btn-action .icon {
  font-size: 1.25rem;
}

@media (max-width: 768px) {
  .home-header h1 {
    font-size: 2rem;
  }

  .navigation-cards {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .nav-card {
    padding: 1.5rem;
  }
}
</style>
