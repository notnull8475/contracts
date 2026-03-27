// contracts.js
import { defineStore } from 'pinia'
import { useRequtil } from '@/store/util.js'
import axios from '@/axios.js'

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
    /// Пагинированный список с фильтрами
    async getPaginatedContracts(params = {}) {
      const res = await axios.get(`${contractRequest}/paginated`, { params })
      return res.data
    },
    /// Batch счётчики файлов и доп соглашений
    async getBatchStats() {
      const res = await axios.get(`${contractRequest}/stats`)
      return res.data
    },
    async uploadFile(contractId, file) {
      const requtil = useRequtil()
      return requtil.makeUploadRequest(
        contractRequest + '/files/',
        contractId,
        file,
        'Ошибка загрузки файла',
      )
    },
    async getContractFiles(contractId) {
      const requtil = useRequtil()
      return requtil.makeGetRequest(
        contractRequest + '/files/',
        contractId,
        'Ошибка получения файлов',
      )
    },
    async downloadFile(fileId) {
      const baseUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080'
      window.open(`${baseUrl}/api/v1/contracts/files/download/${fileId}`, '_blank')
    },
    async deleteFile(fileId) {
      const requtil = useRequtil()
      return requtil.makeDeleteRequest(
        contractRequest + '/files/delete/',
        fileId,
        'Ошибка удаления файла',
      )
    },
  },
})
