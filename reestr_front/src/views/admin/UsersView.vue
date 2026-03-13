<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5 d-flex flex-wrap justify-space-between align-center ga-3">
      <div>
        <h1 class="text-h5 font-weight-bold mb-1">Пользователи системы</h1>
        <p class="text-body-2 text-medium-emphasis">Управление доступом и ролями</p>
      </div>
      <v-btn color="primary" prepend-icon="mdi-account-plus" @click="openForm()">
        Добавить пользователя
      </v-btn>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-card-text>
      <div class="d-flex flex-wrap ga-2 mb-3">
        <v-chip color="primary" variant="tonal">Всего: {{ users.length }}</v-chip>
        <v-chip color="info" variant="tonal">Администраторы: {{ adminCount }}</v-chip>
      </div>

      <v-text-field
        v-model="search"
        label="Поиск по имени или логину"
        prepend-inner-icon="mdi-magnify"
        variant="outlined"
        density="comfortable"
        clearable
        hide-details
      />
    </v-card-text>

    <v-divider />

    <v-progress-linear v-if="loading" indeterminate color="primary" />

    <v-card-text v-else>
      <v-alert v-if="!filteredUsers.length" type="info" variant="tonal">
        По текущему фильтру пользователи не найдены.
      </v-alert>

      <user-list v-else :users="filteredUsers" @edit="openForm" @delete="deleteUser" />
    </v-card-text>
  </v-card>

  <user-form v-model="dialog" :user="selectedUser" @save="saveUser" />
</template>

<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import UserList from '@/components/admin/UserList.vue'
import UserForm from '@/components/admin/UserForm.vue'
import { UserUtil } from '@/store/users.js'

const search = ref('')
const dialog = ref(false)
const selectedUser = ref(null)
const users = ref([])
const loading = ref(false)

const userStore = UserUtil()
const route = useRoute()
const router = useRouter()

const adminCount = computed(() => users.value.filter((u) => u.role === 'admin').length)

const filteredUsers = computed(() => {
  const rows = Array.isArray(users.value) ? users.value : []
  if (!search.value) return rows
  const term = search.value.toLowerCase()
  return rows.filter((s) => {
    const login = (s.login || '').toLowerCase()
    const username = (s.username || '').toLowerCase()
    return login.includes(term) || username.includes(term)
  })
})

onMounted(fetchPage)

watch(
  () => route.query.new,
  async (value) => {
    if (value === '1') {
      openForm()
      const nextQuery = { ...route.query }
      delete nextQuery.new
      await router.replace({ query: nextQuery })
    }
  },
  { immediate: true },
)

async function fetchPage() {
  loading.value = true
  try {
    users.value = await userStore.getAllUsers()
  } catch (e) {
    console.error('Не удалось получить список пользователей', e)
  } finally {
    loading.value = false
  }
}

function openForm(user = null) {
  selectedUser.value = user ? { ...user } : null
  dialog.value = true
}

async function saveUser(user) {
  try {
    if (user.id) {
      await userStore.updateUser(user)
    } else {
      await userStore.addUser(user)
    }
    await fetchPage()
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteUser(id) {
  try {
    await userStore.deleteUser(id)
    users.value = users.value.filter((s) => s.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
  }
}
</script>
