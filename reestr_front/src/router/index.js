import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/store/auth.js'
import Login from '@/views/Login.vue'
import ContractView from '@/views/ContractView.vue'
import OrganizationView from '@/views/OrganizationView.vue'
import ResponsiblePersonView from '@/views/ResponsiblePersonView.vue'

// Lazy loading components
const AppLayout = () => import('@/layouts/AppLayout.vue')
const About = () => import('@/views/AboutView.vue')
const NotFound = () => import('@/views/NotFoundView.vue')
const Users = () => import('@/views/admin/UsersView.vue')

const routes = [
  { path: '/', redirect: '/about' },
  {
    path: '/',
    component: AppLayout,
    children: [
      { path: 'about', component: About },
      { path: 'login', name: 'login', component: Login },

      // Admin-only pages
      {
        path: 'admin/users',
        name: 'users',
        component: Users,
        meta: { requiresAuth: true, requiresAdmin: true },
      },
      {
        path: 'contracts',
        name: 'contracts',
        component: ContractView,
        meta: { requiresAuth: true },
      },
      {
        path: 'organizations',
        name: 'organizations',
        component: OrganizationView,
        meta: { requiresAuth: true },
      },
      {
        path: 'responsibleperson',
        name: 'responsiblepersons',
        component: ResponsiblePersonView,
        meta: { requiresAuth: true },
      },

      { path: '404', name: 'not-found', component: NotFound },
    ],
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/404',
    component: NotFound,
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()

  if (to.meta.requiresAuth && !authStore.token) {
    // Неавторизованные пользователи перенаправляются на login
    return next('/login')
  }

  if (to.meta.requiresAdmin && authStore.user.role !== 'admin') {
    // Пользователи без прав администратора перенаправляются на about
    return next('/about')
  }
  if (to.meta.requiresManager && authStore.user.role !== 'manager') {
    // Пользователи без прав администратора перенаправляются на about
    return next('/about')
  }

  next()
})

export default router
