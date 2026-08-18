<template>
  <component
    :is="serverMode ? VDataTableServer : VDataTable"
    v-bind="tableProps"
    class="contract-table"
    :headers="headers"
    :items="contracts"
    density="comfortable"
    fixed-header
    @update:options="$emit('update:options', $event)"
  >
    <template #item.row_number="{ index }">
      {{ rowOffset + index + 1 }}
    </template>

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

    <template #item.price="{ item }">
      <span v-if="item.price !== null && item.price !== undefined">
        {{ formatPrice(item.price) }}
      </span>
      <span v-else class="text-medium-emphasis text-caption">—</span>
    </template>

    <template #item.sa_count="{ item }">
      <v-chip
        size="small"
        :color="(saCounts?.[item.id] || 0) > 0 ? 'success' : 'default'"
        variant="tonal"
      >
        {{ (saCounts?.[item.id] || 0) > 0 ? saCounts[item.id] : '0' }}
      </v-chip>
    </template>

    <template #item.files="{ item }">
      <v-chip
        size="small"
        :color="(fileCounts?.[item.id] || 0) > 0 ? 'info' : 'default'"
        variant="tonal"
        class="cursor-pointer"
        @click="$emit('files-click', item)"
      >
        {{ (fileCounts?.[item.id] || 0) > 0 ? `Файлов: ${fileCounts[item.id]}` : 'Нет файлов' }}
      </v-chip>
    </template>

    <template #item.upd_count="{ item }">
      <v-chip
        size="small"
        :color="(updCounts?.[item.id] || 0) > 0 ? 'warning' : 'default'"
        variant="tonal"
        class="cursor-pointer"
        @click="$emit('upd-click', item)"
      >
        {{ (updCounts?.[item.id] || 0) > 0 ? updCounts[item.id] : '0' }}
      </v-chip>
    </template>

    <template #item.contract_status_id="{ item }">
      <v-chip
        v-if="statusMap[item.contract_status_id]"
        size="small"
        variant="tonal"
        :color="statusMap[item.contract_status_id].color"
      >
        {{ statusMap[item.contract_status_id].name }}
      </v-chip>
      <span v-else class="text-medium-emphasis text-caption">—</span>
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" variant="text" size="small" @click="$emit('edit', item)" />
    </template>
  </component>
</template>

<script setup>
import { computed } from 'vue'
import { VDataTable, VDataTableServer } from 'vuetify/components'

const props = defineProps([
  'contracts',
  'respPersonsOpt',
  'organizationsOpt',
  'validityTypesOpt',
  'fileCounts',
  'updCounts',
  'statusesOpt',
  'saCounts',
  // Server mode props
  'serverMode',
  'serverItemsLength',
  'serverPage',
  'serverPerPage',
  'serverSortBy',
  'serverSortOrder',
  'itemsPerPageOptions',
])
defineEmits(['edit', 'files-click', 'upd-click', 'update:options'])

const headers = [
  { title: '#', key: 'row_number', sortable: false, width: '50px' },
  { title: 'Номер', key: 'number', sortable: true },
  { title: 'Дата начала', key: 'date_from', sortable: true },
  { title: 'Дата окончания', key: 'date_to', sortable: true },
  { title: 'Срок (мес.)', key: 'contract_period', sortable: true },
  { title: 'Организация', key: 'organization_id', sortable: true },
  { title: 'Тип', key: 'type_of_validity', sortable: true },
  { title: 'Ответственный', key: 'responsible_person_id', sortable: true },
  { title: 'Цена', key: 'price', sortable: true },
  { title: 'Доп. согл.', key: 'sa_count', sortable: false },
  { title: 'Файлы', key: 'files', sortable: false },
  { title: 'УПД', key: 'upd_count', sortable: false },
  { title: 'Статус', key: 'contract_status_id', sortable: true },
  { title: 'Действия', key: 'actions', sortable: false, align: 'end' },
]

/**
 * В серверном режиме нужен именно v-data-table-server: обычная v-data-table
 * пагинирует и сортирует уже полученную страницу на клиенте, из-за чего футер
 * показывает размер страницы вместо общего количества, а сортировка работает
 * только внутри страницы.
 */
const tableProps = computed(() => {
  if (!props.serverMode) {
    return {
      itemsPerPage: 15,
      itemsPerPageOptions: [15, 30, 50],
    }
  }

  return {
    itemsLength: props.serverItemsLength ?? 0,
    itemsPerPage: props.serverPerPage ?? 50,
    itemsPerPageOptions: props.itemsPerPageOptions ?? [50, 100, 200],
    page: props.serverPage ?? 1,
    sortBy: props.serverSortBy ?? [],
  }
})

/** Сквозная нумерация строк: на второй странице счёт продолжается, а не начинается заново. */
const rowOffset = computed(() => {
  if (!props.serverMode) return 0
  const page = props.serverPage ?? 1
  const perPage = props.serverPerPage ?? 0
  // perPage <= 0 — режим «все записи», страница одна.
  if (perPage <= 0) return 0
  return (page - 1) * perPage
})

const statusMap = computed(() => {
  const map = {}
  props.statusesOpt?.forEach((s) => {
    map[s.id] = s
  })
  return map
})

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

function formatPrice(value) {
  if (value === null || value === undefined) return '—'
  const num = parseFloat(value)
  return isNaN(num) ? '—' : num.toLocaleString('ru-RU', { style: 'currency', currency: 'RUB' })
}
</script>

<style scoped>
.contract-table :deep(.v-data-table-header__content) {
  font-weight: 600;
  white-space: nowrap;
  overflow: visible;
}
</style>
