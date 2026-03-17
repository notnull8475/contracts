import axios from 'axios'
import { useAuthStore } from '@/store/auth.js'

// В dev-режиме (npm run dev) используем localhost:8080 напрямую.
// В production baseURL пустой — запросы идут на тот же хост,
// nginx проксирует /api/ на 127.0.0.1:8080 автоматически.
const isDev = import.meta.env.DEV

const apiClient = axios.create({
  baseURL: isDev ? 'http://localhost:8080' : '',
})
// Глобальный обработчик ошибок
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response && error.response.status === 401) {
      const errorText =
        error.response?.data?.error ||
        (typeof error.response?.data === 'string' ? error.response.data : '')

      if (String(errorText).toLowerCase().includes('token')) {
        const authStore = useAuthStore()
        authStore.logout() // Разлогиниваем пользователя
        window.location.href = '/login' // Перенаправляем на страницу входа
      }
    }
    return Promise.reject(error)
  },
)
export default apiClient
