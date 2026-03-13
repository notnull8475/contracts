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
        @edit="openForm"
      />
    </v-card-text>
  </v-card>

  <contract-form
    v-model="dialog"
    :contract="selectedContract"
    :respPersonsOpt="respPersonsOpt"
    :organizationsOpt="organizationsOpt"
    :validityTypesOpt="validityTypesOpt"
    @save="saveContract"
    @delete="deleteContract"
  />

  <validity-types-form
    v-model="VTdialog"
    :validityTypesOpt="validityTypesOpt"
    @save="saveType"
    @delete="deleteType"
  />
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import ContractList from '@/components/lists/ContractList.vue'
import ContractForm from '@/components/forms/ContractForm.vue'
import { ContractUtil } from '@/store/contracts.js'
import { OrganizationUtil } from '@/store/organizations.js'
import { ResponsiblePersonUtil } from '@/store/responsiblePersons.js'
import ValidityTypesForm from '@/components/forms/ValidityTypesForm.vue'
import { ValidityTypesUtil } from '@/store/validityTypes.js'

const search = ref('')
const dialog = ref(false)
const VTdialog = ref(false)
const selectedContract = ref(null)
const contracts = ref([])
const organizations = ref([])
const respPersons = ref([])
const validityTypes = ref([])
const loading = ref(false)

const contractStore = ContractUtil()
const organizationStore = OrganizationUtil()
const responsiblePersonStore = ResponsiblePersonUtil()
const validityTypesStore = ValidityTypesUtil()

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
  if (!search.value) return rows
  const term = search.value.toLowerCase()
  return rows.filter((c) => c.number && c.number.toLowerCase().includes(term))
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
    respPersons.value = Array.isArray(personRows) ? personRows : []
    organizations.value = Array.isArray(organizationRows) ? organizationRows : []
    validityTypes.value = Array.isArray(typeRows) ? typeRows : []
  } catch (e) {
    console.error('Не удалось получить список договоров', e)
  } finally {
    loading.value = false
  }
}

onMounted(fetchPage)

function openForm(contract = null) {
  selectedContract.value = contract ? { ...contract } : null
  dialog.value = true
}

function openTypeForm(type = null) {
  selectedContract.value = type ? { ...type } : null
  VTdialog.value = true
}

async function saveContract(contract) {
  try {
    if (contract.id) {
      await contractStore.updateContract(contract)
      const idx = contracts.value.findIndex((c) => c.id === contract.id)
      if (idx !== -1) contracts.value[idx] = contract
      return
    }

    const created = await contractStore.addContract(contract)
    contracts.value.push(created ?? { ...contract, id: Date.now() })
  } catch (e) {
    console.error('Ошибка сохранения', e)
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
</script>
