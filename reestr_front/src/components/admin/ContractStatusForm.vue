<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="520"
  >
    <v-card rounded="lg">
      <v-card-title class="px-6 pt-5 pb-1">
        {{ form.id ? 'Редактировать статус' : 'Новый статус' }}
      </v-card-title>

      <v-card-text class="px-6 pb-2">
        <v-text-field
          v-model="form.name"
          label="Название статуса"
          variant="outlined"
          density="comfortable"
          class="mb-3"
          :error="!!errors.name"
          :error-messages="errors.name"
          @keydown.enter="save"
        />

        <!-- Превью -->
        <div class="d-flex align-center ga-2 mb-3">
          <span class="text-body-2 text-medium-emphasis">Предпросмотр:</span>
          <v-chip :color="form.color" size="small" variant="tonal">
            {{ form.name.trim() || 'Статус' }}
          </v-chip>
        </div>

        <!-- Палитра -->
        <p class="text-caption text-medium-emphasis mb-1">Выберите цвет:</p>
        <v-color-picker
          v-model="form.color"
          :swatches="colorSwatches"
          show-swatches
          hide-canvas
          hide-inputs
          swatches-max-height="260"
          width="100%"
          elevation="0"
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
import { reactive, watch } from 'vue'

const props = defineProps(['modelValue', 'status'])
const emit = defineEmits(['update:modelValue', 'save'])

const form = reactive({ id: null, name: '', color: '#1976D2' })
const errors = reactive({ name: '' })

const colorSwatches = [
  ['#B71C1C', '#880E4F', '#4A148C', '#0D47A1', '#006064'],
  ['#C62828', '#AD1457', '#6A1B9A', '#1565C0', '#00838F'],
  ['#E53935', '#E91E63', '#9C27B0', '#1976D2', '#00ACC1'],
  ['#EF5350', '#F06292', '#CE93D8', '#64B5F6', '#4DD0E1'],
  ['#1B5E20', '#F57F17', '#BF360C', '#3E2723', '#212121'],
  ['#2E7D32', '#F9A825', '#D84315', '#4E342E', '#424242'],
  ['#43A047', '#FDD835', '#F4511E', '#6D4C41', '#757575'],
  ['#81C784', '#FFF176', '#FF8A65', '#A1887F', '#BDBDBD'],
]

watch(
  () => props.status,
  (val) => {
    Object.assign(form, val || { id: null, name: '', color: '#1976D2' })
    errors.name = ''
  },
  { immediate: true },
)

function save() {
  errors.name = ''
  if (!form.name.trim()) {
    errors.name = 'Введите название статуса'
    return
  }
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>
