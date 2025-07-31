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

      <v-text-field
        v-model="form.name"
        label="Название организации"
        :class="{ 'error-field': errors.name }"
      />
      <div v-if="errors.name" class="error-message">{{ errors.name }}</div>

      <v-text-field
        v-model="form.inn"
        label="ИНН"
        type="number"
        :class="{ 'error-field': errors.inn }"
      />
      <div v-if="errors.inn" class="error-message">{{ errors.inn }}</div>

      <v-text-field
        v-model="form.fact_address"
        label="Фактический адрес"
        :class="{ 'error-field': errors.fact_address }"
      />
      <div v-if="errors.fact_address" class="error-message">{{ errors.fact_address }}</div>

      <v-text-field
        v-model="form.address"
        label="Юридический адрес"
        :class="{ 'error-field': errors.address }"
      />
      <div v-if="errors.address" class="error-message">{{ errors.address }}</div>
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive, watch } from 'vue'

// Принимаем входные параметры
const props = defineProps(['modelValue', 'organization'])
const emit = defineEmits(['update:modelValue', 'save'])

// Реактивная форма
const form = reactive({
  id: null,
  name: '',
  inn: null,
  fact_address: '',
  address: '',
})
const errors = reactive({
  name: null,
  inn: null,
  fact_address: null,
  address: null
})
// Следим за изменениями входного объекта `organization`
watch(
  () => props.organization,
  (newVal) => {
    Object.assign(
      form,
      newVal || {
        id: null,
        name: '',
        inn: null,
        fact_address: '',
        address: '',
      },
    )
  },
  { immediate: true },
)

// Сохранение данных
function save() {
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>
