<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="780"
    scrollable
  >
    <v-card rounded="lg">
      <v-card-title class="px-6 pt-5 pb-2">Статусы договоров</v-card-title>

      <v-card-text class="px-6 pb-4">
        <v-row>
          <!-- ── Левая колонка: форма добавления ── -->
          <v-col cols="12" md="6">
            <p class="text-subtitle-2 text-medium-emphasis mb-3">Добавить новый статус</p>

            <v-text-field
              v-model="form.name"
              label="Название статуса"
              :error="!!errors.name"
              :error-messages="errors.name"
              variant="outlined"
              density="comfortable"
              class="mb-3"
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
            <v-color-picker
              v-model="form.color"
              mode="hex"
              hide-inputs
              :modes="['hex']"
              width="100%"
              elevation="0"
            />

            <v-btn
              class="mt-4"
              color="primary"
              block
              :disabled="!form.name.trim()"
              prepend-icon="mdi-plus"
              @click="save"
            >
              Добавить статус
            </v-btn>
          </v-col>

          <!-- ── Правая колонка: список статусов ── -->
          <v-col cols="12" md="6">
            <p class="text-subtitle-2 text-medium-emphasis mb-3">
              Текущие статусы
              <v-chip size="x-small" class="ml-1">{{ statusesOpt?.length || 0 }}</v-chip>
            </p>

            <v-list density="comfortable" rounded="lg" border>
              <v-list-item
                v-for="status in statusesOpt"
                :key="status.id"
              >
                <template #prepend>
                  <div
                    class="mr-3 rounded"
                    :style="{
                      width: '14px',
                      height: '14px',
                      background: status.color,
                      flexShrink: 0,
                    }"
                  />
                </template>

                <v-list-item-title>
                  <v-chip :color="status.color" size="small" variant="tonal">
                    {{ status.name }}
                  </v-chip>
                </v-list-item-title>

                <template #append>
                  <v-btn
                    icon="mdi-delete-outline"
                    size="small"
                    variant="text"
                    color="error"
                    @click="$emit('delete', status.id)"
                  />
                </template>
              </v-list-item>

              <v-list-item v-if="!statusesOpt?.length">
                <v-list-item-title class="text-medium-emphasis text-body-2">
                  Статусы не добавлены
                </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-col>
        </v-row>
      </v-card-text>

      <v-divider />

      <v-card-actions class="px-6 py-3">
        <v-spacer />
        <v-btn text @click="$emit('update:modelValue', false)">Закрыть</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive } from 'vue'

const props = defineProps(['modelValue', 'statusesOpt'])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])

const form = reactive({ name: '', color: '#1976d2' })
const errors = reactive({ name: '' })

function save() {
  errors.name = ''
  if (!form.name.trim()) {
    errors.name = 'Введите название статуса'
    return
  }
  emit('save', { name: form.name.trim(), color: form.color || '#607d8b' })
  form.name = ''
  form.color = '#1976d2'
}
</script>
