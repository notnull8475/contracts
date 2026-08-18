import { defineStore } from 'pinia'
import axios from '@/axios.js'

/** Сообщение из тела ответа бэкенда, иначе — запасной текст. */
function serverError(error, fallback) {
  const data = error.response?.data
  if (data?.error) return data.error
  if (typeof data === 'string' && data) return data
  return fallback
}

export const UserUtil = defineStore('users', {
  state: () => ({}),
  actions: {
    async addUser(newUser) {
      try {
        const response = await axios.post('/api/v1/admin/users/add', newUser)
        return response.data
      } catch (error) {
        throw new Error(serverError(error, 'Не удалось создать пользователя'))
      }
    },
    async updateUser(newUser) {
      try {
        const response = await axios.post('/api/v1/admin/users/update', newUser)
        return response.data
      } catch (error) {
        throw new Error(serverError(error, 'Не удалось обновить пользователя'))
      }
    },
    async deleteUser(userId) {
      try {
        const response = await axios.delete(`/api/v1/admin/users/delete/${userId}`)
        return response.data
      } catch (error) {
        throw new Error(serverError(error, 'Не удалось удалить пользователя'))
      }
    },
    async getAllUsers() {
      try {
        const response = await axios.get('/api/v1/admin/users/get/list')
        return response.data
      } catch (error) {
        throw new Error(serverError(error, 'Не удалось получить список пользователей'))
      }
    },

    async getRoles() {
      try {
        const response = await axios.get('/api/v1/roles/get')
        return response.data
      } catch (error) {
        throw new Error(serverError(error, 'Не удалось получить список ролей'))
      }
    },
  },
})
