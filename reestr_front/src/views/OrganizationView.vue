<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-semibold">Организации</h2>
      <v-btn color="primary" @click="openForm()">Добавить организацию</v-btn>
    </div>

    <v-text-field
      v-model="search"
      placeholder="Поиск по организациям"
      append-inner-icon="mdi-magnify"
      class="mb-4"
    />

    <!-- Добавляем проверку на загрузку -->
    <v-progress-linear
      v-if="loading"
      indeterminate
      color="primary"
      class="mb-4"
    />

    <OrganizationList
      v-else
      :organizations="filteredOrganizations"
      @edit="openForm"
      @delete="deleteOrganization"
    />

    <v-alert
      v-if="!loading && (!filteredOrganizations || !filteredOrganizations.length)"
      type="info"
      class="mt-4"
    >
      {{ organizations && organizations.length ? 'Организации по вашему запросу не найдены' : 'Организации не найдены' }}
    </v-alert>

    <organization-form
      v-model="dialog"
      :organization="selectedOrganization"
      @save="saveOrganization"
    />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import OrganizationForm from '@/components/forms/OrganizationForm.vue'
import { OrganizationUtil } from '@/store/organizations.js'
import { useNotify } from '@/composables/useNotify.js'
import OrganizationList from '@/components/lists/OrganizationList.vue'

const search = ref('')
const dialog = ref(false)
const selectedOrganization = ref(null)
const organizations = ref([])
const loading = ref(false) // Добавляем флаг загрузки

const { notifySuccess, notifyError } = useNotify()
const organizationUtil = OrganizationUtil()

const fetchPage = async () => {
  loading.value = true
  try {
    const response = await organizationUtil.getOrganizations()
    console.log('Полный ответ от API:', response)

    // В зависимости от структуры ответа API
    if (Array.isArray(response)) {
      organizations.value = response
    } else if (response && Array.isArray(response.data)) {
      organizations.value = response.data
    } else if (response && response.data && Array.isArray(response.data.data)) {
      organizations.value = response.data.data
    } else {
      console.warn('Неожиданная структура ответа:', response)
      organizations.value = []
    }

    console.log('Загруженные организации:', organizations.value)
  } catch (e) {
    console.error('Не удалось получить список организаций', e)
    notifyError('Ошибка загрузки', 'Не удалось загрузить список организаций')
    organizations.value = []
  } finally {
    loading.value = false
  }
}

onMounted(fetchPage)

const filteredOrganizations = computed(() => {
  // Защита от undefined
  if (!organizations.value || !Array.isArray(organizations.value)) {
    return []
  }

  if (!search.value.trim()) {
    return organizations.value
  }

  const searchTerm = search.value.toLowerCase()
  return organizations.value.filter(o =>
    o.short_name_with_opf && o.short_name_with_opf.toLowerCase().includes(searchTerm)
  )
})

function openForm(org = null) {
  selectedOrganization.value = org ? { ...org } : null
  dialog.value = true
}

async function saveOrganization(org) {
  try {
    if (org.id) {
      await organizationUtil.updateOrganization(org)
      const idx = organizations.value.findIndex(o => o.id === org.id)
      if (idx !== -1) organizations.value[idx] = org
      notifySuccess('Организация изменена')
    } else {
      const created = await organizationUtil.addOrganization(org)
      organizations.value.push(created)
      notifySuccess('Организация добавлена')
    }
    dialog.value = false
  } catch (e) {
    notifyError('Ошибка сохранения', e.message)
    console.error('Ошибка сохранения', e)
  }
}

async function deleteOrganization(id) {
  if (!confirm('Вы уверены, что хотите удалить эту организацию?')) {
    return
  }

  try {
    await organizationUtil.delOrganization(id)
    organizations.value = organizations.value.filter(o => o.id !== id)
    notifySuccess(`Организация удалена`)
  } catch (e) {
    notifyError('Ошибка удаления', e.message)
    console.error('Ошибка удаления', e)
  }
}
</script>