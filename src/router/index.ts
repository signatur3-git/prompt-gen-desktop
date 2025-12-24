import { createRouter, createWebHashHistory } from 'vue-router';
import PackageEditor from '../components/PackageEditor.vue';
import MarketplacePage from '../pages/MarketplacePage.vue';

const routes = [
  {
    path: '/',
    name: 'Editor',
    component: PackageEditor,
  },
  {
    path: '/marketplace',
    name: 'Marketplace',
    component: MarketplacePage,
  },
];

const router = createRouter({
  // Use hash mode for Tauri (file:// protocol)
  history: createWebHashHistory(),
  routes,
});

export default router;

