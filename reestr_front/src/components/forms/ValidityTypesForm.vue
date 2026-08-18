<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card rounded="lg">
      <v-card-title>
        {{ form.id ? 'Редактировать тип договора' : 'Добавить тип договора' }}
      </v-card-title>
      <v-card-text>
        <!-- Название типа договора -->
        <v-text-field
          v-model="form.name"
          label="Название тип договора"
          :error="!!errors.name"
          :error-messages="errors.name"
          variant="outlined"
          density="comfortable"
        />

        <!-- Список уже добавленных типов договоров -->
        <div class="mt-4">
          <!--          <h3>Уже добавленные типы договоров:</h3>-->
          <v-list density="compact">
            <v-list-item
              v-for="type in validityTypesOpt"
              :key="type.id"
              class="d-flex justify-space-between"
            >
              {{ type.name }}
              <template #append>
                <v-btn
                  icon="mdi-delete"
                  size="small"
                  color="error"
                  variant="text"
                  @click="$emit('delete', type.id)"
                />
              </template>
            </v-list-item>
            <v-list-item v-if="!validityTypesOpt || validityTypesOpt.length === 0">
              Пока нет добавленных типов
            </v-list-item>
          </v-list>
        </div>
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
import { reactive } from 'vue'
import { useNotify } from '@/composables/useNotify.js'

defineProps(['modelValue', 'validityTypesOpt'])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])

const { notifyError } = useNotify()

const form = reactive({
  id: null,
  name: '',
})

const errors = reactive({ name: '' })

function clearErrors() {
  errors.name = ''
}

function validateForm() {
  clearErrors()
  let valid = true

  if (!form.name) {
    errors.name = 'Название обязательно'
    valid = false
  }
  return valid
}

function save() {
  if (!validateForm()) {
    notifyError('Ошибка заполнения формы', 'Пожалуйста, исправьте ошибки')
    return
  }

  // Поле очищает родитель после успешного ответа сервера — иначе при ошибке
  // (дубликат, сеть) пользователь получит пустое поле и потеряет введённое.
  emit('save', { ...form, name: form.name.trim() })
}

/** Вызывается родителем после успешного сохранения. */
function reset() {
  form.id = null
  form.name = ''
  clearErrors()
}

defineExpose({ reset })
</script>
