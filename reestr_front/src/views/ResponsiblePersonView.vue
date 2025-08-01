<template>
  <div class="p-6">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-2xl font-semibold">Ответственные лица</h2>
      <v-btn color="primary" @click="openForm()">Добавить ответственное лицо</v-btn>
    </div>

    <v-text-field
      v-model="search"
      placeholder="Поиск по ответственным лицам"
      append-inner-icon="mdi-magnify"
      class="mb-4"
    />

    <responsible-person-list
      :responsiblePersons="filteredResponsiblePersons"
      :userOptions="userOptions"
      @edit="openForm"
      @delete="deleteResponsiblePerson"
    />

    <responsible-person-form
      v-model="dialog"
      :responsiblePerson="selectedResponsiblePerson"
      :userOptions="userOptions"
      @save="saveResponsiblePerson"
    />
  </div>
</template>
<script setup async>
import { computed, onMounted, ref, watch } from 'vue'
import ResponsiblePersonList from '@/components/lists/ResponsiblePersonList.vue'
import ResponsiblePersonForm from '@/components/forms/ResponsiblePersonForm.vue'
import { ResponsiblePersonUtil } from '@/store/responsiblePersons.js'
import { UserUtil } from '@/store/users.js'

/* ═══ реактивные переменные ══════════════════════════════════════ */
const search = ref('')
const dialog = ref(false)
const selectedResponsiblePerson = ref(null)
const responsiblePersons = ref([]) // ← всегда стартуем с []
const users = ref([])
/* ═══ утилита доступа к API / хранилищу ══════════════════════════ */
const responsiblePersonUtil = ResponsiblePersonUtil()

const userOptions = computed(() =>
  users.value.map((user) => ({
    id: user.id,
    username: user.username,
  })),
)
/* ═══ загрузка данных  ═══════════════════════════════════════════ */
const fetchPage = async () => {
  try {
    responsiblePersons.value = await responsiblePersonUtil.getResponsiblePersons()
    users.value = await UserUtil().getAllUsers()
  } catch (e) {
    console.error('Не удалось получить список ответственных лиц', e)
  }
}
onMounted(fetchPage)

/* ═══ фильтр по поиску  ═════════════════════════════════════════ */
const filteredResponsiblePersons = computed(() =>
  (Array.isArray(responsiblePersons.value) ? responsiblePersons.value : []).filter(
    (p) => p.lastname && p.lastname.toLowerCase().includes(search.value.toLowerCase()),
  ),
)

/* ═══ формы add / edit / delete  ════════════════════════════════ */
function openForm(person = null) {
  selectedResponsiblePerson.value = person ? { ...person } : null
  dialog.value = true
}

async function saveResponsiblePerson(person) {
  try {
    if (person.id) {
      await responsiblePersonUtil.updateResponsiblePerson(person)
      const idx = responsiblePersons.value.findIndex((p) => p.id === person.id)
      if (idx !== -1) responsiblePersons.value[idx] = person
    } else {
      const created = await responsiblePersonUtil.addResponsiblePerson(person)
      responsiblePersons.value.push(created ?? { ...person, id: Date.now() })
    }
  } catch (e) {
    console.error('Ошибка сохранения', e)
  }
}

async function deleteResponsiblePerson(id) {
  try {
    await responsiblePersonUtil.deleteResponsiblePerson(id)
    responsiblePersons.value = responsiblePersons.value.filter((p) => p.id !== id)
  } catch (e) {
    console.error('Ошибка удаления', e)
  }
}
</script>
