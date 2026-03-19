<template>
  <v-data-table
    :headers="headers"
    :items="statuses"
    :items-per-page="15"
    density="comfortable"
    fixed-header
    height="480"
  >
    <template #item.name="{ item }">
      <v-chip size="small" variant="tonal" :color="item.color">{{ item.name }}</v-chip>
    </template>

    <template #item.color="{ item }">
      <div class="d-flex align-center ga-2">
        <div
          :style="{ width: '20px', height: '20px', borderRadius: '4px', background: item.color }"
        />
        <span class="text-caption text-medium-emphasis">{{ item.color }}</span>
      </div>
    </template>

    <template #item.actions="{ item }">
      <v-btn icon="mdi-pencil" size="small" variant="text" @click="$emit('edit', item)" />
      <v-btn
        icon="mdi-delete"
        size="small"
        variant="text"
        color="error"
        @click="$emit('delete', item.id)"
      />
    </template>
  </v-data-table>
</template>

<script setup>
defineProps({ statuses: { type: Array, default: () => [] } })
defineEmits(['edit', 'delete'])

const headers = [
  { title: 'Название', key: 'name', sortable: true },
  { title: 'Цвет', key: 'color', sortable: false },
  { title: 'Действия', key: 'actions', sortable: false, align: 'end' },
]
</script>
