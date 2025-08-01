<script setup async>
import { computed, onMounted, ref } from 'vue'
import UserList from '@/components/admin/UserList.vue'
import UserForm from '@/components/admin/UserForm.vue'
import { UserUtil } from '@/store/users.js'

/* ═══ реактивные переменные ══════════════════════════════════════ */
const search = ref('')
const dialog = ref(false)
const selectedUser = ref(null)
const users = ref([]) // ← всегда стартуем с []

/* ═══ утилита доступа к API / хранилищу ══════════════════════════ */
const specUtil = UserUtil()

/* ═══ загрузка данных  ═══════════════════════════════════════════ */
const fetchPage = async () => {
  try {
    users.value = await specUtil.getAllUsers()
    console.log(users.value)
    return users.value
  } catch (e) {
    console.error('Не удалось получить список пользователей', e)
  }
}

onMounted(fetchPage)
/* ═══ фильтр по поиску  ═════════════════════════════════════════ */
const filteredUsers = computed(() =>
  (Array.isArray(users.value) ? users.value : []).filter(
    (s) => s.username && s.username.toLowerCase().includes(search.value.toLowerCase()),
  ),
)

/* ═══ формы add / edit / delete  ════════════════════════════════ */
function openForm(spec = null) {
  selectedUser.value = spec ? { ...spec } : null
  dialog.value = true
}

async function saveUser(spec) {
  try {
    if (spec.id) {
      await specUtil.updateUser(spec)
      const idx = users.value.findIndex((s) => s.id === spec.id)
      if (idx !== -1) users.value[idx] = spec
    } else {
      const created = await specUtil.addUser(spec)
      users.value.push(created ?? { ...spec, id: Date.now() })
    }
    await fetchPage()
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteUser(id) {
  try {
    await specUtil.deleteUser(id)
    users.value = users.value.filter((s) => s.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
  }
}
</script>

<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-semibold">Пользователи</h2>
      <v-btn color="primary" @click="openForm()">Добавить пользователя</v-btn>
    </div>

    <v-text-field
      v-model="search"
      placeholder="Поиск по пользователям"
      append-inner-icon="mdi-magnify"
    />

    <user-list :users="filteredUsers" @edit="openForm" @delete="deleteUser" />

    <user-form v-model="dialog" :user="selectedUser" @save="saveUser" />
  </div>
</template>
