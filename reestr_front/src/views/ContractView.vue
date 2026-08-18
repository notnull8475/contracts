<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5">
      <div class="d-flex flex-wrap justify-space-between align-center ga-3">
        <div>
          <h1 class="text-h5 font-weight-bold mb-1">Реестр договоров</h1>
          <p class="text-body-2 text-medium-emphasis">
            Рабочий список договоров · Найдено: {{ totalContracts }}
          </p>
        </div>

        <div class="d-flex flex-wrap ga-2">
          <v-btn
            color="secondary"
            variant="tonal"
            prepend-icon="mdi-shape-outline"
            @click="openTypeForm()"
          >
            Типы
          </v-btn>
          <v-btn
            color="secondary"
            variant="tonal"
            prepend-icon="mdi-tag-multiple"
            @click="statusesDialog = true"
          >
            Статусы
          </v-btn>
          <v-btn
            color="secondary"
            variant="tonal"
            prepend-icon="mdi-currency-rub"
            @click="pricelistDialog = true"
          >
            Прайс
          </v-btn>
          <v-btn color="primary" prepend-icon="mdi-plus" @click="openForm()">Новый договор</v-btn>
        </div>
      </div>

      <div class="d-flex flex-wrap ga-2 mt-3">
        <v-text-field
          v-model="search"
          label="Поиск (номер, организация, адрес, ответственный...)"
          prepend-inner-icon="mdi-magnify"
          variant="outlined"
          density="comfortable"
          hide-details
          clearable
          @update:model-value="onSearchDebounced"
        />
        <v-select
          v-model="yearFilter"
          :items="yearFilterOptions"
          label="Год"
          density="comfortable"
          hide-details
          variant="outlined"
          @update:model-value="onFilterChange"
        />
        <v-select
          v-model="statusFilter"
          :items="statusFilterOptions"
          label="Статус"
          density="comfortable"
          hide-details
          variant="outlined"
          clearable
          @update:model-value="onFilterChange"
        >
          <template #selection="{ item }">
            <v-chip v-if="item.raw.color" :color="item.raw.color" size="small" variant="tonal">{{
              item.title
            }}</v-chip>
            <span v-else>{{ item.title }}</span>
          </template>
        </v-select>
      </div>

      <div class="d-flex flex-wrap ga-2 mt-4">
        <v-chip color="primary" variant="tonal">Всего: {{ totalContracts }}</v-chip>
        <v-chip color="warning" variant="tonal"
          >Загружено файлов: {{ Object.keys(fileCounts).length > 0 ? '✓' : '...' }}</v-chip
        >
      </div>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-divider />
    <v-progress-linear v-if="loading" indeterminate color="primary" />

    <contract-list
      v-if="contracts.length || !loading"
      :contracts="contracts"
      :respPersonsOpt="respPersonsOpt"
      :organizationsOpt="organizationsOpt"
      :validityTypesOpt="validityTypesOpt"
      :statusesOpt="contractStatuses"
      :fileCounts="fileCounts"
      :updCounts="updCounts"
      :saCounts="saCounts"
      :serverMode="true"
      :serverItemsLength="totalContracts"
      :serverPage="currentPage"
      :serverPerPage="perPage"
      :serverSortBy="serverSortByComputed"
      :serverSortOrder="sortOrder"
      :itemsPerPageOptions="itemsPerPageOptions"
      @edit="openForm"
      @files-click="openContractFiles"
      @upd-click="openContractUpdFiles"
      @update:options="onTableOptionsChange"
    />
  </v-card>

  <contract-form
    v-model="dialog"
    :contract="selectedContract"
    :respPersonsOpt="respPersonsOpt"
    :organizationsOpt="organizationsOpt"
    :organizationsRaw="organizations"
    :validityTypesOpt="validityTypesOpt"
    :statusesOpt="contractStatuses"
    :pricelistOpt="pricelist"
    @save="saveContract"
    @delete="deleteContract"
    @organization-added="handleOrganizationAdded"
    @open-supplementary-form="handleOpenSupplementaryForm"
  />

  <validity-types-form
    ref="validityTypesFormRef"
    v-model="VTdialog"
    :validityTypesOpt="validityTypesOpt"
    @save="saveType"
    @delete="deleteType"
  />

  <contract-statuses-form
    v-model="statusesDialog"
    :statusesOpt="contractStatuses"
    @save="saveStatus"
    @delete="deleteStatus"
  />

  <pricelist-form
    v-model="pricelistDialog"
    :pricelistOpt="pricelist"
    @save="savePricelistItem"
    @delete="deletePricelistItem"
  />

  <supplementary-agreement-form
    v-model="saDialog"
    :agreement="selectedSupplementary"
    :files="selectedSaFiles"
    @save="saveSupplementaryAgreement"
    @file-deleted="onSaFileDeleted"
  />

  <v-dialog v-model="filesDialog" max-width="720">
    <v-card rounded="lg">
      <v-card-title>Файлы договора</v-card-title>
      <v-card-text>
        <v-progress-linear v-if="filesDialogLoading" indeterminate color="primary" class="mb-3" />

        <v-alert v-else-if="!selectedContractFiles.length" type="info" variant="tonal">
          У договора нет прикрепленных файлов.
        </v-alert>

        <v-list v-else density="comfortable">
          <v-list-item
            v-for="file in selectedContractFiles"
            :key="file.id"
            @click="downloadFile(file.id)"
          >
            <template #prepend><v-icon>mdi-file-document</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-download"
                size="small"
                variant="text"
                @click.stop="downloadFile(file.id)"
              />
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="filesDialog = false">Закрыть</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <v-dialog v-model="updDialog" max-width="720">
    <v-card rounded="lg">
      <v-card-title>УПД договора</v-card-title>
      <v-card-text>
        <v-progress-linear v-if="updDialogLoading" indeterminate color="primary" class="mb-3" />

        <v-alert v-else-if="!selectedContractUpdFiles.length" type="info" variant="tonal">
          У договора нет прикрепленных УПД.
        </v-alert>

        <v-list v-else density="comfortable">
          <v-list-item
            v-for="file in selectedContractUpdFiles"
            :key="file.id"
            @click="downloadFile(file.id)"
          >
            <template #prepend><v-icon>mdi-file-document-outline</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-download"
                size="small"
                variant="text"
                @click.stop="downloadFile(file.id)"
              />
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="updDialog = false">Закрыть</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ContractList from '@/components/lists/ContractList.vue'
import ContractForm from '@/components/forms/ContractForm.vue'
import { ContractUtil } from '@/store/contracts.js'
import { OrganizationUtil } from '@/store/organizations.js'
import { ResponsiblePersonUtil } from '@/store/responsiblePersons.js'
import ValidityTypesForm from '@/components/forms/ValidityTypesForm.vue'
import { ValidityTypesUtil } from '@/store/validityTypes.js'
import { ContractStatusUtil } from '@/store/contractStatuses.js'
import ContractStatusesForm from '@/components/forms/ContractStatusesForm.vue'
import { PricelistUtil } from '@/store/pricelist.js'
import PricelistForm from '@/components/forms/PricelistForm.vue'
import { SupplementaryAgreementUtil } from '@/store/supplementaryAgreements.js'
import SupplementaryAgreementForm from '@/components/forms/SupplementaryAgreementForm.vue'
import { useToastStore } from '@/store/toast.js'

// --- Refs ---
const search = ref('')
const dialog = ref(false)
const VTdialog = ref(false)
const validityTypesFormRef = ref(null)
const statusesDialog = ref(false)
const selectedContract = ref(null)
const filesDialog = ref(false)
const filesDialogLoading = ref(false)
const selectedContractFiles = ref([])

// Paginated contracts data
const contracts = ref([])
const totalContracts = ref(0)
const currentPage = ref(1)
const perPage = ref(50)
const sortBy = ref([])
const sortOrder = ref('asc')
const serverSortByComputed = computed(() => {
  if (!sortBy.value.length) return []
  return [{ key: sortBy.value[0], order: sortOrder.value || 'asc' }]
})
// -1 — соглашение Vuetify для «показать все»; на бэкенд это уходит как per_page=0.
const itemsPerPageOptions = [
  { value: 50, title: '50' },
  { value: 100, title: '100' },
  { value: 200, title: '200' },
  { value: -1, title: 'Все' },
]

// Stats
const fileCounts = ref({})
const updCounts = ref({})
const saCounts = ref({})

// Lookup data
const organizations = ref([])
const respPersons = ref([])
const validityTypes = ref([])
const contractStatuses = ref([])
const pricelist = ref([])

// Dialogs
const pricelistDialog = ref(false)
const saDialog = ref(false)
const updDialog = ref(false)
const updDialogLoading = ref(false)
const selectedSupplementary = ref(null)
const selectedSaFiles = ref([])
const selectedContractUpdFiles = ref([])

// Loading
const loading = ref(false)

// Filters
const yearFilter = ref('all')
const statusFilter = ref(null)

// Stores
const contractStore = ContractUtil()
const organizationStore = OrganizationUtil()
const responsiblePersonStore = ResponsiblePersonUtil()
const validityTypesStore = ValidityTypesUtil()
const contractStatusStore = ContractStatusUtil()
const pricelistStore = PricelistUtil()
const saStore = SupplementaryAgreementUtil()
const route = useRoute()
const router = useRouter()
const toast = useToastStore()

// --- Computed ---
const statusFilterOptions = computed(() => [
  { title: 'Все статусы', value: null, color: null },
  ...contractStatuses.value.map((s) => ({ title: s.name, value: s.id, color: s.color })),
])

// Генерируем годы: текущий-5 … текущий+1
const yearFilterOptions = computed(() => {
  const now = new Date().getFullYear()
  const years = []
  for (let y = now + 1; y >= now - 5; y--) {
    years.push({ title: String(y), value: String(y) })
  }
  return [{ title: 'Все годы', value: 'all' }, ...years]
})

const respPersonsOpt = computed(() =>
  respPersons.value.map((i) => ({ id: i.id, lastname: i.lastname })),
)

const organizationsOpt = computed(() =>
  organizations.value.map((i) => ({ id: i.id, short_name_with_opf: i.short_name_with_opf })),
)

const validityTypesOpt = computed(() =>
  validityTypes.value.map((i) => ({ id: i.id, name: i.name })),
)

// --- Fetch functions ---
let searchTimeout = null

async function fetchContracts() {
  loading.value = true
  lastRequestSignature = requestSignature()
  try {
    const params = {
      page: currentPage.value,
      per_page: perPage.value < 0 ? 0 : perPage.value,
    }
    if (search.value) params.search = search.value
    if (yearFilter.value !== 'all') params.year = yearFilter.value
    if (statusFilter.value !== null) params.status = statusFilter.value
    if (sortBy.value.length) {
      params.sort_by = sortBy.value[0]
      params.sort_order = sortOrder.value || 'asc'
    }

    const result = await contractStore.getPaginatedContracts(params)
    contracts.value = result.items || []
    totalContracts.value = result.total || 0

    // Batch stats
    const stats = await contractStore.getBatchStats()
    fileCounts.value = stats.files || {}
    updCounts.value = stats.upd || {}
    saCounts.value = stats.supplementary || {}
  } catch (e) {
    console.error('Ошибка загрузки договоров:', e)
    toast.push('Не удалось загрузить список договоров', 'error')
  } finally {
    loading.value = false
  }
}

async function fetchLookups() {
  try {
    const [personRows, organizationRows, typeRows, statusRows, pricelistRows] = await Promise.all([
      responsiblePersonStore.getResponsiblePersons(),
      organizationStore.getOrganizations(),
      validityTypesStore.getValidityTypes(),
      contractStatusStore.getStatuses(),
      pricelistStore.getList(),
    ])
    respPersons.value = Array.isArray(personRows) ? personRows : []
    organizations.value = Array.isArray(organizationRows) ? organizationRows : []
    validityTypes.value = Array.isArray(typeRows) ? typeRows : []
    contractStatuses.value = Array.isArray(statusRows) ? statusRows : []
    pricelist.value = Array.isArray(pricelistRows) ? pricelistRows : []
  } catch (e) {
    console.error('Ошибка загрузки справочников:', e)
  }
}

// --- Handlers ---
function onSearchDebounced() {
  clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    currentPage.value = 1
    fetchContracts()
  }, 400)
}

function onFilterChange() {
  currentPage.value = 1
  fetchContracts()
}

/** Подпись текущего запроса — чтобы не дёргать сервер, когда таблица переэмитила те же опции. */
function requestSignature() {
  return JSON.stringify([
    currentPage.value,
    perPage.value,
    sortBy.value,
    sortOrder.value,
    search.value,
    yearFilter.value,
    statusFilter.value,
  ])
}

let lastRequestSignature = null

function onTableOptionsChange(options) {
  if (options.page) currentPage.value = options.page
  if (options.itemsPerPage !== undefined && options.itemsPerPage !== null) {
    perPage.value = options.itemsPerPage
  }
  if (options.sortBy && options.sortBy.length) {
    sortBy.value = options.sortBy.map((s) => s.key)
    sortOrder.value = options.sortBy[0]?.order || 'asc'
  } else {
    sortBy.value = []
    sortOrder.value = 'asc'
  }

  // v-data-table-server эмитит опции и при первом рендере — повторный запрос не нужен.
  if (requestSignature() === lastRequestSignature) return
  fetchContracts()
}

// --- Lifecycle ---
onMounted(async () => {
  await fetchLookups()
  await fetchContracts()
})

watch(
  () => route.query.new,
  async (value) => {
    if (value === '1') {
      openForm()
      const nextQuery = { ...route.query }
      delete nextQuery.new
      await router.replace({ query: nextQuery })
    }
  },
  { immediate: true },
)

// --- Form actions ---
async function openForm(contract = null) {
  if (!contract?.id) {
    selectedContract.value = null
    dialog.value = true
    return
  }

  // В списке приходит облегчённый DTO (без address, comment, additional_agreement,
  // file_link). Открывать форму на нём нельзя: сохранение затрёт эти поля.
  try {
    const full = await contractStore.getContract(contract.id)
    selectedContract.value = { ...contract, ...full }
  } catch (e) {
    console.error('Не удалось загрузить договор', e)
    toast.push('Не удалось загрузить договор', 'error')
    return
  }
  dialog.value = true
}

function openTypeForm() {
  VTdialog.value = true
}

async function saveContract(payload) {
  const contract = { ...(payload?.contract ?? payload) }
  const pendingFiles = Array.isArray(payload?.pendingFiles) ? payload.pendingFiles : []
  const pendingUpdFiles = Array.isArray(payload?.pendingUpdFiles) ? payload.pendingUpdFiles : []

  // Номер и организацию проверяет форма: раньше пустые значения молча
  // подменялись на «б/н <дата>» и организацию-заглушку «Не указано».
  try {
    if (contract.id) {
      await contractStore.updateContract(contract)
      if (pendingFiles.length) await uploadPendingFiles(contract.id, pendingFiles)
      if (pendingUpdFiles.length) await uploadPendingUpdFiles(contract.id, pendingUpdFiles)
      dialog.value = false
      await fetchContracts()
      return
    }

    const created = await contractStore.addContract(contract)
    const contractId = created?.id

    if (contractId && pendingFiles.length) {
      await uploadPendingFiles(contractId, pendingFiles)
    } else if (!contractId && pendingFiles.length) {
      toast.push('Договор сохранен, но не удалось определить ID для загрузки файлов', 'error')
    }

    if (contractId && pendingUpdFiles.length) {
      await uploadPendingUpdFiles(contractId, pendingUpdFiles)
    }

    dialog.value = false
    await fetchContracts()
  } catch (e) {
    console.error('Ошибка сохранения', e)
    toast.push('Ошибка сохранения договора', 'error')
  }
}

async function uploadPendingFiles(contractId, files) {
  const results = await Promise.allSettled(
    files.map((file) => contractStore.uploadFile(contractId, file, 'contract')),
  )
  const ok = results.filter((r) => r.status === 'fulfilled').length
  const fail = results.length - ok
  if (ok) toast.push(`Загружено файлов: ${ok}`, 'success')
  if (fail) toast.push(`Не удалось загрузить файлов: ${fail}`, 'error')
}

async function uploadPendingUpdFiles(contractId, files) {
  const results = await Promise.allSettled(
    files.map((file) => contractStore.uploadFile(contractId, file, 'upd')),
  )
  const ok = results.filter((r) => r.status === 'fulfilled').length
  const fail = results.length - ok
  if (ok) toast.push(`Загружено УПД: ${ok}`, 'success')
  if (fail) toast.push(`Не удалось загрузить УПД: ${fail}`, 'error')
}

async function saveType(type) {
  try {
    await validityTypesStore.addValidityTypes(type)
    validityTypes.value = await validityTypesStore.getValidityTypes()
    validityTypesFormRef.value?.reset()
    toast.push('Тип договора добавлен', 'success')
  } catch (e) {
    console.error('Ошибка сохранения', e)
    toast.push(e.message || 'Ошибка добавления типа договора', 'error')
  }
}

async function deleteType(id) {
  try {
    await validityTypesStore.delValidityTypes(id)
    validityTypes.value = validityTypes.value.filter((t) => t.id !== id)
    toast.push('Тип договора удалён', 'success')
  } catch (e) {
    console.error('Ошибка удаления типа', e)
    toast.push(e.message || 'Ошибка удаления типа договора', 'error')
  }
}

async function saveStatus(dto) {
  try {
    const c = await contractStatusStore.addStatus(dto)
    contractStatuses.value.push(c)
  } catch (e) {
    toast.push(e.message || 'Ошибка добавления статуса', 'error')
  }
}

async function deleteStatus(id) {
  try {
    await contractStatusStore.deleteStatus(id)
    contractStatuses.value = contractStatuses.value.filter((s) => s.id !== id)
    contracts.value.forEach((c) => {
      if (c.contract_status_id === id) c.contract_status_id = null
    })
  } catch (e) {
    toast.push(e.message || 'Ошибка удаления статуса', 'error')
  }
}

async function deleteContract(id) {
  try {
    await contractStore.delContract(id)
    toast.push('Договор удалён', 'success')
    await fetchContracts()
  } catch (e) {
    console.error('Ошибка удаления', e)
    toast.push(e.message || 'Не удалось удалить договор', 'error')
  }
}

async function savePricelistItem(dto) {
  try {
    const c = await pricelistStore.add(dto)
    pricelist.value.push(c)
    toast.push('Позиция добавлена', 'success')
  } catch (e) {
    toast.push(e.message || 'Ошибка добавления позиции', 'error')
  }
}

async function deletePricelistItem(id) {
  try {
    await pricelistStore.delete(id)
    pricelist.value = pricelist.value.filter((p) => p.id !== id)
    toast.push('Позиция удалена', 'success')
  } catch (e) {
    toast.push(e.message || 'Ошибка удаления позиции', 'error')
  }
}

async function handleOpenSupplementaryForm({ contractId, agreement }) {
  selectedSupplementary.value = agreement
    ? { ...agreement, contract_id: contractId }
    : { contract_id: contractId }
  selectedSaFiles.value = []
  if (agreement?.id) {
    try {
      const files = await contractStore.getSaFiles(agreement.id)
      selectedSaFiles.value = Array.isArray(files) ? files : []
    } catch (e) {
      console.error('Failed to load SA files', e)
    }
  }
  saDialog.value = true
}

async function saveSupplementaryAgreement(dto) {
  const pendingFiles = dto.pendingFiles || []
  const cleanDto = { ...dto }
  delete cleanDto.pendingFiles

  try {
    let saId = cleanDto.id
    if (saId) {
      await saStore.update(cleanDto)
      toast.push('Соглашение обновлено', 'success')
    } else {
      const created = await saStore.add(cleanDto)
      saId = created?.id
      toast.push('Соглашение добавлено', 'success')
    }

    if (saId && pendingFiles.length) {
      const cid = cleanDto.contract_id
      const results = await Promise.allSettled(
        pendingFiles.map((file) => contractStore.uploadFile(cid, file, 'supplementary', saId)),
      )
      const ok = results.filter((r) => r.status === 'fulfilled').length
      const fail = results.length - ok
      if (ok) toast.push(`Загружено файлов: ${ok}`, 'success')
      if (fail) toast.push(`Не удалось загрузить файлов: ${fail}`, 'error')
    }

    saDialog.value = false
    if (cleanDto.contract_id) {
      saCounts.value[cleanDto.contract_id] = await saStore.countByContract(cleanDto.contract_id)
      await fetchContracts()
    }
  } catch (e) {
    toast.push(e.message || 'Ошибка сохранения соглашения', 'error')
  }
}

function onSaFileDeleted(fileId) {
  selectedSaFiles.value = selectedSaFiles.value.filter((f) => f.id !== fileId)
}

function handleOrganizationAdded(organization) {
  if (!organization?.id) return
  if (!organizations.value.some((item) => item.id === organization.id))
    organizations.value.push(organization)
}

async function openContractFiles(contract) {
  filesDialogLoading.value = true
  selectedContractFiles.value = []
  filesDialog.value = true
  try {
    const files = await contractStore.getContractFiles(contract.id, 'contract')
    selectedContractFiles.value = Array.isArray(files) ? files : []
    if (selectedContractFiles.value.length === 1) {
      filesDialog.value = false
      downloadFile(selectedContractFiles.value[0].id)
      return
    }
  } catch (error) {
    filesDialog.value = false
    toast.push('Не удалось получить список файлов', 'error')
  } finally {
    filesDialogLoading.value = false
  }
}

async function openContractUpdFiles(contract) {
  updDialogLoading.value = true
  selectedContractUpdFiles.value = []
  updDialog.value = true
  try {
    const files = await contractStore.getContractFiles(contract.id, 'upd')
    selectedContractUpdFiles.value = Array.isArray(files) ? files : []
    if (selectedContractUpdFiles.value.length === 1) {
      updDialog.value = false
      downloadFile(selectedContractUpdFiles.value[0].id)
      return
    }
  } catch (error) {
    updDialog.value = false
    toast.push('Не удалось получить список УПД', 'error')
  } finally {
    updDialogLoading.value = false
  }
}

async function downloadFile(fileId) {
  try {
    await contractStore.downloadFile(fileId)
  } catch (e) {
    toast.push('Не удалось скачать файл', 'error')
  }
}

function formatFileSize(bytes) {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
</script>
