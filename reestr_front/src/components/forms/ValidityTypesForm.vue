<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card>
      <v-card-title>
        {{ form.id ? 'Редактировать тип договора' : 'Добавить тип договора' }}
      </v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Название типа договора -->
        <v-text-field
          v-model="form.name"
          label="Название тип договора"
          :error="!!errors.name"
          :error-messages="errors.name"
        />

        <!-- Список уже добавленных типов договоров -->
        <div class="mt-4">
          <!--          <h3>Уже добавленные типы договоров:</h3>-->
          <v-list dense>
            <v-list dense>
              <v-list-item
                v-for="type in validityTypesOpt"
                :key="type.id"
                class="d-flex justify-space-between"
              >
                {{ type.name }}
                <v-btn icon color="error" @click="$emit('delete', type.id)">
                  <v-icon>mdi-delete</v-icon>
                </v-btn>
              </v-list-item>
            </v-list>
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
import { reactive, watch } from 'vue'
import { useNotify } from '@/composables/useNotify.js'

const props = defineProps(['modelValue', 'organization', 'validityTypesOpt'])
const emit = defineEmits(['update:modelValue', 'save'])

const { notifyError } = useNotify()

const form = reactive({
  id: null,
  name: '',
})

const errors = reactive({ name: '' })

watch(
  () => props.organization,
  (newVal) => {
    Object.assign(form, newVal || { id: null, name: '' })
    clearErrors()
  },
  { immediate: true },
)

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
