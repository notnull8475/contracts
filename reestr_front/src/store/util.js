import { defineStore } from 'pinia'
import axios from '@/axios.js'

export const useRequtil = defineStore('requtil', {
  state: () => ({}),
  actions: {
    /**
     * Выполняет POST-запрос
     * @param {string} url - URL для запроса
     * @param {Object} data - Данные для отправки
     * @param {string} errorMessage - Сообщение об ошибке
     * @returns {Promise<any>} - Результат запроса
     */
    async makePostRequest(url, data, errorMessage) {
      try {
        const response = await axios.post(url, data)
        return response.data
      } catch (error) {
        console.error('Ошибка в makePostRequest:', error.response?.status, error.message)

        const serverError =
          error.response?.data?.error || errorMessage || 'Ошибка при выполнении POST-запроса'

        throw new Error(serverError)
      }
    },

    /**
     * Выполняет GET-запрос
     * @param {string} url - URL для запроса
     * @param {string} params - Параметры запроса
     * @param {string} errorMessage - Сообщение об ошибке
     * @returns {Promise<any>} - Результат запроса
     */
    async makeGetRequest(url, params, errorMessage) {
      try {
        const response = await axios.get(`${url}${params}`)
        return response.data
      } catch (error) {
        console.error('Ошибка в makeGetRequest:', error.response?.status, error.message)
        throw new Error(errorMessage || 'Ошибка при выполнении GET-запроса')
      }
    },

    /**
     * Выполняет DELETE-запрос
     * @param {string} url - URL для запроса
     * @param {string} params - Параметры запроса
     * @param {string} errorMessage - Сообщение об ошибке
     * @returns {Promise<any>} - Результат запроса
     */
    async makeDeleteRequest(url, params, errorMessage) {
      try {
        const response = await axios.delete(`${url}${params}`)
        return response.data
      } catch (error) {
        console.error('Ошибка в makeDeleteRequest:', error.response?.status, error.message)
        throw new Error(errorMessage || 'Ошибка при выполнении DELETE-запроса')
      }
    },
  },
})
