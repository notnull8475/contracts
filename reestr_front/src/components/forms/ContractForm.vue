<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card>
      <v-card-title>{{ contract?.id ? 'Редактировать договор' : 'Добавить договор' }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Номер договора -->
        <v-text-field v-model="form.number" label="Номер договора" />

        <!-- Дата договора -->
        <v-text-field v-model="formattedDate" label="Дата договора" type="date" />

        <!-- ID организации -->
        <v-select
          v-model="form.organization_id"
          :items="organizationsOpt"
          label="Организация"
          item-title="name"
          item-value="id"
        />
        <v-select
          v-model="form.type_of_validity"
          :items="validityTypesOpt"
          label="Тип договора"
          item-title="name"
          item-value="id"
        />

        <!-- Ответственное лицо -->
        <v-select
          v-model="form.responsible_person_id"
          label="Ответственное лицо"
          :items="respPersonsOpt"
          item-title="lastname"
          item-value="id"
        />

        <!-- Адрес -->
        <v-text-field v-model="form.address" label="Адрес" />

        <!-- Дополнительное соглашение -->
        <v-text-field v-model="form.additional_agreement" label="Дополнительное соглашение" />

        <!-- Комментарий -->
        <v-textarea v-model="form.comment" label="Комментарий" />
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
import { reactive, watch, computed } from 'vue'

// Принимаем входные параметры
const props = defineProps(['modelValue', 'contract','organizationsOpt','respPersonsOpt','validityTypesOpt'])
const emit = defineEmits(['update:modelValue', 'save'])

// Реактивная форма
const form = reactive({
  id: null,
  number: '',
  date: null,
  organization_id: null,
  type_of_validity: null,
  responsible_person_id: null,
  address: '',
  additional_agreement: '',
  comment: '',
})

// Форматируем дату для отображения в поле ввода
const formattedDate = computed({
  get() {
    return form.date ? new Date(form.date).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date = value ? new Date(value) : null
  },
})

// Следим за изменениями входного объекта `contract`
watch(
  () => props.contract,
  (newVal) => {
    Object.assign(
      form,
      newVal || {
        id: null,
        number: '',
        date: null,
        organization_id: null,
        type_of_validity: null,
        responsible_person_id: null,
        address: '',
        additional_agreement: '',
        comment: '',
      },
    )
  },
  { immediate: true },
)

// Сохранение данных
function save() {
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>
