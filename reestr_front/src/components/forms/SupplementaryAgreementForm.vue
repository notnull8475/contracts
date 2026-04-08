<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="520"
  >
    <v-card rounded="lg">
      <v-card-title class="px-6 pt-5 pb-1">
        {{ form.id ? 'Редактировать соглашение' : 'Новое доп соглашение' }}
      </v-card-title>

      <v-card-text class="px-6 pb-2">
        <v-text-field
          v-model="form.number"
          label="Номер соглашения"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <v-text-field
          v-model="formattedDate"
          label="Дата"
          type="date"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <v-textarea
          v-model="form.description"
          label="Описание"
          variant="outlined"
          density="comfortable"
          rows="2"
          auto-grow
          class="mb-2"
        />

        <v-text-field
          v-model.number="form.price"
          label="Цена (₽)"
          type="number"
          step="0.01"
          min="0"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <div v-if="form.id">
          <v-file-input
            v-model="selectedFile"
            label="Прикрепить файл"
            variant="outlined"
            density="comfortable"
            prepend-icon="mdi-paperclip"
            show-size
            accept="*/*"
            :loading="uploading"
            class="mb-2"
          />
          <v-btn
            v-if="selectedFile"
            size="small"
            color="primary"
            variant="tonal"
            :loading="uploading"
            @click="uploadFile"
            class="mb-2"
          >
            Загрузить файл
          </v-btn>

          <v-list density="compact" class="mt-1">
            <v-list-item v-for="file in files" :key="file.id">
              <template #prepend><v-icon size="small">mdi-file-document</v-icon></template>
              <v-list-item-title class="text-body-2">{{ file.original_name }}</v-list-item-title>
              <v-list-item-subtitle class="text-caption">{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
              <template #append>
                <v-btn icon="mdi-download" size="x-small" variant="text" @click="downloadFile(file.id)" />
                <v-btn icon="mdi-delete" size="x-small" variant="text" color="error" @click="deleteFile(file.id)" />
              </template>
            </v-list-item>
            <v-list-item v-for="(f, idx) in pendingFiles" :key="`p-${idx}`">
              <template #prepend><v-icon size="small" color="warning">mdi-file-plus</v-icon></template>
              <v-list-item-title class="text-body-2">{{ f.name }}</v-list-item-title>
              <v-list-item-subtitle class="text-caption">{{ formatFileSize(f.size) }}</v-list-item-subtitle>
              <template #append>
                <v-btn icon="mdi-close" size="x-small" variant="text" color="error" @click="removePending(idx)" />
              </template>
            </v-list-item>
            <v-list-item v-if="!files.length && !pendingFiles.length">
              <v-list-item-title class="text-caption text-medium-emphasis">Файлы не прикреплены</v-list-item-title>
            </v-list-item>
          </v-list>
        </div>
        <div v-else>
          <v-file-input
            v-model="selectedFile"
            label="Прикрепить файл (сохраните соглашение для загрузки)"
            variant="outlined"
            density="comfortable"
            prepend-icon="mdi-paperclip"
            show-size
            accept="*/*"
            hint="Сначала сохраните соглашение, затем загрузите файл"
            persistent-hint
          />
        </div>
      </v-card-text>

      <v-divider />

      <v-card-actions class="px-6 py-3">
        <v-spacer />
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
        <v-btn color="primary" @click="save">Сохранить</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, reactive, ref, watch } from 'vue'

const props = defineProps(['modelValue', 'agreement'])
const emit = defineEmits(['update:modelValue', 'save', 'files-loaded'])

const form = reactive({
  id: null,
  number: '',
  date_from: null,
  description: '',
  price: null,
})

const selectedFile = ref(null)
const files = ref([])
const pendingFiles = ref([])
const uploading = ref(false)

watch(
  () => props.agreement,
  async (val) => {
    Object.assign(form, val || { id: null, number: '', date_from: null, description: '', price: null })
    selectedFile.value = null
    pendingFiles.value = []
    files.value = []
    if (form.id) {
      emit('files-loaded', { saId: form.id, files })
    }
  },
  { immediate: true },
)

const formattedDate = computed({
  get() {
    return form.date_from ? new Date(form.date_from).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_from = value ? new Date(value + 'T00:00:00').toISOString() : null
  },
})

function save() {
  emit('save', {
    ...form,
    pendingFiles: [...pendingFiles.value],
  })
  emit('update:modelValue', false)
}

function uploadFile() {
  const file = selectedFile.value
  if (!file) return
  pendingFiles.value.push(file)
  selectedFile.value = null
}

function removePending(idx) {
  pendingFiles.value.splice(idx, 1)
}

function downloadFile(fileId) {
  window.open(`/api/v1/contracts/files/download/${fileId}`, '_blank')
}

async function deleteFile(fileId) {
  try {
    const res = await fetch(`/api/v1/contracts/files/delete/${fileId}`, {
      method: 'DELETE',
      headers: { Authorization: `Bearer ${localStorage.getItem('token')}` },
    })
    if (res.ok) {
      files.value = files.value.filter((f) => f.id !== fileId)
    }
  } catch (e) {
    console.error('Failed to delete file', e)
  }
}

function formatFileSize(bytes) {
  if (!bytes) return '0 B'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
</script>
