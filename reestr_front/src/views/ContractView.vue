<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5">
      <div class="d-flex flex-wrap justify-space-between align-center ga-3">
        <div>
          <h1 class="text-h5 font-weight-bold mb-1">Реестр договоров</h1>
          <p class="text-body-2 text-medium-emphasis">Рабочий список договоров с быстрым поиском</p>
        </div>

        <div class="d-flex flex-wrap ga-2">
          <v-btn color="secondary" variant="tonal" prepend-icon="mdi-shape-outline" @click="openTypeForm()">
            Типы
          </v-btn>
          <v-btn color="primary" prepend-icon="mdi-plus" @click="openForm()">Новый договор</v-btn>
        </div>
      </div>

      <div class="d-flex flex-wrap ga-2 mt-3">
        <v-select
          v-model="yearFilter"
          :items="yearFilterOptions"
          label="Год"
          density="comfortable"
          hide-details
          variant="outlined"
          style="max-width: 180px"
        />
        <v-select
          v-model="statusFilter"
          :items="statusFilterOptions"
          label="Статус"
          density="comfortable"
          hide-details
          variant="outlined"
          style="max-width: 220px"
        />
        <v-select
          v-model="expiryFilter"
          :items="expiryFilterOptions"
          label="Срок действия"
          density="comfortable"
          hide-details
          variant="outlined"
          style="max-width: 260px"
        />
      </div>

      <div class="d-flex flex-wrap ga-2 mt-4">
        <v-chip color="primary" variant="tonal">Всего: {{ contracts.length }}</v-chip>
        <v-chip color="success" variant="tonal">Актуальные: {{ activeContracts }}</v-chip>
        <v-chip color="warning" variant="tonal">Истекают (30 дн): {{ expiringContracts }}</v-chip>
      </div>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-card-text>
      <v-text-field
        v-model="search"
        label="Поиск по номеру договора"
        prepend-inner-icon="mdi-magnify"
        variant="outlined"
        density="comfortable"
        hide-details
        clearable
      />
    </v-card-text>

    <v-divider />

    <v-progress-linear v-if="loading" indeterminate color="primary" />

    <v-card-text v-else>
      <v-alert
        v-if="!filteredContracts.length"
        type="info"
        variant="tonal"
        icon="mdi-information-outline"
      >
        По текущему фильтру договоров не найдено.
      </v-alert>

      <contract-list
        v-else
        :contracts="filteredContracts"
        :respPersonsOpt="respPersonsOpt"
        :organizationsOpt="organizationsOpt"
        :validityTypesOpt="validityTypesOpt"
        :fileCounts="fileCounts"
        @edit="openForm"
        @files-click="openContractFiles"
      />
    </v-card-text>
  </v-card>

  <contract-form
    v-model="dialog"
    :contract="selectedContract"
    :respPersonsOpt="respPersonsOpt"
    :organizationsOpt="organizationsOpt"
    :organizationsRaw="organizations"
    :validityTypesOpt="validityTypesOpt"
    @save="saveContract"
    @delete="deleteContract"
    @organization-added="handleOrganizationAdded"
  />

  <validity-types-form
    v-model="VTdialog"
    :validityTypesOpt="validityTypesOpt"
    @save="saveType"
    @delete="deleteType"
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
          <v-list-item v-for="file in selectedContractFiles" :key="file.id" @click="downloadFile(file.id)">
            <template #prepend><v-icon>mdi-file-document</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn icon="mdi-download" size="small" variant="text" @click.stop="downloadFile(file.id)" />
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
import { useToastStore } from '@/store/toast.js'

const search = ref('')
const dialog = ref(false)
const VTdialog = ref(false)
const selectedContract = ref(null)
const unknownOrganizationId = ref(null)
const filesDialog = ref(false)
const filesDialogLoading = ref(false)
const selectedContractFiles = ref([])
const contracts = ref([])
const fileCounts = ref({})
const organizations = ref([])
const respPersons = ref([])
const validityTypes = ref([])
const loading = ref(false)
const yearFilter = ref('all')
const statusFilter = ref('all')
const expiryFilter = ref('all')

const contractStore = ContractUtil()
const organizationStore = OrganizationUtil()
const responsiblePersonStore = ResponsiblePersonUtil()
const validityTypesStore = ValidityTypesUtil()
const route = useRoute()
const router = useRouter()
const toast = useToastStore()

const statusFilterOptions = [
  { title: 'Все', value: 'all' },
  { title: 'Только актуальные', value: 'active' },
  { title: 'Только неактуальные', value: 'inactive' },
]

const expiryFilterOptions = [
  { title: 'Любой срок', value: 'all' },
  { title: 'Истекает в 30 дней', value: '30' },
  { title: 'Истекает в 60 дней', value: '60' },
  { title: 'Уже истек', value: 'expired' },
]

const yearFilterOptions = computed(() => {
  const years = new Set()
  contracts.value.forEach((contract) => {
    const sourceDate = contract.date_from || contract.date_to
    if (!sourceDate) return
    const year = new Date(sourceDate).getFullYear()
    if (!Number.isNaN(year)) years.add(String(year))
  })

  return [{ title: 'Все годы', value: 'all' }, ...[...years].sort((a, b) => Number(b) - Number(a)).map((year) => ({ title: year, value: year }))]
})

const respPersonsOpt = computed(() =>
  respPersons.value.map((i) => ({
    id: i.id,
    lastname: i.lastname,
  })),
)

const organizationsOpt = computed(() =>
  organizations.value.map((i) => ({
    id: i.id,
    short_name_with_opf: i.short_name_with_opf,
  })),
)

const validityTypesOpt = computed(() =>
  validityTypes.value.map((i) => ({
    id: i.id,
    name: i.name,
  })),
)

const filteredContracts = computed(() => {
  const rows = Array.isArray(contracts.value) ? contracts.value : []
  const term = search.value.toLowerCase()
  const today = new Date()

  return rows.filter((c) => {
    const numberMatch = !term || (c.number && c.number.toLowerCase().includes(term))

    const statusMatch =
      statusFilter.value === 'all' ||
      (statusFilter.value === 'active' && c.actual) ||
      (statusFilter.value === 'inactive' && !c.actual)

    const contractYear = c.date_from ? String(new Date(c.date_from).getFullYear()) : null
    const yearMatch = yearFilter.value === 'all' || (contractYear && contractYear === yearFilter.value)

    let expiryMatch = true
    if (expiryFilter.value !== 'all') {
      if (!c.date_to) {
        expiryMatch = false
      } else {
        const endDate = new Date(c.date_to)
        const daysLeft = Math.ceil((endDate - today) / (1000 * 60 * 60 * 24))
        if (expiryFilter.value === 'expired') expiryMatch = daysLeft < 0
        if (expiryFilter.value === '30') expiryMatch = daysLeft >= 0 && daysLeft <= 30
        if (expiryFilter.value === '60') expiryMatch = daysLeft >= 0 && daysLeft <= 60
      }
    }

    return numberMatch && statusMatch && expiryMatch && yearMatch
  })
})

const activeContracts = computed(() => contracts.value.filter((c) => c.actual).length)

const expiringContracts = computed(() => {
  const today = new Date()
  const soon = new Date()
  soon.setDate(today.getDate() + 30)
  return contracts.value.filter((c) => {
    if (!c.date_to || !c.actual) return false
    const endDate = new Date(c.date_to)
    return endDate >= today && endDate <= soon
  }).length
})

const fetchPage = async () => {
  loading.value = true
  try {
    const [contractRows, personRows, organizationRows, typeRows] = await Promise.all([
      contractStore.getContracts(),
      responsiblePersonStore.getResponsiblePersons(),
      organizationStore.getOrganizations(),
      validityTypesStore.getValidityTypes(),
    ])

    contracts.value = Array.isArray(contractRows) ? contractRows : []
    await fetchFileCounts()
    respPersons.value = Array.isArray(personRows) ? personRows : []
    organizations.value = Array.isArray(organizationRows) ? organizationRows : []
    validityTypes.value = Array.isArray(typeRows) ? typeRows : []
  } catch (e) {
    console.error('Не удалось получить список договоров', e)
  } finally {
    loading.value = false
  }
}

async function fetchFileCounts() {
  const ids = contracts.value.map((contract) => contract.id).filter(Boolean)
  if (!ids.length) {
    fileCounts.value = {}
    return
  }

  const results = await Promise.allSettled(ids.map((id) => contractStore.getContractFiles(id)))
  const next = {}

  ids.forEach((id, idx) => {
    const result = results[idx]
    next[id] = result.status === 'fulfilled' && Array.isArray(result.value) ? result.value.length : 0
  })

  fileCounts.value = next
}

onMounted(fetchPage)

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

function openForm(contract = null) {
  selectedContract.value = contract ? { ...contract } : null
  dialog.value = true
}

function openTypeForm(type = null) {
  selectedContract.value = type ? { ...type } : null
  VTdialog.value = true
}

async function saveContract(payload) {
  const contract = { ...(payload?.contract ?? payload) }
  const pendingFiles = Array.isArray(payload?.pendingFiles) ? payload.pendingFiles : []

  contract.number = contract.number?.trim() ? contract.number.trim() : `б/н ${new Date().toISOString().slice(0, 10)}`
  contract.organization_id = contract.organization_id || (await ensureOrganizationIdForIncompleteContract())

  try {
    if (contract.id) {
      await contractStore.updateContract(contract)
      const idx = contracts.value.findIndex((c) => c.id === contract.id)
      if (idx !== -1) contracts.value[idx] = contract

      if (pendingFiles.length) {
        await uploadPendingFiles(contract.id, pendingFiles)
      }

      await fetchFileCounts()

      return
    }

    const created = await contractStore.addContract(contract)
    let contractId = created?.id

    if (!contractId) {
      await fetchPage()
      const lastCreated = [...contracts.value]
        .filter((item) => item.number === contract.number)
        .sort((a, b) => b.id - a.id)[0]
      contractId = lastCreated?.id
    } else {
      contracts.value.push(created)
    }

    if (contractId && pendingFiles.length) {
      await uploadPendingFiles(contractId, pendingFiles)
    } else if (!contractId && pendingFiles.length) {
      toast.push('Договор сохранен, но не удалось определить ID для загрузки файлов', 'error')
    }

    await fetchFileCounts()
  } catch (e) {
    console.error('Ошибка сохранения', e)
    toast.push('Ошибка сохранения договора', 'error')
  }
}

async function ensureOrganizationIdForIncompleteContract() {
  if (unknownOrganizationId.value) {
    return unknownOrganizationId.value
  }

  const existing = organizations.value.find((org) => org.short_name_with_opf === 'Не указано')
  if (existing?.id) {
    unknownOrganizationId.value = existing.id
    return existing.id
  }

  const generatedInn = Number(`9${Date.now().toString().slice(-9)}`)
  const created = await organizationStore.addOrganization({
    short_name_with_opf: 'Не указано',
    inn: generatedInn,
    fact_address: '',
    legal_address: '',
    management_post: '',
    management_name: '',
    ogrn: '',
    full_name_with_opf: 'Организация не указана',
    opf_full: '',
    opf_short: '',
  })

  organizations.value.push(created)
  unknownOrganizationId.value = created.id
  return created.id
}

async function uploadPendingFiles(contractId, files) {
  const results = await Promise.allSettled(files.map((file) => contractStore.uploadFile(contractId, file)))
  const successCount = results.filter((result) => result.status === 'fulfilled').length
  const failedCount = results.length - successCount

  if (successCount) {
    toast.push(`Загружено файлов: ${successCount}`, 'success')
  }

  if (failedCount) {
    toast.push(`Не удалось загрузить файлов: ${failedCount}`, 'error')
  }
}

async function saveType(type) {
  try {
    await validityTypesStore.addValidityTypes(type)
    validityTypes.value = await validityTypesStore.getValidityTypes()
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteType(id) {
  try {
    await validityTypesStore.delValidityTypes(id)
    validityTypes.value = validityTypes.value.filter((t) => t.id !== id)
  } catch (e) {
    console.error('Ошибка удаления типа', e)
  }
}

async function deleteContract(id) {
  try {
    await contractStore.delContract(id)
    contracts.value = contracts.value.filter((c) => c.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
    alert(e.message)
  }
}

function handleOrganizationAdded(organization) {
  if (!organization?.id) return
  const exists = organizations.value.some((item) => item.id === organization.id)
  if (!exists) {
    organizations.value.push(organization)
  }
}

async function openContractFiles(contract) {
  filesDialogLoading.value = true
  selectedContractFiles.value = []

  try {
    const files = await contractStore.getContractFiles(contract.id)
    selectedContractFiles.value = Array.isArray(files) ? files : []

    if (selectedContractFiles.value.length === 1) {
      downloadFile(selectedContractFiles.value[0].id)
      return
    }

    filesDialog.value = true
  } catch (error) {
    toast.push('Не удалось получить список файлов', 'error')
  } finally {
    filesDialogLoading.value = false
  }
}

function downloadFile(fileId) {
  contractStore.downloadFile(fileId)
}

function formatFileSize(bytes) {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
</script>
