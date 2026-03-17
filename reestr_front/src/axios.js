import axios from 'axios'
import { useAuthStore } from '@/store/auth.js'

const apiClient = axios.create({
  baseURL: 'http://localhost:8080', // URL вашего бэкенда
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
