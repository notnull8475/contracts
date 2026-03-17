<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-5 px-5 d-flex flex-wrap justify-space-between align-center ga-3">
      <div>
        <h1 class="text-h5 font-weight-bold mb-1">Ответственные лица</h1>
        <p class="text-body-2 text-medium-emphasis">Сотрудники, закрепленные за договорами</p>
      </div>
      <v-btn color="primary" prepend-icon="mdi-account-plus" @click="openForm()">
        Добавить ответственного
      </v-btn>
    </v-card-text>
  </v-card>

  <v-card rounded="lg" elevation="1">
    <v-card-text>
      <div class="d-flex flex-wrap ga-2 mb-3">
        <v-chip color="primary" variant="tonal">Всего: {{ responsiblePersons.length }}</v-chip>
        <v-chip color="info" variant="tonal">С привязкой к пользователю: {{ linkedUsers }}</v-chip>
      </div>

      <v-text-field
        v-model="search"
        label="Поиск по фамилии"
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
        v-if="!filteredResponsiblePersons.length"
        type="info"
        variant="tonal"
        icon="mdi-information-outline"
      >
        По текущему фильтру ответственные не найдены.
      </v-alert>

      <responsible-person-list
        v-else
        :responsiblePersons="filteredResponsiblePersons"
        :userOptions="userOptions"
        @edit="openForm"
      />
    </v-card-text>
  </v-card>

  <responsible-person-form
    v-model="dialog"
    :responsiblePerson="selectedResponsiblePerson"
    :userOptions="userOptions"
    @save="saveResponsiblePerson"
    @delete="deleteResponsiblePerson"
  />
</template>

<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ResponsiblePersonList from '@/components/lists/ResponsiblePersonList.vue'
import ResponsiblePersonForm from '@/components/forms/ResponsiblePersonForm.vue'
import { ResponsiblePersonUtil } from '@/store/responsiblePersons.js'
import { UserUtil } from '@/store/users.js'
import { useAuthStore } from '@/store/auth.js'

const search = ref('')
const dialog = ref(false)
const selectedResponsiblePerson = ref(null)
const responsiblePersons = ref([])
const users = ref([])
const loading = ref(false)

const responsiblePersonStore = ResponsiblePersonUtil()
const userStore = UserUtil()
const authStore = useAuthStore()
const route = useRoute()
const router = useRouter()

const isAdmin = computed(() => authStore.user?.role === 'admin')

const userOptions = computed(() =>
  users.value.map((user) => ({
    id: user.id,
    username: user.username,
  })),
)

const linkedUsers = computed(() => responsiblePersons.value.filter((p) => p.user_id).length)

const fetchPage = async () => {
  loading.value = true
  try {
    const personRows = await responsiblePersonStore.getResponsiblePersons()
    responsiblePersons.value = Array.isArray(personRows) ? personRows : []

    if (isAdmin.value) {
      try {
        const userRows = await userStore.getAllUsers()
        users.value = Array.isArray(userRows) ? userRows : []
      } catch (e) {
        console.error('Не удалось получить список пользователей', e)
        users.value = []
      }
    } else {
      users.value = []
    }
  } catch (e) {
    console.error('Не удалось получить список ответственных лиц', e)
    responsiblePersons.value = []
  } finally {
    loading.value = false
  }
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

const filteredResponsiblePersons = computed(() => {
  const rows = Array.isArray(responsiblePersons.value) ? responsiblePersons.value : []
  if (!search.value) return rows
  const term = search.value.toLowerCase()
  return rows.filter((person) => person.lastname && person.lastname.toLowerCase().includes(term))
})

function openForm(person = null) {
  selectedResponsiblePerson.value = person ? { ...person } : null
  dialog.value = true
}

async function saveResponsiblePerson(person) {
  try {
    if (person.id) {
      await responsiblePersonStore.updateResponsiblePerson(person)
      const idx = responsiblePersons.value.findIndex((p) => p.id === person.id)
      if (idx !== -1) responsiblePersons.value[idx] = person
      return
    }

    const created = await responsiblePersonStore.addResponsiblePerson(person)
    responsiblePersons.value.push(created ?? { ...person, id: Date.now() })
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteResponsiblePerson(id) {
  try {
    await responsiblePersonStore.delResponsiblePerson(id)
    responsiblePersons.value = responsiblePersons.value.filter((p) => p.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
    alert(e.message)
  }
}
</script>
