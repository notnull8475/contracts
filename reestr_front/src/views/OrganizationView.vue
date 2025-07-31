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

    <organization-list
      :organizations="filteredOrganizations"
      @edit="openForm"
      @delete="deleteOrganization"
    />

    <organization-form
      v-model="dialog"
      :organization="selectedOrganization"
      @save="saveOrganization"
    />
  </div>
</template>
<script setup async>
import { computed, onMounted, ref } from 'vue'
import OrganizationList from '@/components/lists/OrganizationList.vue'
import OrganizationForm from '@/components/forms/OrganizationForm.vue'
import { OrganizationUtil } from '@/store/organizations.js'
import { useNotify } from '@/composables/useNotify.js'


/* ═══ реактивные переменные ══════════════════════════════════════ */
const search = ref('')
const dialog = ref(false)
const selectedOrganization = ref(null)
const organizations = ref([]) // ← всегда стартуем с []
const {notifySuccess, notifyError} = useNotify()

/* ═══ утилита доступа к API / хранилищу ══════════════════════════ */
const organizationUtil = OrganizationUtil()

/* ═══ загрузка данных  ═══════════════════════════════════════════ */
const fetchPage = async () => {
  try {
    organizations.value = await organizationUtil.getOrganizations()
  } catch (e) {
    console.error('Не удалось получить список организаций', e)
  }
}

onMounted(fetchPage)

/* ═══ фильтр по поиску  ═════════════════════════════════════════ */
const filteredOrganizations = computed(() =>
  (Array.isArray(organizations.value) ? organizations.value : []).filter(
    o => o.name && o.name.toLowerCase().includes(search.value.toLowerCase())
  )
)

/* ═══ формы add / edit / delete  ════════════════════════════════ */
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
      organizations.value.push(created ?? { ...org, id: Date.now() })
      notifySuccess("Организация добавлена")
    }
  } catch (e) {
    notifyError('Ошибка сохранения', e.message)
    console.error('Ошибка сохранения', e)
  }
}

async function deleteOrganization(id) {
  try {
    await organizationUtil.delOrganization(id)
    organizations.value = organizations.value.filter(o => o.id !== id)
    notifySuccess(`Организация с идентификатором ${id} удалена`)
  } catch (e) {
    notifyError('Ошибка удаления', e)
    console.error('Ошибка удаления', e)
  }
}
</script>