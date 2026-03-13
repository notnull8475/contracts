<template>
  <div class="login-shell">
    <v-card class="mx-auto login-card" rounded="xl" elevation="6">
      <v-card-item class="pb-1">
        <v-card-title class="text-h5 font-weight-bold">Вход в систему</v-card-title>
        <v-card-subtitle>Реестр договоров</v-card-subtitle>
      </v-card-item>

      <v-card-text>
        <v-text-field
          v-model="credentials.login"
          label="Логин"
          prepend-inner-icon="mdi-account-outline"
          variant="outlined"
          density="comfortable"
          autocomplete="username"
        />

        <v-text-field
          v-model="credentials.password"
          label="Пароль"
          type="password"
          prepend-inner-icon="mdi-lock-outline"
          variant="outlined"
          density="comfortable"
          autocomplete="current-password"
          @keydown.enter="handleLogin"
        />

        <v-btn
          class="mt-2"
          color="primary"
          size="large"
          block
          :loading="loading"
          @click="handleLogin"
        >
          Войти
        </v-btn>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup>
import { onMounted, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth.js'

const router = useRouter()
const authStore = useAuthStore()

const loading = ref(false)
const credentials = reactive({
  login: '',
  password: '',
})

onMounted(() => {
  if (authStore.token) {
    router.push('/contracts')
  }
})

async function handleLogin() {
  try {
    loading.value = true
    await authStore.login(credentials)
    await router.push('/contracts')
  } catch (error) {
    alert(error.message)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-shell {
  min-height: calc(100vh - 64px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.login-card {
  width: 100%;
  max-width: 420px;
  border: 1px solid rgba(17, 75, 95, 0.08);
}
</style>
