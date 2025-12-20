<script setup lang="ts">
import { ref } from 'vue'
import PackageEditor from './components/PackageEditor.vue'

const appName = 'RPG Desktop'
const version = 'v1.0.0'
</script>

<template>
  <div class="app">
    <PackageEditor />
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}
</style>


