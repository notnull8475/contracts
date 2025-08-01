<template>
  <v-table>
    <thead>
      <tr>
        <th>Имя</th>
        <th>Логин</th>
        <th>Роли</th>
        <th class="text-right">Действия</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="user in users" :key="user.id">
        <td>{{ user.username }}</td>
        <td>{{ user.login }}</td>
        <td>{{ user.role }}</td>
        <td class="text-right">
          <v-btn icon @click="$emit('edit', user)">
            <v-icon>mdi-pencil</v-icon>
          </v-btn>
          <v-btn icon color="error" @click="$emit('delete', user.id)">
            <v-icon icon="mdi-delete" />
          </v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>

<script setup>
defineProps(['users'])
</script>
