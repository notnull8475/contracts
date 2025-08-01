// contracts.js
import { defineStore } from 'pinia'
import { useRequtil } from '@/store/util.js'

const contractRequest = '/api/v1/contracts'

export const ContractUtil = defineStore('contract', {
  state: () => ({}),
  actions: {
    async addContract(newContract) {
      const requtil = useRequtil()
      return requtil.makePostRequest(
        contractRequest + '/add',
        newContract,
        'Ошибка добавления договора',
      )
    },
    async updateContract(contractData) {
      const requtil = useRequtil()
      return requtil.makePostRequest(
        contractRequest + '/update',
        contractData,
        'Ошибка обновления договора',
      )
    },
    async delContract(contractID) {
      const requtil = useRequtil()
      return requtil.makeDeleteRequest(
        contractRequest + '/del/',
        contractID,
        'Ошибка удаления договора',
      )
    },
    async getContract(contractID) {
      const requtil = useRequtil()
      return requtil.makeGetRequest(
        contractRequest + '/get/',
        contractID,
        'Ошибка получения договора',
      )
    },
    async getContracts() {
      const requtil = useRequtil()
      return requtil.makeGetRequest(
        contractRequest + '/list',
        '',
        'Ошибка получения списка договоров',
      )
    },
  },
})
