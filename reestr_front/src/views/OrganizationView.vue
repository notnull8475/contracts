<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5 d-flex flex-wrap justify-space-between align-center ga-3">
      <div>
        <h1 class="text-h5 font-weight-bold mb-1">Реестр организаций</h1>
        <p class="text-body-2 text-medium-emphasis">Карточки контрагентов с реквизитами и адресами</p>
      </div>
      <v-btn color="primary" prepend-icon="mdi-plus" @click="openForm()">Новая организация</v-btn>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-card-text>
      <div class="d-flex flex-wrap ga-2 mb-3">
        <v-chip color="primary" variant="tonal">Всего: {{ organizations.length }}</v-chip>
        <v-chip color="info" variant="tonal">С ОГРН: {{ withOgrnCount }}</v-chip>
      </div>

      <v-text-field
        v-model="search"
        label="Поиск по названию или ИНН"
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
        v-if="!filteredOrganizations.length"
        type="info"
        variant="tonal"
        icon="mdi-information-outline"
      >
        По текущему фильтру организации не найдены.
      </v-alert>

      <OrganizationList v-else :organizations="filteredOrganizations" @edit="openForm" />
    </v-card-text>
  </v-card>

  <organization-form
    v-model="dialog"
    :organization="selectedOrganization"
    @save="saveOrganization"
    @delete="deleteOrganization"
  />
</template>

<script setup>
import { computed, onMounted, ref } from 'vue'
import OrganizationForm from '@/components/forms/OrganizationForm.vue'
import { OrganizationUtil } from '@/store/organizations.js'
import { useNotify } from '@/composables/useNotify.js'
import OrganizationList from '@/components/lists/OrganizationList.vue'

const search = ref('')
const dialog = ref(false)
const selectedOrganization = ref(null)
const organizations = ref([])
const loading = ref(false)

const { notifySuccess, notifyError } = useNotify()
const organizationStore = OrganizationUtil()

const withOgrnCount = computed(() => organizations.value.filter((org) => Boolean(org.ogrn)).length)

const fetchPage = async () => {
  loading.value = true
  try {
    const response = await organizationStore.getOrganizations()
    organizations.value = Array.isArray(response)
      ? response
      : Array.isArray(response?.data)
        ? response.data
        : []
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
  if (!Array.isArray(organizations.value)) return []
  const term = search.value.trim().toLowerCase()
  if (!term) return organizations.value

  return organizations.value.filter((org) => {
    const shortName = (org.short_name_with_opf || '').toLowerCase()
    const inn = String(org.inn || '')
    return shortName.includes(term) || inn.includes(term)
  })
})

function openForm(org = null) {
  selectedOrganization.value = org ? { ...org } : null
  dialog.value = true
}

async function saveOrganization(org) {
  try {
    if (org.id) {
      await organizationStore.updateOrganization(org)
      const idx = organizations.value.findIndex((item) => item.id === org.id)
      if (idx !== -1) organizations.value[idx] = org
      notifySuccess('Организация изменена')
    } else {
      const created = await organizationStore.addOrganization(org)
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
  if (!confirm('Вы уверены, что хотите удалить эту организацию?')) return

  try {
    await organizationStore.delOrganization(id)
    organizations.value = organizations.value.filter((org) => org.id !== id)
    notifySuccess('Организация удалена')
  } catch (e) {
    notifyError('Ошибка удаления', e.message)
    console.error('Ошибка удаления', e)
  }
}
</script>
