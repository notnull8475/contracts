<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="520"
  >
    <v-card rounded="lg">
      <v-card-title class="px-6 pt-5 pb-1">Прайс-лист</v-card-title>

      <v-card-text class="px-6 pb-2">
        <v-text-field
          v-model="form.name"
          label="Название позиции"
          :error="!!errors.name"
          :error-messages="errors.name"
          variant="outlined"
          density="comfortable"
          class="mb-2"
          @keydown.enter="save"
        />

        <v-text-field
          v-model.number="form.price"
          label="Цена (₽)"
          type="number"
          step="0.01"
          min="0"
          :error="!!errors.price"
          :error-messages="errors.price"
          variant="outlined"
          density="comfortable"
          @keydown.enter="save"
        />

        <!-- Список текущих позиций -->
        <div class="mt-4">
          <v-list density="compact" rounded="lg" border>
            <v-list-item
              v-for="item in pricelistOpt"
              :key="item.id"
            >
              <v-list-item-title>{{ item.name }}</v-list-item-title>
              <v-list-item-subtitle>{{ formatPrice(item.price) }}</v-list-item-subtitle>
              <template #append>
                <v-btn
                  icon="mdi-delete-outline"
                  size="small"
                  variant="text"
                  color="error"
                  @click="$emit('delete', item.id)"
                />
              </template>
            </v-list-item>
            <v-list-item v-if="!pricelistOpt?.length">
              <v-list-item-title class="text-medium-emphasis text-body-2">
                Позиции не добавлены
              </v-list-item-title>
            </v-list-item>
          </v-list>
        </div>
      </v-card-text>

      <v-divider />

      <v-card-actions class="px-6 py-3">
        <v-spacer />
        <v-btn color="primary" @click="save">Добавить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Закрыть</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive } from 'vue'

const props = defineProps(['modelValue', 'pricelistOpt'])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])

const form = reactive({ name: '', price: 0 })
const errors = reactive({ name: '', price: '' })

function save() {
  errors.name = ''
  errors.price = ''

  if (!form.name.trim()) {
    errors.name = 'Введите название позиции'
  }
  if (!form.price || form.price <= 0) {
    errors.price = 'Введите корректную цену'
  }

  if (errors.name || errors.price) return

  emit('save', { name: form.name.trim(), price: form.price })
  form.name = ''
  form.price = 0
}

function formatPrice(value) {
  if (value === null || value === undefined) return '—'
  const num = parseFloat(value)
  return isNaN(num) ? '—' : num.toLocaleString('ru-RU', { style: 'currency', currency: 'RUB' })
}
</script>
