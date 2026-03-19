<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card rounded="lg">
      <v-card-title>Статусы договоров</v-card-title>

      <v-card-text>
        <v-text-field
          v-model="form.name"
          label="Название статуса"
          :error="!!errors.name"
          :error-messages="errors.name"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />

        <div class="d-flex align-center ga-3 mb-3">
          <v-text-field
            v-model="form.color"
            label="Цвет (HEX)"
            variant="outlined"
            density="comfortable"
            hide-details
            style="max-width: 180px"
          />
          <v-chip :color="form.color || '#607d8b'" size="small" variant="tonal">
            {{ form.name || 'Предпросмотр' }}
          </v-chip>
        </div>

        <div class="mt-4">
          <v-list density="compact">
            <v-list-item
              v-for="status in statusesOpt"
              :key="status.id"
              class="d-flex justify-space-between"
            >
              <template #prepend>
                <v-chip :color="status.color" size="small" variant="tonal" class="mr-2">
                  {{ status.name }}
                </v-chip>
              </template>
              <span class="text-caption text-medium-emphasis">{{ status.color }}</span>
              <template #append>
                <v-btn
                  icon="mdi-delete"
                  size="small"
                  color="error"
                  variant="text"
                  @click="$emit('delete', status.id)"
                />
              </template>
            </v-list-item>
            <v-list-item v-if="!statusesOpt || statusesOpt.length === 0">
              Пока нет добавленных статусов
            </v-list-item>
          </v-list>
        </div>
      </v-card-text>

      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" @click="save">Добавить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Закрыть</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive } from 'vue'

const props = defineProps(['modelValue', 'statusesOpt'])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])

const form = reactive({ name: '', color: '#607d8b' })
const errors = reactive({ name: '' })

function save() {
  errors.name = ''
  if (!form.name.trim()) {
    errors.name = 'Название обязательно'
    return
  }
  emit('save', { name: form.name.trim(), color: form.color || '#607d8b' })
  form.name = ''
  form.color = '#607d8b'
}
</script>
