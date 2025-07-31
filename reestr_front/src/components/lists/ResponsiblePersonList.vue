<template>
  <v-table>
    <thead>
    <tr>
      <th>ID</th>
      <th>Имя</th>
      <th>Фамилия</th>
      <th>Отчество</th>
      <th v-if="role === 'admin'">Пользователь</th>
      <th class="text-right">Действия</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="person in responsiblePersons" :key="person.id">
      <td>{{ person.id }}</td>
      <td>{{ person.firstname }}</td>
      <td>{{ person.lastname }}</td>
      <td>{{ person.name }}</td>
      <td v-if="role === 'admin'">{{ userIdToName[person.user_id] || '—' }}</td>
      <td class="text-right">
        <v-btn icon @click="$emit('edit', person)">
          <v-icon>mdi-pencil</v-icon>
        </v-btn>
        <v-btn icon color="error" @click="$emit('delete', person.id)">
          <v-icon>mdi-delete</v-icon>
        </v-btn>
      </td>
    </tr>
    </tbody>
  </v-table>
</template>
<script setup>
import { computed } from 'vue'
import { useAuthStore } from '@/store/auth.js'
const authStore = useAuthStore();
const role = computed(() => {
  if (authStore.token) {
    return authStore.user.role;
  } else return null;
});
const props = defineProps({
  responsiblePersons: Array,
  userOptions: Array // 👈 передаём список пользователей [{ id, username }]
})
const userIdToName = computed(() => {
  const map = {}
  props.userOptions?.forEach(user => {
    map[user.id] = user.username
  })
  return map
})
</script>