<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600">
    <v-card>
      <v-card-title>{{ responsiblePerson?.id ? 'Редактировать ответственное лицо' : 'Добавить ответственное лицо' }}</v-card-title>
      <v-card-text>
        <!-- ID -->
        <v-text-field v-model="form.id" label="ID" disabled />

        <!-- Имя -->
        <v-text-field v-model="form.firstname" label="Имя" />

        <!-- Фамилия -->
        <v-text-field v-model="form.lastname" label="Фамилия" />

        <!-- Отчество -->
        <v-text-field v-model="form.name" label="Отчество" />

        <v-select
          v-if="role === 'admin'"
          v-model="form.user_id"
          :items="userOptions"
          label="Пользователь системы"
          item-title="username"
          item-value="id"
        />
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
import { computed, reactive, ref, watch } from 'vue'
import {useAuthStore} from '@/store/auth';
import { UserUtil } from '@/store/users.js'
const authStore = useAuthStore();
// Принимаем входные параметры
const props = defineProps(['modelValue', 'responsiblePerson','userOptions'])
const emit = defineEmits(['update:modelValue', 'save'])

// Реактивная форма
const form = reactive({
  id: null,
  firstname: '',
  lastname: '',
  name: '',
  user_id: null
})
const users = ref([])
const role = computed(() => {
  if (authStore.token) {
    return authStore.user.role;
  } else return null;
});

// Следим за изменениями входного объекта `responsiblePerson`
watch(() => props.responsiblePerson,

  (newVal) => {
  Object.assign(form, newVal || {
    id: null,
    firstname: '',
    lastname: '',
    name: '',
    user_id: null
  })
}, { immediate: true })



// Сохранение данных
function save() {
  emit('save', { ...form })
  emit('update:modelValue', false)
}
</script>