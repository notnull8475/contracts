<template>
  <v-table>
    <thead>
    <tr>
      <th>ID</th>
      <th>Номер договора</th>
      <th>Дата договора</th>
      <th>ID организации</th>
      <th>Тип валидности</th>
      <th>ID ответственного лица</th>
      <th>Адрес</th>
      <th class="text-right">Действия</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="contract in contracts" :key="contract.id">
      <td>{{ contract.id }}</td>
      <td>{{ contract.number }}</td>
      <td>{{ formatDate(contract.date) }}</td>
      <td>{{ contract.organization_id }}</td>
      <td>{{ contract.type_of_validity }}</td>
      <td>{{ contract.responsible_person_id }}</td>
      <td>{{ contract.address }}</td>
      <td class="text-right">
        <v-btn icon @click="$emit('edit', contract)">
          <v-icon>mdi-pencil</v-icon>
        </v-btn>
        <v-btn icon color="error" @click="$emit('delete', contract.id)">
          <v-icon>mdi-delete</v-icon>
        </v-btn>
      </td>
    </tr>
    </tbody>
  </v-table>
</template>
<script setup>
import { defineProps } from 'vue'

const props = defineProps(['contracts'])

function formatDate(date) {
  return date ? new Date(date).toLocaleDateString() : ''
}
</script>