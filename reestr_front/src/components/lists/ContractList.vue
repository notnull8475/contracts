<template>
  <v-data-table
    class="contract-table"
    :headers="headers"
    :items="contracts"
    :items-per-page="15"
    density="comfortable"
    fixed-header
    height="580"
  >
    <template #item.date_from="{ item }">
      {{ formatDate(item.date_from) || '---' }}
    </template>

    <template #item.date_to="{ item }">
      {{ formatDate(item.date_to) || '---' }}
    </template>

    <template #item.contract_period="{ item }">
      {{ item.contract_period || '---' }}
    </template>

    <template #item.organization_id="{ item }">
      {{ orgIdToName[item.organization_id] || '---' }}
    </template>

    <template #item.type_of_validity="{ item }">
      {{ validityTypeIdToName[item.type_of_validity] || '---' }}
    </template>

    <template #item.responsible_person_id="{ item }">
      {{ responsibleIdToName[item.responsible_person_id] || '---' }}
    </template>

    <template #item.address="{ item }">
      {{ item.address || '---' }}
    </template>

    <template #item.actual="{ item }">
      <v-chip :color="item.actual ? 'success' : 'default'" size="small" variant="tonal">
        {{ item.actual ? 'Да' : 'Нет' }}
      </v-chip>
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" variant="text" size="small" @click="$emit('edit', item)" />
    </template>
  </v-data-table>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps(['contracts', 'respPersonsOpt', 'organizationsOpt', 'validityTypesOpt'])
defineEmits(['edit'])

const headers = [
  { title: 'Номер', key: 'number', sortable: true },
  { title: 'Дата начала', key: 'date_from', sortable: true },
  { title: 'Дата окончания', key: 'date_to', sortable: true },
  { title: 'Срок (мес.)', key: 'contract_period', sortable: true },
  { title: 'Организация', key: 'organization_id', sortable: true },
  { title: 'Тип', key: 'type_of_validity', sortable: true },
  { title: 'Ответственный', key: 'responsible_person_id', sortable: true },
  { title: 'Адрес', key: 'address', sortable: false },
  { title: 'Актуален', key: 'actual', sortable: true },
  { title: 'Действия', key: 'actions', sortable: false, align: 'end' },
]

const orgIdToName = computed(() => {
  const map = {}
  props.organizationsOpt?.forEach((i) => {
    map[i.id] = i.short_name_with_opf
  })
  return map
})

const responsibleIdToName = computed(() => {
  const map = {}
  props.respPersonsOpt?.forEach((i) => {
    map[i.id] = i.lastname
  })
  return map
})

const validityTypeIdToName = computed(() => {
  const map = {}
  props.validityTypesOpt?.forEach((i) => {
    map[i.id] = i.name
  })
  return map
})

function formatDate(value) {
  if (!value) return ''
  return new Date(value).toLocaleDateString()
}
</script>

<style scoped>
.contract-table :deep(.v-data-table-header__content) {
  font-weight: 600;
}
</style>
