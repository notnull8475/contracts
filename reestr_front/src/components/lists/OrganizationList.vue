<template>
  <v-table>
    <thead>
      <tr>
        <th>ID</th>
        <th>Название организации</th>
        <th>ИНН</th>
        <th>Фактический адрес</th>
        <th>Юридический адрес</th>
        <th class="text-right">Действия</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="organization in organizations" :key="organization.id">
        <td>{{ organization.id }}</td>
        <td>{{ organization.name }}</td>
        <td>{{ organization.inn }}</td>
        <td>{{ organization.fact_address }}</td>
        <td>{{ organization.address }}</td>
        <td class="text-right">
          <v-btn icon @click="$emit('edit', organization)">
            <v-icon>mdi-pencil</v-icon>
          </v-btn>
          <v-btn icon color="error" @click="$emit('delete', organization.id)">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </td>
      </tr>
    </tbody>
  </v-table>
</template>
<script setup>
defineProps(['organizations'])
</script>
