import {defineStore} from 'pinia'

export const useRequtil = defineStore('requtil',{
  state:()=>({}),
  actions:()=>({
    async makePost(url:string,data:any,errorMessage:string){}
  })
})