import { defineStore } from 'pinia'
import { requtil } from '@/store/util.js'

let validTypesRequest = '/api/v1/types/validity'
export const ValidTypeUtil = defineStore('validType', {
  state: () => ({}),
  actions: {
    async addValidType(newValidType) {
      return requtil.makePostRequest(validTypesRequest + '/add', newValidType, 'Ошибка добавления договора')
    },
    async updateValidType(validTypeData) {
      return requtil.makePostRequest(validTypesRequest + '/update',validTypeData,'Ошибка обновления договора',)
    },
    async delValidType(validTypeID) {
      return requtil.makeGetRequest(validTypesRequest + '/del', validTypeID, 'Ошибка удаления договора')
    },

    async getValidType(validTypeID) {
      return requtil.makeGetRequest(validTypesRequest + 'update', validTypeID, 'Ошибка получения договора')
    },
    async getValidTypes() {
      return requtil.makeGetRequest(validTypesRequest + 'update','','Ошибка получения списка договоров',)
    },
  },
})
