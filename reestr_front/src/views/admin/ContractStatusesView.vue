<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5 d-flex flex-wrap justify-space-between align-center ga-3">
      <div>
        <h1 class="text-h5 font-weight-bold mb-1">Статусы договоров</h1>
        <p class="text-body-2 text-medium-emphasis">
          Управление справочником статусов и их цветами
        </p>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openForm()">Добавить статус</v-btn>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-card-text>
      <v-chip color="primary" variant="tonal">Всего статусов: {{ statuses.length }}</v-chip>
    </v-card-text>

    <v-divider />
    <v-progress-linear v-if="loading" indeterminate color="primary" />

    <v-card-text v-else>
      <v-alert v-if="!statuses.length" type="info" variant="tonal">
        Статусы не добавлены.
      </v-alert>
      <contract-status-list
        v-else
        :statuses="statuses"
        @edit="openForm"
        @delete="deleteStatus"
      />
    </v-card-text>
  </v-card>

  <contract-status-form
    v-model="dialog"
    :status="selectedStatus"
    @save="saveStatus"
  />
</template>

<script setup>
import { onMounted, ref } from 'vue'
import ContractStatusList from '@/components/admin/ContractStatusList.vue'
import ContractStatusForm from '@/components/admin/ContractStatusForm.vue'
import { ContractStatusUtil } from '@/store/contractStatuses.js'
import { useToastStore } from '@/store/toast.js'

const statusStore = ContractStatusUtil()
const toast = useToastStore()

const statuses = ref([])
const loading = ref(false)
const dialog = ref(false)
const selectedStatus = ref(null)

onMounted(fetchStatuses)

async function fetchStatuses() {
  loading.value = true
  try {
    statuses.value = await statusStore.getStatuses()
  } catch (e) {
    toast.push('Не удалось загрузить статусы', 'error')
  } finally {
    loading.value = false
  }
}

function openForm(status = null) {
  selectedStatus.value = status ? { ...status } : null
  dialog.value = true
}

async function saveStatus(dto) {
  try {
    if (dto.id) {
      const updated = await statusStore.updateStatus(dto)
      const idx = statuses.value.findIndex((s) => s.id === dto.id)
      if (idx !== -1) statuses.value[idx] = updated
      toast.push('Статус обновлён', 'success')
    } else {
      const created = await statusStore.addStatus({ name: dto.name, color: dto.color })
      statuses.value.push(created)
      toast.push('Статус добавлен', 'success')
    }
  } catch (e) {
    toast.push(e.message || 'Ошибка сохранения', 'error')
  }
}

async function deleteStatus(id) {
  try {
    await statusStore.deleteStatus(id)
    statuses.value = statuses.value.filter((s) => s.id !== id)
    toast.push('Статус удалён. В договорах поле статуса обнулено.', 'success')
  } catch (e) {
    toast.push(e.message || 'Ошибка удаления', 'error')
  }
}
</script>
