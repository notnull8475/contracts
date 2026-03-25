import { defineStore } from 'pinia'
import axios from '@/axios.js'

const BASE = '/api/v1/admin/pricelist'

export const PricelistUtil = defineStore('pricelist', {
  actions: {
    async getList() {
      const res = await axios.get(`${BASE}/list`)
      return res.data
    },
    async add(dto) {
      const res = await axios.post(`${BASE}/add`, dto)
      return res.data
    },
    async update(dto) {
      const res = await axios.post(`${BASE}/update`, dto)
      return res.data
    },
    async delete(id) {
      const res = await axios.delete(`${BASE}/del/${id}`)
      return res.data
    },
  },
})
