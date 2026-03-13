<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-semibold">Договоры</h2>
      <v-btn color="primary" @click="openTypeForm()">Добавить тип договора</v-btn>
      <v-btn color="primary" @click="openForm()">Добавить договор</v-btn>
    </div>

    <v-text-field
      v-model="search"
      placeholder="Поиск по договорам"
      append-inner-icon="mdi-magnify"
      class="mb-4"
    />

    <contract-list
      :contracts="filteredContracts"
      :respPersonsOpt="respPersonsOpt"
      :organizationsOpt="organizationsOpt"
      :validityTypesOpt="validityTypesOpt"
      @edit="openForm"
    />

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
  </div>
</template>
<script setup async>
import { computed, onMounted, ref } from 'vue'
import ContractList from '@/components/lists/ContractList.vue'
import ContractForm from '@/components/forms/ContractForm.vue'
import { ContractUtil } from '@/store/contracts.js'
import { OrganizationUtil } from '@/store/organizations.js'
import { ResponsiblePersonUtil } from '@/store/responsiblePersons.js'
import ValidityTypesForm from '@/components/forms/ValidityTypesForm.vue'
import { ValidityTypesUtil } from '@/store/validityTypes.js'

/* ═══ реактивные переменные ══════════════════════════════════════ */
const search = ref('')
const dialog = ref(false)
const VTdialog = ref(false)
const selectedContract = ref(null)
const contracts = ref([]) // ← всегда стартуем с []
const organizations = ref([]) // ← всегда стартуем с []
const respPersons = ref([]) // ← всегда стартуем с []
const validityTypes = ref([])
/* ═══ утилита доступа к API / хранилищу ══════════════════════════ */
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

/* ═══ загрузка данных  ═══════════════════════════════════════════ */
const fetchPage = async () => {
  try {
    contracts.value = await contractStore.getContracts()
    respPersons.value = await responsiblePersonStore.getResponsiblePersons()
    organizations.value = await organizationStore.getOrganizations()
    validityTypes.value = await validityTypesStore.getValidityTypes()
  } catch (e) {
    console.error('Не удалось получить список договоров', e)
  }
}

onMounted(fetchPage)

/* ═══ фильтр по поиску  ═════════════════════════════════════════ */
const filteredContracts = computed(() =>
  (Array.isArray(contracts.value) ? contracts.value : []).filter(
    (c) => c.number && c.number.toLowerCase().includes(search.value.toLowerCase()),
  ),
)

/* ═══ формы add / edit / delete  ════════════════════════════════ */
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
    } else {
      const created = await contractStore.addContract(contract)
      contracts.value.push(created ?? { ...contract, id: Date.now() })
    }
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
