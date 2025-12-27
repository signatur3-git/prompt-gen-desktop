import { createRouter, createWebHashHistory } from 'vue-router';
import HomePage from '../pages/HomePage.vue';
import EditPage from '../pages/EditPage.vue';
import MarketplacePage from '../pages/MarketplacePage.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomePage,
  },
  {
    path: '/edit',
    name: 'Edit',
    component: EditPage,
  },
  {
    path: '/generate',
    name: 'Generate',
    component: () => import('../pages/GeneratePage.vue'),
  },
  {
    path: '/library',
    name: 'Library',
    component: () => import('../pages/LibraryPage.vue'),
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
