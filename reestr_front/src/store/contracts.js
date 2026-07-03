// contracts.js
import { defineStore } from 'pinia'
import { useRequtil } from '@/store/util.js'
import axios from '@/axios.js'

const contractRequest = '/api/v1/contracts'

function getFilenameFromHeaders(res, fallback) {
  const disposition = res.headers['content-disposition'] || ''
  const star = /filename\*=(?:UTF-8'')?([^;]+)/i.exec(disposition)
  if (star) {
    try {
      return decodeURIComponent(star[1].replace(/"/g, '').trim())
    } catch {
      /* ignore */
    }
  }
  const plain = /filename="?([^";]+)"?/i.exec(disposition)
  if (plain) return plain[1].trim()
  return fallback
}

function triggerBlobDownload(res, fallbackName) {
  const filename = getFilenameFromHeaders(res, fallbackName)
  const url = window.URL.createObjectURL(res.data)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  a.remove()
  window.URL.revokeObjectURL(url)
}

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
    async getPaginatedContracts(params = {}) {
      const res = await axios.get(`${contractRequest}/paginated`, { params })
      return res.data
    },
    async getBatchStats() {
      const res = await axios.get(`${contractRequest}/stats`)
      return res.data
    },
    async uploadFile(contractId, file, fileType = 'contract', saId = null) {
      const formData = new FormData()
      formData.append('file', file)
      let url = `${contractRequest}/files/${contractId}?file_type=${fileType}`
      if (saId) url += `&supplementary_agreement_id=${saId}`
      const res = await axios.post(url, formData)
      return res.data
    },
    async getContractFiles(contractId, fileType = 'contract') {
      const res = await axios.get(`${contractRequest}/files/${contractId}?file_type=${fileType}`)
      return res.data
    },
    async getSaFiles(saId) {
      const res = await axios.get(`${contractRequest}/supplementary-agreements/files/${saId}`)
      return res.data
    },
    async downloadFile(fileId) {
      const res = await axios.get(`${contractRequest}/files/download/${fileId}`, {
        responseType: 'blob',
      })
      triggerBlobDownload(res, `file_${fileId}`)
    },
    async deleteFile(fileId) {
      const requtil = useRequtil()
      return requtil.makeDeleteRequest(
        contractRequest + '/files/delete/',
        fileId,
        'Ошибка удаления файла',
      )
    },
    async getContractHistory(contractId) {
      const res = await axios.get(`${contractRequest}/history/${contractId}`)
      return res.data
    },
  },
})
