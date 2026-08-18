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
    const status = error.response?.status
    const errorText = String(
      error.response?.data?.error ||
        (typeof error.response?.data === 'string' ? error.response.data : ''),
    ).toLowerCase()

    // Логин отвечает 401 «Invalid credentials» — там разлогинивать нечего.
    const tokenRejected = status === 401 && errorText.includes('token')
    // Бэкенд проверяет is_active на каждом запросе: отключённая учётная запись
    // получает 403 уже с валидным токеном.
    const accountDeactivated = status === 403 && errorText.includes('deactivated')

    if (tokenRejected || accountDeactivated) {
      const authStore = useAuthStore()
      authStore.logout()
      window.location.href = '/login'
    }

    return Promise.reject(error)
  },
)
export default apiClient
