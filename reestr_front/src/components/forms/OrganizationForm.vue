<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card>
      <v-card-title>{{
        organization?.id ? 'Редактировать организацию' : 'Добавить организацию'
      }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Название организации -->
        <v-text-field
          v-model="form.name"
          label="Название организации"
          :error="!!errors.name"
          :error-messages="errors.name"
        />

        <!-- ИНН -->
        <v-text-field
          v-model="form.inn"
          label="ИНН"
          type="number"
          :error="!!errors.inn"
          :error-messages="errors.inn"
        />

        <!-- Фактический адрес -->
        <v-text-field v-model="form.fact_address" label="Фактический адрес" />

        <!-- Юридический адрес -->
        <v-text-field v-model="form.address" label="Юридический адрес" />
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

const props = defineProps(['modelValue', 'organization'])
const emit = defineEmits(['update:modelValue', 'save'])

const { notifyError } = useNotify()

const form = reactive({
  id: null,
  name: '',
  inn: null,
  fact_address: '',
  address: '',
})

const errors = reactive({ name: '', inn: '' })

watch(
  () => props.organization,
  (newVal) => {
    Object.assign(form, newVal || { id: null, name: '', inn: null, fact_address: '', address: '' })
    clearErrors()
  },
  { immediate: true },
)

function clearErrors() {
  errors.name = ''
  errors.inn = ''
}

function validateForm() {
  clearErrors()
  let valid = true

  if (!form.name) {
    errors.name = 'Название обязательно'
    valid = false
  }

  const innLength = String(form.inn).length
  if (!form.inn || isNaN(Number(form.inn)) || innLength < 10 || innLength > 12) {
    errors.inn = 'ИНН должен быть числом длиной от 10 до 12 символов'
    valid = false
  }

  return valid
}

async function save() {
  if (!validateForm()) {
    notifyError('Ошибка заполнения формы', 'Пожалуйста, исправьте ошибки')
    return
  }

  try {
    await emit('save', { ...form })
    emit('update:modelValue', false)
  } catch (e) {
    notifyError('Ошибка сохранения', e.message)
  }
}
</script>
