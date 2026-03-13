<template>
  <v-app-bar color="#114b5f" density="comfortable" elevation="2">
    <v-app-bar-nav-icon v-if="isAuthorized" class="d-md-none" @click="drawer = !drawer" />
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

  <v-navigation-drawer v-model="drawer" temporary location="left" class="d-md-none">
    <v-list-item
      :title="authStore.user?.username || authStore.user?.login || 'Пользователь'"
      :subtitle="role || 'Гость'"
      prepend-icon="mdi-account-circle"
      class="mb-2"
    />

    <v-divider />

    <v-list nav density="comfortable">
      <v-list-subheader>Навигация</v-list-subheader>
      <v-list-item
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        :prepend-icon="item.icon"
        :title="item.title"
        @click="drawer = false"
      />

      <v-list-subheader>Быстрые действия</v-list-subheader>
      <v-list-item
        v-for="action in quickActions"
        :key="action.key"
        :to="action.to"
        :prepend-icon="action.icon"
        :title="action.title"
        @click="drawer = false"
      />
    </v-list>
  </v-navigation-drawer>
</template>

<script setup>
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/store/auth'

const authStore = useAuthStore()
const router = useRouter()
const drawer = ref(false)

const role = computed(() => authStore.user?.role || null)
const isAuthorized = computed(() => Boolean(authStore.token))
const navItems = computed(() => {
  const items = [
    { title: 'Договоры', to: '/contracts', icon: 'mdi-file-document-multiple' },
    { title: 'Организации', to: '/organizations', icon: 'mdi-domain' },
    { title: 'Ответственные', to: '/responsibleperson', icon: 'mdi-account-tie' },
  ]

  if (role.value === 'admin') {
    items.push({ title: 'Пользователи', to: '/admin/users', icon: 'mdi-account-cog' })
  }

  return items
})

const quickActions = computed(() => {
  const actions = [
    { key: 'new-contract', title: 'Новый договор', to: '/contracts?new=1', icon: 'mdi-plus-circle-outline' },
    { key: 'new-org', title: 'Новая организация', to: '/organizations?new=1', icon: 'mdi-domain-plus' },
    {
      key: 'new-person',
      title: 'Новый ответственный',
      to: '/responsibleperson?new=1',
      icon: 'mdi-account-plus',
    },
  ]

  if (role.value === 'admin') {
    actions.push({ key: 'new-user', title: 'Новый пользователь', to: '/admin/users?new=1', icon: 'mdi-account-multiple-plus' })
  }

  return actions
})

const handleLogout = async () => {
  try {
    drawer.value = false
    await authStore.logout()
    await router.push('/login')
  } catch (error) {
    alert(error.message)
  }
}
</script>
