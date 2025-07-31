import { defineStore } from 'pinia';
import { useRequtil } from '@/store/util.js';


const vtRequest = '/api/v1/types/validity';

export const ValidityTypesUtil = defineStore('validityTypes', {
  state: () => ({ validityTypes: [] }),
  actions: {

    async addValidityTypes(newOrg) {
      const requtil = useRequtil();
      return requtil.makePostRequest(`${vtRequest}/add`, newOrg, 'Ошибка добавления типа договора');
    },


    async updateValidityTypes(OrgData) {
      const requtil = useRequtil();
      return requtil.makePostRequest(`${vtRequest}/update`, OrgData, 'Ошибка обновления типа договора');
    },

    async delValidityTypes(OrgID) {
      const requtil = useRequtil();
      return requtil.makeDeleteRequest(`${vtRequest}/del/`, OrgID, 'Ошибка удаления типа договора');
    },

    async getValidityType(OrgID) {
      const requtil = useRequtil();
      return requtil.makeGetRequest(`${vtRequest}/get/`, OrgID, 'Ошибка получения типа договора');
    },

    async getValidityTypes() {
      const requtil = useRequtil();
      return requtil.makeGetRequest(`${vtRequest}/list`, '', 'Ошибка получения списка организаций');
    },
  },
});