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
              v-model.number="form.contract_period"
              label="Срок действия (мес.)"
              type="number"
              min="0"
              variant="outlined"
              density="comfortable"
            />
          </v-col>

          <v-col cols="12" md="8">
            <v-autocomplete
              v-model="form.organization_id"
              :items="orgAutocompleteItems"
              label="Организация"
              item-title="display"
              item-value="id"
              v-model:search="searchOrganization"
              placeholder="Введите название организации"
              clearable
              hide-no-data
              variant="outlined"
              density="comfortable"
              :menu-props="{ maxHeight: '220px' }"
            >
              <template #no-data>
                <div class="pa-3 d-flex flex-column ga-2">
                  <span class="text-body-2 text-medium-emphasis">Организация не найдена в справочнике</span>
                  <v-btn
                    color="primary"
                    size="small"
                    variant="tonal"
                    prepend-icon="mdi-plus"
                    @click="openQuickOrganizationDialog"
                  >
                    Добавить новую организацию
                  </v-btn>
                </div>
              </template>
            </v-autocomplete>
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

          <v-col cols="12" md="4">
            <v-select
              v-model="form.pricelist_id"
              :items="pricelistOpt"
              label="Позиция прайса"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Выберите..."
              @update:model-value="onPricelistSelected"
            >
              <template #selection="{ item }">
                {{ item.raw.name }} — {{ formatPrice(item.raw.price) }}
              </template>
              <template #item="{ item, props: itemProps }">
                <v-list-item v-bind="itemProps">
                  <template #append>
                    <span class="text-caption text-medium-emphasis">{{ formatPrice(item.raw.price) }}</span>
                  </template>
                </v-list-item>
              </template>
            </v-select>
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model.number="form.price"
              label="Цена (₽)"
              type="number"
              step="0.01"
              min="0"
              variant="outlined"
              density="comfortable"
              :hint="isCustomPrice ? 'Спец. цена' : undefined"
              persistent-hint
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-select
              v-model="form.contract_status_id"
              :items="statusesOpt"
              label="Статус"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Не задан"
            >
              <template #selection="{ item }">
                <v-chip :color="item.raw.color" size="small" variant="tonal">{{ item.title }}</v-chip>
              </template>
              <template #item="{ item, props: itemProps }">
                <v-list-item v-bind="itemProps">
                  <template #prepend>
                    <v-chip :color="item.raw.color" size="x-small" class="mr-2" />
                  </template>
                </v-list-item>
              </template>
            </v-select>
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

        <v-divider class="my-3" />

        <h4 class="text-subtitle-1 mb-2">УПД (счета-фактуры, акты)</h4>

        <v-row dense>
          <v-col cols="12" md="9">
            <v-file-input
              v-model="newUpdFile"
              label="Выберите файл УПД"
              accept=".pdf,.doc,.docx,.xls,.xlsx,.jpg,.jpeg,.png"
              prepend-icon="mdi-file-document-outline"
              :loading="uploadingUpd"
              variant="outlined"
              density="comfortable"
              show-size
              clearable
            />
          </v-col>
          <v-col cols="12" md="3">
            <v-btn
              block
              color="warning"
              :disabled="!selectedUploadUpdFile || uploadingUpd"
              :loading="uploadingUpd"
              @click="handleUpdUpload"
            >
              {{ form.id ? 'Загрузить УПД' : 'Добавить УПД' }}
            </v-btn>
          </v-col>
        </v-row>

        <v-alert v-if="!form.id" type="info" variant="tonal" class="mb-2">
          УПД добавляются в очередь и загрузятся после сохранения договора.
        </v-alert>

        <v-list v-if="form.id" density="compact">
          <v-list-item v-for="file in updFiles" :key="file.id">
            <template #prepend><v-icon color="warning">mdi-file-document-outline</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-download" size="small" variant="text" @click="downloadFile(file.id)" />
              <v-btn icon="mdi-delete" size="small" variant="text" color="error" @click="removeUpdFile(file.id)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!updFiles.length">
            <v-list-item-title class="text-medium-emphasis">УПД не прикреплены</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-list v-else density="compact">
          <v-list-item v-for="(file, idx) in pendingUpdFiles" :key="`${file.name}-${idx}`">
            <template #prepend><v-icon color="warning">mdi-file-plus</v-icon></template>
            <v-list-item-title>{{ file.name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-close" size="small" variant="text" color="error" @click="removePendingUpdFile(idx)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!pendingUpdFiles.length">
            <v-list-item-title class="text-medium-emphasis">УПД в очередь не добавлены</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-divider class="my-3" />

        <h4 class="text-subtitle-1 mb-2">Дополнительные соглашения</h4>

        <v-btn
          v-if="form.id"
          size="small"
          color="primary"
          variant="tonal"
          prepend-icon="mdi-plus"
          class="mb-2"
          @click="openSupplementaryForm()"
        >
          Добавить соглашение
        </v-btn>

        <v-alert v-if="!form.id" type="info" variant="tonal" class="mb-2">
          Доп соглашения можно добавить после сохранения договора.
        </v-alert>

        <v-list v-if="form.id" density="compact">
          <v-list-item v-for="sa in supplementaryAgreements" :key="sa.id">
            <template #prepend><v-icon>mdi-file-document-outline</v-icon></template>
            <v-list-item-title>
              {{ sa.number || 'Без номера' }}
              <span v-if="sa.date_from"> от {{ formatDate(sa.date_from) }}</span>
            </v-list-item-title>
            <v-list-item-subtitle>
              {{ sa.description || '—' }}
              <span v-if="sa.price"> · {{ formatPrice(sa.price) }}</span>
            </v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-pencil" size="small" variant="text" @click="openSupplementaryForm(sa)" />
              <v-btn icon="mdi-delete" size="small" variant="text" color="error" @click="removeSupplementaryAgreement(sa.id)" />
            </template>
          </v-list-item>
          <v-list-item v-if="!supplementaryAgreements.length">
            <v-list-item-title class="text-medium-emphasis">Нет доп соглашений</v-list-item-title>
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

  <v-dialog v-model="quickOrgDialog" max-width="680">
    <v-card rounded="lg">
      <v-card-title>Новая организация</v-card-title>
      <v-card-text>
        <v-row dense>
          <v-col cols="12" md="8">
            <v-text-field
              v-model="quickOrg.short_name_with_opf"
              label="Краткое наименование"
              variant="outlined"
              density="comfortable"
              :error="Boolean(quickOrgErrors.short_name_with_opf)"
              :error-messages="quickOrgErrors.short_name_with_opf"
            />
          </v-col>
          <v-col cols="12" md="4">
            <v-text-field
              v-model="quickOrg.inn"
              label="ИНН"
              variant="outlined"
              density="comfortable"
              :error="Boolean(quickOrgErrors.inn)"
              :error-messages="quickOrgErrors.inn"
            />
          </v-col>
          <v-col cols="12">
            <v-text-field
              v-model="quickOrg.full_name_with_opf"
              label="Полное наименование"
              variant="outlined"
              density="comfortable"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field v-model="quickOrg.legal_address" label="Юридический адрес" variant="outlined" density="comfortable" />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field v-model="quickOrg.fact_address" label="Фактический адрес" variant="outlined" density="comfortable" />
          </v-col>
        </v-row>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="quickOrgDialog = false">Отмена</v-btn>
        <v-btn color="primary" :loading="quickOrgSaving" @click="saveQuickOrganization">Сохранить</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, reactive, ref, toRefs, watch } from 'vue'
import { ContractUtil } from '@/store/contracts'
import { OrganizationUtil } from '@/store/organizations'
import { SupplementaryAgreementUtil } from '@/store/supplementaryAgreements'
import { useToastStore } from '@/store/toast'

const props = defineProps([
  'modelValue',
  'contract',
  'organizationsOpt',
  'organizationsRaw',
  'respPersonsOpt',
  'validityTypesOpt',
  'statusesOpt',
  'pricelistOpt',
])
const emit = defineEmits(['update:modelValue', 'save', 'delete', 'organization-added', 'open-supplementary-form'])
const { modelValue, contract, organizationsOpt, organizationsRaw, respPersonsOpt, validityTypesOpt, statusesOpt, pricelistOpt } = toRefs(props)

const contractStore = ContractUtil()
const organizationStore = OrganizationUtil()
const saStore = SupplementaryAgreementUtil()
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
  contract_status_id: null,
  file_link: null,
  price: null,
  pricelist_id: null,
})

const newFile = ref(null)
const files = ref([])
const pendingFiles = ref([])
const uploading = ref(false)
const newUpdFile = ref(null)
const updFiles = ref([])
const pendingUpdFiles = ref([])
const uploadingUpd = ref(false)
const searchOrganization = ref('')
const quickOrgDialog = ref(false)
const quickOrgSaving = ref(false)
const supplementaryAgreements = ref([])
const supplementaryFormDialog = ref(false)
const selectedSupplementary = ref(null)
const isCustomPrice = ref(false)
const periodChanging = ref(false)

const quickOrg = reactive({
  short_name_with_opf: '',
  inn: '',
  full_name_with_opf: '',
  legal_address: '',
  fact_address: '',
  management_post: '',
  management_name: '',
  ogrn: '',
  opf_full: '',
  opf_short: '',
})

const quickOrgErrors = reactive({
  short_name_with_opf: '',
  inn: '',
})

const filteredOrganizations = computed(() => {
  const source = Array.isArray(organizationsRaw.value) && organizationsRaw.value.length
    ? organizationsRaw.value
    : organizationsOpt.value

  if (!source) return []

  const term = (searchOrganization.value || '').toLowerCase()
  if (!term) return source

  return source.filter((org) => {
    const shortName = (org.short_name_with_opf || '').toLowerCase()
    const fullName = (org.full_name_with_opf || '').toLowerCase()
    const inn = String(org.inn || '')
    return shortName.includes(term) || fullName.includes(term) || inn.includes(term)
  })
})

const orgAutocompleteItems = computed(() =>
  filteredOrganizations.value.map((org) => ({
    ...org,
    display: org.inn ? `${org.short_name_with_opf} (ИНН ${org.inn})` : org.short_name_with_opf,
  })),
)

const selectedUploadFile = computed(() => resolveSelectedFile(newFile.value))
const selectedUploadUpdFile = computed(() => resolveSelectedFile(newUpdFile.value))

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
    if (periodChanging.value) { periodChanging.value = false; return }
    form.contract_period = calculateContractPeriod(form.date_from, form.date_to)
  },
)

watch(
  () => form.contract_period,
  (period) => {
    if (!form.date_from || !period || period <= 0) return
    const start = new Date(form.date_from)
    const origDay = start.getDate()
    start.setMonth(start.getMonth() + period)
    // Корректировка: если день изменился из-за короткого месяца
    if (start.getDate() < origDay) {
      start.setDate(0) // последний день предыдущего месяца
    }
    periodChanging.value = true
    form.date_to = start.toISOString().split('T')[0] + 'T00:00:00'
  },
)

watch(
  () => contract.value,
  (newVal) => {
    isCustomPrice.value = false
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
        contract_status_id: null,
        file_link: null,
        price: null,
        pricelist_id: null,
      },
    )

    newFile.value = null
    pendingFiles.value = []

    if (newVal?.id) {
      loadFiles(newVal.id)
      loadUpdFiles(newVal.id)
      loadSupplementaryAgreements(newVal.id)
    } else {
      files.value = []
      supplementaryAgreements.value = []
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

async function loadUpdFiles(contractId) {
  try {
    updFiles.value = await contractStore.getContractFiles(contractId, 'upd')
  } catch (e) {
    console.error('Failed to load UPD files:', e)
  }
}

async function handleUpdUpload() {
  const file = selectedUploadUpdFile.value

  if (!file) {
    toast.push('Сначала выберите файл УПД', 'error')
    return
  }

  if (form.id) {
    uploadingUpd.value = true
    try {
      const uploadedFile = await contractStore.uploadFile(form.id, file, 'upd')
      updFiles.value.push(uploadedFile)
      toast.push('УПД загружен', 'success')
    } catch (e) {
      toast.push(e.message, 'error')
    } finally {
      uploadingUpd.value = false
      newUpdFile.value = null
    }
    return
  }

  pendingUpdFiles.value.push(file)
  newUpdFile.value = null
  toast.push('УПД добавлен в очередь', 'success')
}

function removePendingUpdFile(index) {
  pendingUpdFiles.value.splice(index, 1)
}

async function removeUpdFile(fileId) {
  try {
    await contractStore.deleteFile(fileId)
    updFiles.value = updFiles.value.filter((f) => f.id !== fileId)
    toast.push('УПД удален', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

async function loadSupplementaryAgreements(contractId) {
  try {
    supplementaryAgreements.value = await saStore.getByContract(contractId)
  } catch (e) {
    console.error('Failed to load supplementary agreements:', e)
  }
}

function openSupplementaryForm(agreement = null) {
  selectedSupplementary.value = agreement ? { ...agreement } : null
  emit('open-supplementary-form', { contractId: form.id, agreement: selectedSupplementary.value })
}

async function removeSupplementaryAgreement(id) {
  try {
    await saStore.delete(id)
    supplementaryAgreements.value = supplementaryAgreements.value.filter((sa) => sa.id !== id)
    toast.push('Соглашение удалено', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

function onPricelistSelected(pricelistId) {
  if (!pricelistId) {
    isCustomPrice.value = true
    return
  }
  const item = props.pricelistOpt?.find((p) => p.id === pricelistId)
  if (item) {
    form.price = parseFloat(item.price)
    isCustomPrice.value = false
  }
}

function formatDate(value) {
  if (!value) return ''
  return new Date(value).toLocaleDateString()
}

function formatPrice(value) {
  if (value === null || value === undefined) return '—'
  const num = parseFloat(value)
  return isNaN(num) ? '—' : num.toLocaleString('ru-RU', { style: 'currency', currency: 'RUB' })
}

function openQuickOrganizationDialog() {
  quickOrg.short_name_with_opf = searchOrganization.value || ''
  quickOrg.inn = ''
  quickOrg.full_name_with_opf = ''
  quickOrg.legal_address = ''
  quickOrg.fact_address = ''
  quickOrgErrors.short_name_with_opf = ''
  quickOrgErrors.inn = ''
  quickOrgDialog.value = true
}

async function saveQuickOrganization() {
  quickOrgErrors.short_name_with_opf = ''
  quickOrgErrors.inn = ''

  if (!quickOrg.short_name_with_opf?.trim()) {
    quickOrgErrors.short_name_with_opf = 'Введите краткое наименование'
  }

  const innValue = String(quickOrg.inn || '').trim()
  if (!/^\d{10,12}$/.test(innValue)) {
    quickOrgErrors.inn = 'ИНН должен содержать 10-12 цифр'
  }

  if (quickOrgErrors.short_name_with_opf || quickOrgErrors.inn) {
    return
  }

  quickOrgSaving.value = true
  try {
    const payload = {
      ...quickOrg,
      short_name_with_opf: quickOrg.short_name_with_opf.trim(),
      inn: Number(innValue),
    }

    const created = await organizationStore.addOrganization(payload)
    form.organization_id = created.id
    searchOrganization.value = created.short_name_with_opf || ''
    quickOrgDialog.value = false
    emit('organization-added', created)
    toast.push('Организация добавлена в справочник', 'success')
  } catch (error) {
    toast.push(error.message || 'Не удалось добавить организацию', 'error')
  } finally {
    quickOrgSaving.value = false
  }
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

  emit('save', {
    contract: payload,
    pendingFiles: [...pendingFiles.value],
    pendingUpdFiles: [...pendingUpdFiles.value],
  })
  emit('update:modelValue', false)
}

function deleteItem() {
  emit('delete', form.id)
  emit('update:modelValue', false)
}
</script>
