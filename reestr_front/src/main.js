// Vuetify
import 'vuetify/styles'
// Без шрифта Material Design Icons все иконки mdi-* отрисовываются пустыми.
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import './index.css' // Tailwind CSS
import apiClient from './axios'
import { useAuthStore } from '@/store/auth.js'

const app = createApp(App)
const pinia = createPinia()
const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: { mdi },
  },
})
app.use(router)
app.use(pinia)
app.use(vuetify)

// Добавляем Axios в глобальные свойства приложения
app.config.globalProperties.$axios = apiClient

// Инициализируем состояние авторизации
const authStore = useAuthStore()
authStore.initialize()

app.mount('#app')
