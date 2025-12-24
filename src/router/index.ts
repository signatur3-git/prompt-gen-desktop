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
    path: '/library',
    name: 'Library',
    component: () => import('../pages/LibraryPage.vue'),
  },
  {
    path: '/generate',
    name: 'Generate',
    component: () => import('../pages/GeneratePage.vue'),
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
