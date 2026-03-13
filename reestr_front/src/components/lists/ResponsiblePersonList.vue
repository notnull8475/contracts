<template>
  <v-data-table
    class="person-table"
    :headers="headers"
    :items="responsiblePersons"
    :items-per-page="15"
    density="comfortable"
    fixed-header
    height="560"
  >
    <template #item.user_id="{ item }">
      {{ userIdToName[item.user_id] || '—' }}
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" variant="text" size="small" @click="$emit('edit', item)" />
    </template>
  </v-data-table>
</template>

<script setup>
import { computed } from 'vue'
import { useAuthStore } from '@/store/auth.js'

const authStore = useAuthStore()

const role = computed(() => {
  if (authStore.token) return authStore.user.role
  return null
})

const props = defineProps({
  responsiblePersons: {
    type: Array,
    default: () => [],
  },
  userOptions: {
    type: Array,
    default: () => [],
  },
})

defineEmits(['edit'])

const headers = computed(() => {
  const base = [
    { title: 'ID', key: 'id', sortable: true },
    { title: 'Имя', key: 'firstname', sortable: true },
    { title: 'Фамилия', key: 'lastname', sortable: true },
    { title: 'Отчество', key: 'name', sortable: true },
  ]

  if (role.value === 'admin') {
    base.push({ title: 'Пользователь', key: 'user_id', sortable: true })
  }

  base.push({ title: 'Действия', key: 'actions', sortable: false, align: 'end' })
  return base
})

const userIdToName = computed(() => {
  const map = {}
  props.userOptions.forEach((user) => {
    map[user.id] = user.username
  })
  return map
})
</script>

<style scoped>
.person-table :deep(.v-data-table-header__content) {
  font-weight: 600;
}
</style>
