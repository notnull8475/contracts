<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="800"
  >
    <v-card>
      <v-card-title>{{
          organization?.id ? 'Редактировать организацию' : 'Добавить организацию'
        }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- ИНН с кнопкой автозаполнения -->
        <div class="d-flex align-center gap-2">
          <v-text-field
            v-model="form.inn"
            label="ИНН"
            type="number"
            :error="!!errors.inn"
            :error-messages="errors.inn"
            class="flex-grow-1"
          />
          <v-btn
            color="primary"
            variant="outlined"
            @click="fillByInn"
            :loading="isLoadingInn"
            :disabled="!form.inn || !!errors.inn"
          >
            Заполнить
          </v-btn>
        </div>

        <!-- Краткое наименование с ОПФ -->
        <v-text-field
          v-model="form.short_name_with_opf"
          label="Краткое наименование с ОПФ"
          :error="!!errors.short_name_with_opf"
          :error-messages="errors.short_name_with_opf"
        />

        <!-- Полное наименование с ОПФ -->
        <v-text-field
          v-model="form.full_name_with_opf"
          label="Полное наименование с ОПФ"
        />

        <!-- Фактический адрес -->
        <v-text-field v-model="form.fact_address" label="Фактический адрес" />

        <!-- Юридический адрес -->
        <v-text-field v-model="form.legal_address" label="Юридический адрес" />

        <!-- ОГРН -->
        <v-text-field v-model="form.ogrn" label="ОГРН" />

        <!-- Должность руководителя -->
        <v-text-field v-model="form.management_post" label="Должность руководителя" />

        <!-- ФИО руководителя -->
        <v-text-field v-model="form.management_name" label="ФИО руководителя" />

        <!-- Полная ОПФ -->
        <v-text-field v-model="form.opf_full" label="Организационно-правовая форма (полная)" />

        <!-- Краткая ОПФ -->
        <v-text-field v-model="form.opf_short" label="Организационно-правовая форма (краткая)" />
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive, watch, ref } from 'vue'
import { useNotify } from '@/composables/useNotify.js'
import { OrganizationUtil } from '@/store/organizations.js'

const props = defineProps(['modelValue', 'organization'])
const emit = defineEmits(['update:modelValue', 'save'])

const { notifyError, notifySuccess } = useNotify()
const organizationStore = OrganizationUtil()

const isLoadingInn = ref(false)

const form = reactive({
  id: null,
  short_name_with_opf: '',
  inn: null,
  fact_address: '',
  legal_address: '',
  management_post: '',
  management_name: '',
  ogrn: '',
  full_name_with_opf: '',
  opf_full: '',
  opf_short: '',
})

const errors = reactive({
  short_name_with_opf: '',
  inn: ''
})

watch(
  () => props.organization,
  (newVal) => {
    Object.assign(form, newVal || {
      id: null,
      short_name_with_opf: '',
      inn: null,
      fact_address: '',
      legal_address: '',
      management_post: '',
      management_name: '',
      ogrn: '',
      full_name_with_opf: '',
      opf_full: '',
      opf_short: '',
    })
    clearErrors()
  },
  { immediate: true },
)

function clearErrors() {
  errors.short_name_with_opf = ''
  errors.inn = ''
}

function validateForm() {
  clearErrors()
  let valid = true

  if (!form.short_name_with_opf) {
    errors.short_name_with_opf = 'Название обязательно'
    valid = false
  }

  const innLength = String(form.inn).length
  if (!form.inn || isNaN(Number(form.inn)) || innLength < 10 || innLength > 12) {
    errors.inn = 'ИНН должен быть числом длиной от 10 до 12 символов'
    valid = false
  }

  return valid
}

async function fillByInn() {
  if (!form.inn) {
    notifyError('Ошибка', 'Укажите ИНН для автозаполнения')
    return
  }

  // Валидация ИНН перед запросом
  const innLength = String(form.inn).length
  if (isNaN(Number(form.inn)) || innLength < 10 || innLength > 12) {
    notifyError('Ошибка', 'Некорректный ИНН')
    return
  }

  isLoadingInn.value = true

  try {
    const orgData = await organizationStore.getOrganizationByInn(form.inn)
    // if (response && response.data) {
    if (orgData) {

      console.log(orgData)

      // Заполняем форму данными с бэкенда
      form.short_name_with_opf = orgData.short_name_with_opf || ''
      form.full_name_with_opf = orgData.full_name_with_opf || ''
      form.fact_address = orgData.fact_address || ''
      form.legal_address = orgData.legal_address || ''
      form.management_post = orgData.management_post || ''
      form.management_name = orgData.management_name || ''
      form.ogrn = orgData.ogrn || ''
      form.opf_full = orgData.opf_full || ''
      form.opf_short = orgData.opf_short || ''

      notifySuccess('Успешно', 'Информация об организации загружена')
      clearErrors()
    }
  } catch (error) {
    console.error('Ошибка при получении данных по ИНН:', error)
    notifyError('Ошибка', 'Не удалось получить данные организации по ИНН')
  } finally {
    isLoadingInn.value = false
  }
}

async function save() {
  if (!validateForm()) {
    notifyError('Ошибка заполнения формы', 'Пожалуйста, исправьте ошибки')
    return
  }

  try {
    // Преобразуем ИНН в число перед отправкой
    const formData = {
      ...form,
      inn: Number(form.inn) // Приводим к числу
    }

    await emit('save', formData)
    emit('update:modelValue', false)
  } catch (e) {
    notifyError('Ошибка сохранения', e.message)
  }
}
</script>

<style scoped>
.gap-2 {
  gap: 8px;
}
</style>