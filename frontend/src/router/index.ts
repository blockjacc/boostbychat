import { createRouter, createWebHistory } from 'vue-router';

/* page-level components */
import HomePage       from '@/pages/HomePage.vue';
import AddEmployee    from '@/pages/AddEmployee.vue';
import VerifyEmployee from '@/pages/VerifyEmployee.vue';

const routes = [
  { path: '/',         component: HomePage },
  { path: '/add',      component: AddEmployee },
  { path: '/verify',   component: VerifyEmployee },
  // fallback → 404
  { path: '/:pathMatch(.*)*', redirect: '/' },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});
