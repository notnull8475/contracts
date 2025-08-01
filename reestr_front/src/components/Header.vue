<template>
  <v-app-bar app color="#0F7CB7FF" dark>
    <v-img src="@/assets/logo.png" max-height="40" max-width="120" contain class="ml-2"></v-img>
    <v-toolbar-title>РЕЕСТР ДОГОВОРОВ</v-toolbar-title>

    <v-spacer></v-spacer>

    <v-toolbar-items>
      <v-btn v-if="role !== null" variant="text" to="/contracts">Договора</v-btn>
      <v-btn v-if="role !== null" variant="text" to="/responsibleperson">Ответственные</v-btn>
      <v-btn v-if="role !== null" variant="text" to="/organizations">Организации</v-btn>

      <!--      <v-btn v-if="role === 'admin'" variant="text" to="/admin">Админка</v-btn>-->
      <v-btn v-if="role === 'admin'" variant="text" to="/admin/users">Пользователи</v-btn>
      <v-btn v-if="role === null" variant="text" to="/login">Войти</v-btn>
      <v-btn v-if="role !== null" variant="text" @click="handleLogout">Выйти</v-btn>
    </v-toolbar-items>
  </v-app-bar>
</template>

<script setup>
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth'

const authStore = useAuthStore()
const router = useRouter()

// Реактивное вычисляемое свойство роли
const role = computed(() => {
  if (authStore.token) {
    return authStore.user.role
  } else return null
})

// Выход пользователя
const handleLogout = async () => {
  try {
    await authStore.logout()
    await router.push('/login')
  } catch (error) {
    alert(error.message)
  }
}
</script>

<style scoped>
.color__green {
  color: #0f7cb7;
}
</style>
