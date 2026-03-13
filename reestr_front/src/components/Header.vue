<template>
  <v-app-bar color="#114b5f" density="comfortable" elevation="2">
    <v-img src="@/assets/logo.png" max-height="34" max-width="96" contain class="mr-3 ml-2" />
    <v-toolbar-title class="font-weight-bold text-uppercase">Реестр договоров</v-toolbar-title>

    <v-spacer />

    <div v-if="isAuthorized" class="d-none d-md-flex ga-2 mr-4">
      <v-btn variant="text" :to="'/contracts'" prepend-icon="mdi-file-document-multiple">
        Договоры
      </v-btn>
      <v-btn variant="text" :to="'/organizations'" prepend-icon="mdi-domain">
        Организации
      </v-btn>
      <v-btn variant="text" :to="'/responsibleperson'" prepend-icon="mdi-account-tie">
        Ответственные
      </v-btn>
      <v-btn
        v-if="role === 'admin'"
        variant="text"
        :to="'/admin/users'"
        prepend-icon="mdi-account-cog"
      >
        Пользователи
      </v-btn>
    </div>

    <v-chip v-if="isAuthorized" size="small" color="#1a759f" class="mr-3">
      {{ authStore.user?.username || authStore.user?.login || 'Пользователь' }}
    </v-chip>

    <v-btn v-if="!isAuthorized" variant="tonal" :to="'/login'" prepend-icon="mdi-login">Войти</v-btn>
    <v-btn v-else variant="tonal" prepend-icon="mdi-logout" @click="handleLogout">Выйти</v-btn>
  </v-app-bar>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth'

const authStore = useAuthStore()
const router = useRouter()

const role = computed(() => authStore.user?.role || null)
const isAuthorized = computed(() => Boolean(authStore.token))

const handleLogout = async () => {
  try {
    await authStore.logout()
    await router.push('/login')
  } catch (error) {
    alert(error.message)
  }
}
</script>
