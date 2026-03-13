<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="1100" scrollable>
    <v-card rounded="lg">
      <v-card-title>{{ form.id ? 'Редактировать договор' : 'Добавить договор' }}</v-card-title>

      <v-card-text>
        <v-row dense>
          <v-col cols="12" md="4">
            <v-text-field v-model="form.number" label="Номер договора" variant="outlined" density="comfortable" />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model="formattedDateFrom"
              label="Дата начала договора"
              type="date"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model="formattedDateTo"
              label="Дата окончания договора"
              type="date"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              :model-value="form.contract_period ?? ''"
              label="Срок действия (месяцев)"
              variant="outlined"
              density="comfortable"
              readonly
            />
          </v-col>

          <v-col cols="12" md="8">
            <v-autocomplete
              v-model="form.organization_id"
              :items="filteredOrganizations"
              label="Организация"
              item-title="short_name_with_opf"
              item-value="id"
              v-model:search="searchOrganization"
              placeholder="Введите название организации"
              clearable
              hide-no-data
              variant="outlined"
              density="comfortable"
              :menu-props="{ maxHeight: '220px' }"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-select
              v-model="form.type_of_validity"
              :items="validityTypesOpt"
              label="Тип договора"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-select
              v-model="form.responsible_person_id"
              label="Ответственное лицо"
              :items="respPersonsOpt"
              item-title="lastname"
              item-value="id"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-text-field v-model="form.address" label="Адрес" variant="outlined" density="comfortable" />
          </v-col>

          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.additional_agreement"
              label="Дополнительное соглашение"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12">
            <v-textarea v-model="form.comment" label="Комментарий" variant="outlined" density="comfortable" rows="2" auto-grow />
          </v-col>

          <v-col cols="12">
            <v-checkbox v-model="form.actual" label="Актуален" hide-details />
          </v-col>
        </v-row>

        <v-divider class="my-3" />

        <h4 class="text-subtitle-1 mb-2">Файлы</h4>

        <v-row dense>
          <v-col cols="12" md="9">
            <v-file-input
              v-model="newFile"
              label="Выберите файл"
              accept=".pdf,.doc,.docx,.xls,.xlsx,.jpg,.jpeg,.png"
              prepend-icon="mdi-paperclip"
              :loading="uploading"
              variant="outlined"
              density="comfortable"
              show-size
              clearable
            />
          </v-col>
          <v-col cols="12" md="3">
            <v-btn
              block
              color="primary"
              :disabled="!selectedUploadFile || uploading"
              :loading="uploading"
              @click="handleFileUpload"
            >
              {{ form.id ? 'Загрузить файл' : 'Добавить файл' }}
            </v-btn>
          </v-col>
        </v-row>

        <v-alert v-if="!form.id" type="info" variant="tonal" class="mb-2">
          Для нового договора файлы добавляются в очередь и загрузятся после сохранения договора.
        </v-alert>

        <v-list v-if="form.id" density="compact">
          <v-list-item v-for="file in files" :key="file.id">
            <template #prepend><v-icon>mdi-file-document</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-download" size="small" variant="text" @click="downloadFile(file.id)" />
              <v-btn icon="mdi-delete" size="small" variant="text" color="error" @click="removeFile(file.id)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!files.length">
            <v-list-item-title class="text-medium-emphasis">Файлы не прикреплены</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-list v-else density="compact">
          <v-list-item v-for="(file, idx) in pendingFiles" :key="`${file.name}-${idx}`">
            <template #prepend><v-icon>mdi-file-plus</v-icon></template>
            <v-list-item-title>{{ file.name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-close" size="small" variant="text" color="error" @click="removePendingFile(idx)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!pendingFiles.length">
            <v-list-item-title class="text-medium-emphasis">Файлы в очередь не добавлены</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-card-text>

      <v-card-actions>
        <v-btn v-if="form.id" color="error" @click="deleteItem">Удалить</v-btn>
        <v-spacer />
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, reactive, ref, toRefs, watch } from 'vue'
import { ContractUtil } from '@/store/contracts'
import { useToastStore } from '@/store/toast'

const props = defineProps(['modelValue', 'contract', 'organizationsOpt', 'respPersonsOpt', 'validityTypesOpt'])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])
const { modelValue, contract, organizationsOpt, respPersonsOpt, validityTypesOpt } = toRefs(props)

const contractStore = ContractUtil()
const toast = useToastStore()

const form = reactive({
  id: null,
  number: '',
  date_from: null,
  date_to: null,
  contract_period: null,
  organization_id: null,
  type_of_validity: null,
  responsible_person_id: null,
  address: '',
  additional_agreement: '',
  comment: '',
  actual: false,
})

const newFile = ref(null)
const files = ref([])
const pendingFiles = ref([])
const uploading = ref(false)
const searchOrganization = ref('')

const filteredOrganizations = computed(() => {
  if (!organizationsOpt.value) return []
  return organizationsOpt.value.filter((org) =>
    org.short_name_with_opf.toLowerCase().includes(searchOrganization.value?.toLowerCase() || ''),
  )
})

const selectedUploadFile = computed(() => resolveSelectedFile(newFile.value))

const formattedDateFrom = computed({
  get() {
    return form.date_from ? new Date(form.date_from).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_from = value ? `${value}T00:00:00` : null
  },
})

const formattedDateTo = computed({
  get() {
    return form.date_to ? new Date(form.date_to).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_to = value ? `${value}T00:00:00` : null
  },
})

watch(
  () => [form.date_from, form.date_to],
  () => {
    form.contract_period = calculateContractPeriod(form.date_from, form.date_to)
  },
)

watch(
  () => contract.value,
  (newVal) => {
    Object.assign(
      form,
      newVal || {
        id: null,
        number: '',
        date_from: null,
        date_to: null,
        contract_period: null,
        organization_id: null,
        type_of_validity: null,
        responsible_person_id: null,
        address: '',
        additional_agreement: '',
        comment: '',
        actual: false,
      },
    )

    newFile.value = null
    pendingFiles.value = []

    if (newVal?.id) {
      loadFiles(newVal.id)
    } else {
      files.value = []
    }
  },
  { immediate: true },
)

async function loadFiles(contractId) {
  try {
    files.value = await contractStore.getContractFiles(contractId)
  } catch (e) {
    console.error('Failed to load files:', e)
  }
}

async function handleFileUpload() {
  const file = selectedUploadFile.value

  if (!file) {
    toast.push('Сначала выберите файл', 'error')
    return
  }

  if (form.id) {
    uploading.value = true
    try {
      const uploadedFile = await contractStore.uploadFile(form.id, file)
      files.value.push(uploadedFile)
      toast.push('Файл загружен', 'success')
    } catch (e) {
      toast.push(e.message, 'error')
    } finally {
      uploading.value = false
      newFile.value = null
    }
    return
  }

  pendingFiles.value.push(file)
  newFile.value = null
  toast.push('Файл добавлен в очередь', 'success')
}

function resolveSelectedFile(fileInput) {
  if (!fileInput) return null
  if (fileInput instanceof File) return fileInput
  if (Array.isArray(fileInput)) return fileInput[0] || null
  if (fileInput?.target?.files?.length) return fileInput.target.files[0]
  if (fileInput?.length && fileInput[0] instanceof File) return fileInput[0]
  return null
}

function removePendingFile(index) {
  pendingFiles.value.splice(index, 1)
}

function calculateContractPeriod(fromDate, toDate) {
  if (!fromDate || !toDate) return null

  const start = new Date(fromDate)
  const end = new Date(toDate)

  if (Number.isNaN(start.getTime()) || Number.isNaN(end.getTime()) || end < start) {
    return null
  }

  let months = (end.getFullYear() - start.getFullYear()) * 12 + (end.getMonth() - start.getMonth())
  if (end.getDate() < start.getDate()) {
    months -= 1
  }

  return Math.max(months, 0)
}

function downloadFile(fileId) {
  contractStore.downloadFile(fileId)
}

async function removeFile(fileId) {
  try {
    await contractStore.deleteFile(fileId)
    files.value = files.value.filter((f) => f.id !== fileId)
    toast.push('Файл удален', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

function formatFileSize(bytes) {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

function save() {
  const payload = {
    ...form,
    contract_period: calculateContractPeriod(form.date_from, form.date_to),
  }

  emit('save', { contract: payload, pendingFiles: [...pendingFiles.value] })
  emit('update:modelValue', false)
}

function deleteItem() {
  emit('delete', form.id)
  emit('update:modelValue', false)
}
</script>
