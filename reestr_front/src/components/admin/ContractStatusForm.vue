<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="520"
  >
    <v-card rounded="lg">
      <v-card-title>{{ form.id ? 'Редактировать статус' : 'Новый статус' }}</v-card-title>

      <v-card-text>
        <v-text-field
          v-model="form.name"
          label="Название статуса"
          variant="outlined"
          density="comfortable"
          class="mb-3"
          :error="!!errors.name"
          :error-messages="errors.name"
        />

        <div class="mb-2 d-flex align-center ga-3">
          <span class="text-body-2">Цвет:</span>
          <v-chip :color="form.color" size="small" variant="tonal">{{ form.name || 'Предпросмотр' }}</v-chip>
          <span class="text-caption text-medium-emphasis">{{ form.color }}</span>
        </div>

        <v-color-picker
          v-model="form.color"
          mode="hex"
          hide-inputs
          :modes="['hex']"
          width="100%"
        />
      </v-card-text>

      <v-card-actions>
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

const form = reactive({ id: null, name: '', color: '#607d8b' })
const errors = reactive({ name: '' })

watch(
  () => props.status,
  (val) => {
    Object.assign(form, val || { id: null, name: '', color: '#607d8b' })
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
