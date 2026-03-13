<template>
  <v-data-table
    :headers="headers"
    :items="users"
    :items-per-page="15"
    density="comfortable"
    fixed-header
    height="560"
  >
    <template #item.role="{ item }">
      <v-chip size="small" :color="item.role === 'admin' ? 'primary' : 'default'" variant="tonal">
        {{ item.role }}
      </v-chip>
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" size="small" variant="text" @click="$emit('edit', item)" />
      <v-btn icon="mdi-delete" size="small" variant="text" color="error" @click="$emit('delete', item.id)" />
    </template>
  </v-data-table>
</template>

<script setup>
defineProps(['users'])
defineEmits(['edit', 'delete'])

const headers = [
  { title: 'Имя', key: 'username', sortable: true },
  { title: 'Логин', key: 'login', sortable: true },
  { title: 'Роль', key: 'role', sortable: true },
  { title: 'Действия', key: 'actions', sortable: false, align: 'end' },
]
</script>
