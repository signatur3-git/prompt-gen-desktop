<template>
  <div class="edit-page">
    <MainNavigation>
      <template #status>
        <MarketplaceStatus v-if="isAuthenticated" />
      </template>
    </MainNavigation>

    <PackageEditor />
  </div>
</template>

<script setup lang="ts">
import MainNavigation from '../components/MainNavigation.vue';
import MarketplaceStatus from '../components/MarketplaceStatus.vue';
import PackageEditor from '../components/PackageEditor.vue';
import { useMarketplace } from '../composables/useMarketplace';

const { isAuthenticated } = useMarketplace();
</script>

<style scoped>
.edit-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background-color: var(--bg-primary);
}

/* Ensure PackageEditor takes remaining space after MainNavigation */
.edit-page :deep(.package-editor) {
  flex: 1;
}
</style>

