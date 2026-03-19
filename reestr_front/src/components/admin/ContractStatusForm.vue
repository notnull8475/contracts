<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="480"
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

        <!-- Превью чипа -->
        <div class="d-flex align-center ga-2 mb-3">
          <span class="text-body-2 text-medium-emphasis">Предпросмотр:</span>
          <v-chip :color="form.color" size="small" variant="tonal">
            {{ form.name.trim() || 'Статус' }}
          </v-chip>
          <span class="text-caption text-medium-emphasis">{{ form.color }}</span>
        </div>

        <!-- Палитра -->
        <v-color-picker
          v-model="form.color"
          mode="hex"
          hide-inputs
          :modes="['hex']"
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

const form = reactive({ id: null, name: '', color: '#1976d2' })
const errors = reactive({ name: '' })

watch(
  () => props.status,
  (val) => {
    Object.assign(form, val || { id: null, name: '', color: '#1976d2' })
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
