<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useKeyboardShortcuts } from './composables/useKeyboardShortcuts';

// App now uses router for navigation
const router = useRouter();

// Setup global navigation keyboard shortcuts
useKeyboardShortcuts([
  {
    key: 'h',
    ctrl: true,
    alt: true,
    description: 'Go to Home',
    handler: () => router.push('/')
  },
  {
    key: 'g',
    ctrl: true,
    alt: true,
    description: 'Go to Generate',
    handler: () => router.push('/generate')
  },
  {
    key: 'e',
    ctrl: true,
    alt: true,
    description: 'Go to Edit',
    handler: () => router.push('/edit')
  },
  {
    key: 'l',
    ctrl: true,
    alt: true,
    description: 'Go to Library',
    handler: () => router.push('/library')
  },
  {
    key: 'm',
    ctrl: true,
    alt: true,
    description: 'Go to Marketplace',
    handler: () => router.push('/marketplace')
  }
]);
</script>

<template>
  <div class="app">
    <router-view />
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}
</style>


