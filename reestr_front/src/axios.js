import axios from 'axios'
import {useAuthStore} from '@/store/auth.js'

const apiClient = axios.create({
    baseURL: 'http://localhost:8080', // URL вашего бэкенда
    // baseURL: import.meta.env.VITE_API_BASE_URL,
    // baseURL: 'https://image.alchemyidea.ru',
    headers: {
        'Content-Type': 'application/json'
    }
})
// Глобальный обработчик ошибок
apiClient.interceptors.response.use(
    (response) => response,
    (error) => {
        if (error.response && error.response.status === 401) {
            const authStore = useAuthStore()
            authStore.logout() // Разлогиниваем пользователя
            window.location.href = '/login' // Перенаправляем на страницу входа
        }
        return Promise.reject(error)
    }
)
export default apiClient
