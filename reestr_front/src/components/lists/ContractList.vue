<template>
  <v-table>
    <thead>
    <tr>
<!--      <th>ID</th>-->
      <th>Номер договора</th>
      <th>Дата договора</th>
      <th>Организация</th>
      <th>Тип валидности</th>
      <th>Ответственный</th>
      <th>Адрес</th>
      <th class="text-right">Действия</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="contract in contracts" :key="contract.id">
<!--      <td>{{ contract.id }}</td>-->
      <td>{{ contract.number }}</td>
      <td>{{ formatDate(contract.date) }}</td>
      <td>{{ orgidToName[contract.organization_id] || '---' }}</td>
      <td>{{ tvIdToName[contract.type_of_validity] }}</td>
      <td>{{ rpIdToName[contract.responsible_person_id] || '---' }}</td>
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
import { computed, defineProps } from 'vue'


const props = defineProps(['contracts','respPersonsOpt','organizationsOpt','validityTypesOpt'])
const orgidToName = computed(() => {
  const map = {}
  props.organizationsOpt?.forEach(i => {
    map[i.id] = i.name
  })
  return map
})
const rpIdToName = computed(() => {
  const map = {}
  props.respPersonsOpt?.forEach(i => {
    map[i.id] = i.lastname
  })
  return map
})
const tvIdToName = computed(() => {
  const map = {}
  props.validityTypesOpt?.forEach(i => {
    map[i.id] = i.name
  })
  return map
})

function formatDate(date) {
  return date ? new Date(date).toLocaleDateString() : ''
}
</script>