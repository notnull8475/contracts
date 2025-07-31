import {defineStore} from 'pinia'
import axios from '@/axios.js'

export const UserUtil = defineStore('users', {
    state: () => ({}),
    actions: {

        async addUser(newUser) {
            try {
                const response = await axios.post('/api/v1/admin/users/add', newUser)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error add user')
            }
        },
        async updateUser(newUser) {
            try {
                const response = await axios.post('/api/v1/admin/users/update', newUser)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error update user')
            }
        },
        async deleteUser(userId) {
            try {
                const response = await axios.delete(`/api/v1/users/del/${userId}`)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error deleting user')
            }
        },
        async getAllUsers() {
            try {
                const response = await axios.get('/api/v1/admin/users/get/list')
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error get users')
            }
        },

        async getRoles() {
            try {
                const response = await axios.get('/api/v1/roles/get')
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error deleting role')
            }
        },
        async getRolesByUserId(id) {
            try {
                const response = await axios.get('/api/v1/roles/get' + id)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error deleting role')
            }
        },
        async addRole(newRole) {
            try {
                const response = await axios.post('/api/v1/role/add', newRole)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error add role')
            }
        },
        async updateRole(roledto) {
            try {
                const response = await axios.post('/api/v1/role/update', newRole)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error update role')
            }
        },
        async deleteRole(roleId) {
            try {
                const response = await axios.delete('/api/v1/delete/role/' + roleId)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error deleting role')
            }
        },

        async getUserNameById(user_id) {
            try {
                const response = await axios.get(`/api/v1/get/user/name/by/id/${user_id}`)
                console.log("RESPONSE USERNAME: " + response.data)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error get username')
            }
        },
        async getLink() {
            try {
                const response = await axios.get(`/api/v1/user/generate/link`)
                console.log("LINK: " + response.data)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error get user link')
            }
        },
        async getLinkById(id) {
            try {
                const response = await axios.get(`/api/v1/user/generate/link/` + id)
                console.log("LINK: " + response.data)
                return response.data
            } catch (error) {
                console.log(error)
                throw new Error('Error get user link')
            }
        },

    }
})