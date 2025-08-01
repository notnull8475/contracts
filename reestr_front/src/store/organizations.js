import { defineStore } from 'pinia'
import { useRequtil } from '@/store/util.js'

const orgRequest = '/api/v1/organizations'

export const OrganizationUtil = defineStore('organization', {
  state: () => ({ organizations: [] }),
  actions: {
    /**
     * Добавляет новую организацию
     * @param {Object} newOrg - Данные новой организации
     * @returns {Promise<any>} - Результат запроса
     */
    async addOrganization(newOrg) {
      const requtil = useRequtil()
      return requtil.makePostRequest(`${orgRequest}/add`, newOrg, 'Ошибка добавления организации')
    },

    /**
     * Обновляет существующую организацию
     * @param {Object} OrgData - Данные для обновления
     * @returns {Promise<any>} - Результат запроса
     */
    async updateOrganization(OrgData) {
      const requtil = useRequtil()
      return requtil.makePostRequest(
        `${orgRequest}/update`,
        OrgData,
        'Ошибка обновления организации',
      )
    },

    /**
     * Удаляет организацию по ID
     * @param {string|number} OrgID - ID организации
     * @returns {Promise<any>} - Результат запроса
     */
    async delOrganization(OrgID) {
      const requtil = useRequtil()
      return requtil.makeDeleteRequest(`${orgRequest}/del/`, OrgID, 'Ошибка удаления организации')
    },

    /**
     * Получает организацию по ID
     * @param {string|number} OrgID - ID организации
     * @returns {Promise<any>} - Результат запроса
     */
    async getOrganization(OrgID) {
      const requtil = useRequtil()
      return requtil.makeGetRequest(`${orgRequest}/get/`, OrgID, 'Ошибка получения организации')
    },

    /**
     * Получает список всех организаций
     * @returns {Promise<any>} - Результат запроса
     */
    async getOrganizations() {
      const requtil = useRequtil()
      return requtil.makeGetRequest(`${orgRequest}/list`, '', 'Ошибка получения списка организаций')
    },
  },
})
