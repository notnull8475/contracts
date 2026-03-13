<template>
  <v-data-table
    class="organization-table"
    :headers="headers"
    :items="organizations"
    :items-per-page="15"
    density="comfortable"
    fixed-header
    height="580"
  >
    <template #item.full_name_with_opf="{ item }">
      <div class="cell-wrap">{{ item.full_name_with_opf || '-' }}</div>
    </template>

    <template #item.fact_address="{ item }">
      <div class="cell-wrap">{{ item.fact_address || '-' }}</div>
    </template>

    <template #item.legal_address="{ item }">
      <div class="cell-wrap">{{ item.legal_address || '-' }}</div>
    </template>

    <template #item.management="{ item }">
      <div v-if="item.management_name" class="cell-wrap">
        <div class="font-weight-medium">{{ item.management_name }}</div>
        <div class="text-caption text-medium-emphasis">{{ item.management_post }}</div>
      </div>
      <span v-else>-</span>
    </template>

    <template #item.opf="{ item }">
      {{ item.opf_short || item.opf_full || '-' }}
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" size="small" variant="text" @click="$emit('edit', item)" />
    </template>
  </v-data-table>
</template>

<script setup>
defineProps({
  organizations: {
    type: Array,
    default: () => [],
  },
})

defineEmits(['edit'])

const headers = [
  { title: 'ID', key: 'id', sortable: true },
  { title: 'Краткое наименование', key: 'short_name_with_opf', sortable: true },
  { title: 'Полное наименование', key: 'full_name_with_opf', sortable: true },
  { title: 'ИНН', key: 'inn', sortable: true },
  { title: 'ОГРН', key: 'ogrn', sortable: true },
  { title: 'Факт. адрес', key: 'fact_address', sortable: false },
  { title: 'Юр. адрес', key: 'legal_address', sortable: false },
  { title: 'Руководитель', key: 'management', sortable: false },
  { title: 'ОПФ', key: 'opf', sortable: true },
  { title: 'Действия', key: 'actions', sortable: false, align: 'end' },
]
</script>

<style scoped>
.cell-wrap {
  max-width: 260px;
  white-space: normal;
  word-break: break-word;
}

.organization-table :deep(.v-data-table-header__content) {
  font-weight: 600;
}
</style>
