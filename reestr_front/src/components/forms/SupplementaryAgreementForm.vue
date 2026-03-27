<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="520"
  >
    <v-card rounded="lg">
      <v-card-title class="px-6 pt-5 pb-1">
        {{ form.id ? 'Редактировать соглашение' : 'Новое доп соглашение' }}
      </v-card-title>

      <v-card-text class="px-6 pb-2">
        <v-text-field
          v-model="form.number"
          label="Номер соглашения"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <v-text-field
          v-model="formattedDate"
          label="Дата"
          type="date"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <v-textarea
          v-model="form.description"
          label="Описание"
          variant="outlined"
          density="comfortable"
          rows="2"
          auto-grow
          class="mb-2"
        />

        <v-text-field
          v-model.number="form.price"
          label="Цена (₽)"
          type="number"
          step="0.01"
          min="0"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <v-text-field
          v-model="form.file_link"
          label="Ссылка на файл (URL или путь)"
          variant="outlined"
          density="comfortable"
        />
      </v-card-text>

      <v-divider />

      <v-card-actions class="px-6 py-3">
        <v-spacer />
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
        <v-btn color="primary" @click="save">Сохранить</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, reactive, watch } from 'vue'

const props = defineProps(['modelValue', 'agreement'])
const emit = defineEmits(['update:modelValue', 'save'])

const form = reactive({
  id: null,
  number: '',
  date_from: null,
  description: '',
  file_link: '',
  price: null,
})

watch(
  () => props.agreement,
  (val) => {
    Object.assign(form, val || { id: null, number: '', date_from: null, description: '', file_link: '', price: null })
  },
  { immediate: true },
)

const formattedDate = computed({
  get() {
    return form.date_from ? new Date(form.date_from).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_from = value ? new Date(value + 'T00:00:00').toISOString() : null
  },
})

function save() {
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>
