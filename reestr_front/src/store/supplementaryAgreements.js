import { defineStore } from 'pinia'
import axios from '@/axios.js'

const BASE = '/api/v1/contracts'

export const SupplementaryAgreementUtil = defineStore('supplementaryAgreements', {
  actions: {
    async getByContract(contractId) {
      const res = await axios.get(`${BASE}/supplementary-agreements/${contractId}`)
      return res.data
    },
    async countByContract(contractId) {
      const res = await axios.get(`${BASE}/supplementary-agreements/count/${contractId}`)
      return res.data.count
    },
    async add(dto) {
      const res = await axios.post(`${BASE}/supplementary-agreements/add`, dto)
      return res.data
    },
    async update(dto) {
      const res = await axios.post(`${BASE}/supplementary-agreements/update`, dto)
      return res.data
    },
    async delete(id) {
      const res = await axios.delete(`${BASE}/supplementary-agreements/del/${id}`)
      return res.data
    },
  },
})
