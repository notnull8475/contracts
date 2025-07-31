import { defineStore } from 'pinia'
import { useRequtil } from '@/store/util.js'

let personsRequest = '/api/v1/responsible_person'
export const ResponsiblePersonUtil = defineStore('responsiblePersons', {
  state: () => ({}),
  actions: {
    async addResponsiblePerson(newPerson) {
      const requtil = useRequtil()
      return requtil.makePostRequest(
        personsRequest + '/add',
        newPerson,
        'Ошибка добавления ответственного лица',
      )
    },
    async updateResponsiblePerson(personData) {
      const requtil = useRequtil()
      return requtil.makePostRequest(
        personsRequest + '/update',
        personData,
        'Ошибка обновления ответственного лица',
      )
    },
    async delResponsiblePerson(personID) {
      const requtil = useRequtil()
      return requtil.makeDeleteRequest(
        personsRequest + '/del/',
        personID,
        'Ошибка удаления ответственного лица',
      )
    },

    async getResponsiblePerson(personID) {
      const requtil = useRequtil()
      return requtil.makeGetRequest(
        personsRequest + '/get/',
        personID,
        'Ошибка получения ответственного лица',
      )
    },
    async getResponsiblePersons() {
      const requtil = useRequtil()
      return requtil.makeGetRequest(
        personsRequest + '/list',
        '',
        'Ошибка получения списка ответственных лиц',
      )
    },
  },
})
