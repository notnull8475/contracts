import {defineStore} from 'pinia'
import apiClient from '@/axios' // Импортируем отдельный экземпляр Axios

export const useAuthStore = defineStore('auth', {
    state: () => ({
        token: null,
        user: {
            login: null,
            username: null,
            id: null,
            role: null
        }
    }),
    actions: {
        async login(credentials) {
            try {
                const response = await apiClient.post('/api/v1/login', credentials)
                console.log(response)
                this.token = response.data.token

                this.initUser(this.token)

                localStorage.setItem('token', this.token) // Сохраняем токен в localStorage
                apiClient.defaults.headers.common['Authorization'] = `Bearer ${this.token}` // Устанавливаем токен в заголовки
            } catch (error) {
                throw new Error('Ошибка авторизации')
            }
        },
        logout() {
            this.token = null
            this.user = null
            localStorage.removeItem('token')
            localStorage.removeItem('user')
            delete apiClient.defaults.headers.common['Authorization']
        },
        initialize() {
            const token = localStorage.getItem('token')
            if (token) {
                this.token = token
                this.initUser(token)
                apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`
            }
        },
        initUser(token) {
            const decodedData = this.parseJwt(this.token)

            if (decodedData) {
                this.user = {
                    id: decodedData.id,
                    login: decodedData.login,
                    username: decodedData.username,
                    department: decodedData.department,
                    role: decodedData.role
                }
            }
        },
        parseJwt(token) {
            try {
                // Разделяем токен на части
                const base64Url = token.split('.')[1] // Берем часть payload
                const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/') // Исправляем символы для корректного декодирования
                const jsonPayload = decodeURIComponent(
                    atob(base64)
                        .split('')
                        .map(c => `%${c.charCodeAt(0).toString(16).padStart(2, '0')}`)
                        .join('')
                )
                return JSON.parse(jsonPayload) // Преобразуем в объект
            } catch (error) {
                console.error('Ошибка при парсинге JWT:', error)
                return null
            }
        }
    }
})

