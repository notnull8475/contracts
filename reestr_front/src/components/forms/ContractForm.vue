<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card>
      <v-card-title>{{ form.id ? 'Редактировать договор' : 'Добавить договор' }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Номер договора -->
        <v-text-field v-model="form.number" label="Номер договора" />

        <!-- Дата начала договора -->
        <v-text-field v-model="formattedDateFrom" label="Дата начала договора" type="date" />

        <!-- Дата окончания договора -->
        <v-text-field v-model="formattedDateTo" label="Дата окончания договора" type="date" />

        <!-- Срок действия договора (месяцы) -->
        <v-text-field v-model.number="form.contract_period" label="Срок действия (месяцев)" type="number" min="0" />

        <!-- Выбор организации -->
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
          :menu-props="{ maxHeight: '200px' }"
        />

        <!-- Тип договора -->
        <v-select
          v-model="form.type_of_validity"
          :items="validityTypesOpt"
          label="Тип договора"
          item-title="name"
          item-value="id"
        />

        <!-- Ответственное лицо -->
        <v-select
          v-model="form.responsible_person_id"
          label="Ответственное лицо"
          :items="respPersonsOpt"
          item-title="lastname"
          item-value="id"
        />

        <!-- Адрес -->
        <v-text-field v-model="form.address" label="Адрес" />

        <!-- Дополнительное соглашение -->
        <v-text-field v-model="form.additional_agreement" label="Дополнительное соглашение" />

        <!-- Комментарий -->
        <v-textarea v-model="form.comment" label="Комментарий" />

        <!-- Активен -->
        <v-checkbox v-model="form.actual" label="Актуален" />

        <!-- Файлы -->
        <div v-if="contract?.id">
          <v-divider class="my-4" />
          <h4 class="mb-2">Файлы</h4>
          
          <v-file-input
            v-model="newFile"
            label="Выберите файл"
            accept=".pdf,.doc,.docx,.xls,.xlsx,.jpg,.jpeg,.png"
            prepend-icon="mdi-paperclip"
            :loading="uploading"
          />

          <v-btn
            class="mt-2"
            color="primary"
            variant="flat"
            :disabled="!newFile || uploading"
            :loading="uploading"
            @click="handleFileUpload"
          >
            Загрузить файл
          </v-btn>

          <v-list density="compact" class="mt-2">
            <v-list-item
              v-for="file in files"
              :key="file.id"
            >
              <template v-slot:prepend>
                <v-icon>mdi-file-document</v-icon>
              </template>
              <v-list-item-title>{{ file.original_name }}</v-list-item-title>
              <v-list-item-subtitle>
                {{ formatFileSize(file.file_size) }}
              </v-list-item-subtitle>
              <template v-slot:append>
                <v-btn
                  icon="mdi-download"
                  size="small"
                  variant="text"
                  @click="downloadFile(file.id)"
                />
                <v-btn
                  icon="mdi-delete"
                  size="small"
                  variant="text"
                  color="error"
                  @click="removeFile(file.id)"
                />
              </template>
            </v-list-item>
          </v-list>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-btn v-if="contract?.id" color="error" @click="deleteItem">Удалить</v-btn>
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

const props = defineProps([
  'modelValue',
  'contract',
  'organizationsOpt',
  'respPersonsOpt',
  'validityTypesOpt',
])
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
const uploading = ref(false)

const searchOrganization = ref('')

const filteredOrganizations = computed(() => {
  if (!organizationsOpt.value) return []
  return organizationsOpt.value.filter((org) =>
    org.short_name_with_opf.toLowerCase().includes(searchOrganization.value?.toLowerCase() || '')
  )
})

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
      }
    )
    if (newVal?.id) {
      loadFiles(newVal.id)
    } else {
      files.value = []
    }
  },
  { immediate: true }
)

async function loadFiles(contractId) {
  try {
    files.value = await contractStore.getContractFiles(contractId)
  } catch (e) {
    console.error('Failed to load files:', e)
  }
}

async function handleFileUpload(fileInput = newFile.value) {
  let file = null

  if (fileInput instanceof File) {
    file = fileInput
  } else if (Array.isArray(fileInput)) {
    file = fileInput[0] || null
  } else if (fileInput?.target?.files?.length) {
    file = fileInput.target.files[0]
  } else if (fileInput?.length && fileInput[0] instanceof File) {
    file = fileInput[0]
  }

  if (!file || !form.id) return

  uploading.value = true
  try {
    const uploadedFile = await contractStore.uploadFile(form.id, file)
    files.value.push(uploadedFile)
    newFile.value = null
    toast.push('Файл загружен', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  } finally {
    uploading.value = false
  }
}

function downloadFile(fileId) {
  contractStore.downloadFile(fileId)
}

async function removeFile(fileId) {
  try {
    await contractStore.deleteFile(fileId)
    files.value = files.value.filter((f) => f.id !== fileId)
    toast.push('Файл удалён', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

function formatFileSize(bytes) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
}

function save() {
  // Normalize date strings as ISO datetime strings ending with T00:00:00
  if (form.date_from && typeof form.date_from === 'string' && !form.date_from.endsWith('T00:00:00')) {
    form.date_from = `${form.date_from.split('T')[0]}T00:00:00`
  }
  if (form.date_to && typeof form.date_to === 'string' && !form.date_to.endsWith('T00:00:00')) {
    form.date_to = `${form.date_to.split('T')[0]}T00:00:00`
  }

  emit('save', { ...form })
  emit('update:modelValue', false)
}

function deleteItem() {
  emit('delete', form.id)
  emit('update:modelValue', false)
}
</script>
