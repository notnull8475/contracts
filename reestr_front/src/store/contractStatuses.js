import { defineStore } from 'pinia'
import axios from '@/axios.js'

const BASE = '/api/v1/admin/contract-statuses'

export const ContractStatusUtil = defineStore('contractStatuses', {
  state: () => ({ statuses: [] }),
  actions: {
    async getStatuses() {
      const res = await axios.get(`${BASE}/list`)
      return res.data
    },
    async addStatus(dto) {
      const res = await axios.post(`${BASE}/add`, dto)
      return res.data
    },
    async updateStatus(dto) {
      const res = await axios.post(`${BASE}/update`, dto)
      return res.data
    },
    async deleteStatus(id) {
      const res = await axios.delete(`${BASE}/del/${id}`)
      return res.data
    },
  },
})
