import { defineStore } from 'pinia'
import type { AxiosResponse } from 'axios'
import apiClient from '@/axios'
import type { User } from '@/stores/models/users.ts'

// Интерфейс для данных пользователя


// Интерфейс для ответа на запрос авторизации
interface LoginResponse {
  token: string
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: null as string | null,
    user: null as User | null,
  }),
  actions: {
    /**
     * Авторизация пользователя
     * @param credentials - Данные для входа (логин и пароль)
     */
    async login(credentials: Record<string, any>) {
      try {
        const response: AxiosResponse<LoginResponse> = await apiClient.post(
          '/api/v1/login',
          credentials,
        )
        this.token = response.data.token

        this.initUser(this.token)

        localStorage.setItem('token', this.token) // Сохраняем токен в localStorage
        apiClient.defaults.headers.common['Authorization'] = `Bearer ${this.token}` // Устанавливаем токен в заголовки
      } catch (error) {
        console.error('Ошибка при авторизации:', error)
        throw new Error('Ошибка авторизации')
      }
    },

    /**
     * Выход пользователя из системы
     */
    logout() {
      this.token = null
      this.user = null
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      delete apiClient.defaults.headers.common['Authorization']
    },

    /**
     * Инициализация состояния при запуске приложения
     */
    initialize() {
      const token = localStorage.getItem('token')
      if (token) {
        this.token = token
        this.initUser(token)
        apiClient.defaults.headers.common['Authorization'] = `Bearer ${token}`
      }
    },

    /**
     * Инициализация данных пользователя из JWT
     * @param token - JWT токен
     */
    initUser(token: string) {
      const decodedData = this.parseJwt(token)

      if (decodedData) {
        this.user = {
          id: decodedData.id || null,
          login: decodedData.login || null,
          username: decodedData.username || null,
          role: decodedData.role || null,
          is_active: decodedData.is_active || false,
        }
      }
    },

    /**
     * Парсинг JWT токена
     * @param token - JWT токен
     * @returns - Распарсенные данные или null в случае ошибки
     */
    parseJwt(token: string): Record<string, any> | null {
      try {
        // Разделяем токен на части
        const base64Url = token.split('.')[1] // Берем часть payload
        const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/') // Исправляем символы для корректного декодирования
        const jsonPayload = decodeURIComponent(
          atob(base64)
            .split('')
            .map((c) => `%${c.charCodeAt(0).toString(16).padStart(2, '0')}`)
            .join(''),
        )
        return JSON.parse(jsonPayload) // Преобразуем в объект
      } catch (error) {
        console.error('Ошибка при парсинге JWT:', error)
        return null
      }
    },
  },
})
