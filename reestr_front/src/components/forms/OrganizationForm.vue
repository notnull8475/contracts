<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600">
    <v-card>
      <v-card-title>{{ organization?.id ? 'Редактировать организацию' : 'Добавить организацию' }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Название организации -->
        <v-text-field v-model="form.name" label="Название организации" />

        <!-- ИНН -->
        <v-text-field v-model="form.inn" label="ИНН" type="number" />

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
  address: ''
})

// Следим за изменениями входного объекта `organization`
watch(() => props.organization, (newVal) => {
  Object.assign(form, newVal || {
    id: null,
    name: '',
    inn: null,
    fact_address: '',
    address: ''
  })
}, { immediate: true })

// Сохранение данных
function save() {
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>