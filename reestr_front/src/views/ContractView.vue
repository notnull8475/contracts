<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-semibold">Договоры</h2>
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
      @edit="openForm"
      @delete="deleteContract"
    />

    <contract-form
      v-model="dialog"
      :contract="selectedContract"
      @save="saveContract"
    />
  </div>
</template>
<script setup async>
import { computed, onMounted, ref } from 'vue'
import ContractList from '@/components/lists/ContractList.vue'
import ContractForm from '@/components/forms/ContractForm.vue'
import { ContractUtil } from '@/store/contracts.js'

/* ═══ реактивные переменные ══════════════════════════════════════ */
const search = ref('')
const dialog = ref(false)
const selectedContract = ref(null)
const contracts = ref([]) // ← всегда стартуем с []

/* ═══ утилита доступа к API / хранилищу ══════════════════════════ */
const contractUtil = ContractUtil()

/* ═══ загрузка данных  ═══════════════════════════════════════════ */
const fetchPage = async () => {
  try {
    contracts.value = await contractUtil.getContracts()
  } catch (e) {
    console.error('Не удалось получить список договоров', e)
  }
}

onMounted(fetchPage)

/* ═══ фильтр по поиску  ═════════════════════════════════════════ */
const filteredContracts = computed(() =>
  (Array.isArray(contracts.value) ? contracts.value : []).filter(
    c => c.number && c.number.toLowerCase().includes(search.value.toLowerCase())
  )
)

/* ═══ формы add / edit / delete  ════════════════════════════════ */
function openForm(contract = null) {
  selectedContract.value = contract ? { ...contract } : null
  dialog.value = true
}

async function saveContract(contract) {
  try {
    if (contract.id) {
      await contractUtil.updateContract(contract)
      const idx = contracts.value.findIndex(c => c.id === contract.id)
      if (idx !== -1) contracts.value[idx] = contract
    } else {
      const created = await contractUtil.addContract(contract)
      contracts.value.push(created ?? { ...contract, id: Date.now() })
    }
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteContract(id) {
  try {
    await contractUtil.deleteContract(id)
    contracts.value = contracts.value.filter(c => c.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
  }
}
</script>