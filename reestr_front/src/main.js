// Vuetify
import 'vuetify/styles'
import {createVuetify} from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'


import {createApp} from 'vue'
import App from './App.vue'
import router from './router'
import {createPinia} from 'pinia'
import './index.css' // Tailwind CSS
import apiClient from './axios'
import {useAuthStore} from '@/store/auth.js' // Импортируем настроенный Axios

const app = createApp(App)
const pinia = createPinia()
const vuetify = createVuetify({
    components,
    directives,
})
app.use(router)
app.use(pinia)
app.use(vuetify)

// Добавляем Axios в глобальные свойства приложения
app.config.globalProperties.$axios = apiClient

// Инициализируем состояние авторизации
const authStore = useAuthStore(pinia)
authStore.initialize()

app.mount('#app')
