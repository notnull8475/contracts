<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="600"
  >
    <v-card>
      <v-card-title>{{ form.id ? 'Редактировать договор' : 'Добавить договор' }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Номер договора -->
        <v-text-field v-model="form.number" label="Номер договора" />

        <!-- Дата начала договора -->
        <v-text-field v-model="formattedDateFrom" label="Дата начала договора" type="date" />

        <!-- Дата окончания договора -->
        <v-text-field v-model="formattedDateTo" label="Дата окончания договора" type="date" />

        <!-- Срок действия договора (месяцы) -->
        <v-text-field v-model.number="form.contract_period" label="Срок действия (месяцев)" type="number" min="0" />

        <!-- Выбор организации -->
        <v-autocomplete
          v-model="form.organization_id"
          :items="filteredOrganizations"
          label="Организация"
          item-title="short_name_with_opf"
          item-value="id"
          v-model:search="searchOrganization"
          placeholder="Введите название организации"
          clearable
          hide-no-data
          :menu-props="{ maxHeight: '200px' }"
        />

        <!-- Тип договора -->
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

        <!-- Активен -->
        <v-checkbox v-model="form.actual" label="Актуален" />
      </v-card-text>
      <v-card-actions>
        <v-btn v-if="contract?.id" color="error" @click="deleteItem">Удалить</v-btn>
        <v-spacer />
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { reactive, watch, computed, ref } from 'vue'

const props = defineProps([
  'modelValue',
  'contract',
  'organizationsOpt',
  'respPersonsOpt',
  'validityTypesOpt',
])
const emit = defineEmits(['update:modelValue', 'save', 'delete'])

const form = reactive({
  id: null,
  number: '',
  date_from: null,
  date_to: null,
  contract_period: null,
  organization_id: null,
  type_of_validity: null,
  responsible_person_id: null,
  address: '',
  additional_agreement: '',
  comment: '',
  actual: false,
})

const searchOrganization = ref('')

const filteredOrganizations = computed(() => {
  if (!props.organizationsOpt) return []
  return props.organizationsOpt.filter((org) =>
    org.short_name_with_opf.toLowerCase().includes(searchOrganization.value?.toLowerCase() || '')
  )
})

const formattedDateFrom = computed({
  get() {
    return form.date_from ? new Date(form.date_from).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_from = value ? `${value}T00:00:00` : null
  },
})

const formattedDateTo = computed({
  get() {
    return form.date_to ? new Date(form.date_to).toISOString().split('T')[0] : ''
  },
  set(value) {
    form.date_to = value ? `${value}T00:00:00` : null
  },
})

watch(
  () => props.contract,
  (newVal) => {
    Object.assign(
      form,
      newVal || {
        id: null,
        number: '',
        date_from: null,
        date_to: null,
        contract_period: null,
        organization_id: null,
        type_of_validity: null,
        responsible_person_id: null,
        address: '',
        additional_agreement: '',
        comment: '',
        actual: false,
      }
    )
  },
  { immediate: true }
)

function save() {
  // Normalize date strings as ISO datetime strings ending with T00:00:00
  if (form.date_from && typeof form.date_from === 'string' && !form.date_from.endsWith('T00:00:00')) {
    form.date_from = `${form.date_from.split('T')[0]}T00:00:00`
  }
  if (form.date_to && typeof form.date_to === 'string' && !form.date_to.endsWith('T00:00:00')) {
    form.date_to = `${form.date_to.split('T')[0]}T00:00:00`
  }

  emit('save', { ...form })
  emit('update:modelValue', false)
}

function deleteItem() {
  emit('delete', form.id)
  emit('update:modelValue', false)
}
</script>
