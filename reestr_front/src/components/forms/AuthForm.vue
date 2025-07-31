<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-100">
    <form
        @submit.prevent="handleLogin"
        class="bg-white p-6 rounded-lg shadow-md w-full max-w-sm space-y-4 sm:border sm:border-gray-200 sm:p-8 pa-10"
    >
      <h2 class="text-2xl font-bold text-center text-gray-800">Вход</h2>

      <!-- Поле для логина -->
      <div class="pb-4">
        <label for="login" class="block text-sm font-medium text-gray-700 mb-1">Логин</label>
        <input
            id="login"
            v-model="credentials.login"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
        />
      </div>

      <!-- Поле для пароля -->
      <div>
        <label for="password" class="block text-sm font-medium text-gray-700 mb-1">Пароль</label>
        <input
            id="password"
            v-model="credentials.password"
            type="password"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            required
        />
      </div>

      <!-- Кнопка входа -->
      <button
          type="submit"
          class="w-full py-2 text-sm font-medium  bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      >
        Войти
      </button>
    </form>
  </div>
</template>

<script>
import {useAuthStore} from '@/store/auth.js'

export default {
  data() {
    return {
      credentials: {
        login: '',
        password: ''
      }
    }
  },

  created() {
    const authStore = useAuthStore()
    if (authStore.token) {
      // Если пользователь уже авторизован, перенаправляем его
      this.$router.push('/about')
    }
  },

  methods: {
    async handleLogin() {
      try {
        const authStore = useAuthStore()
        await authStore.login(this.credentials)
        this.$router.push('/about') // Переходим на страницу отчетов
      } catch (error) {
        alert(error.message)
      }
    }
  }
}
</script>