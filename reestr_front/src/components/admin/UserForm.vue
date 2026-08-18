<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    max-width="560"
  >
    <v-card rounded="lg">
      <v-card-title>{{
        user?.id ? 'Редактировать пользователя' : 'Новый пользователь'
      }}</v-card-title>

      <v-card-text>
        <v-text-field
          v-model="form.username"
          label="Имя"
          variant="outlined"
          density="comfortable"
        />
        <v-text-field v-model="form.login" label="Логин" variant="outlined" density="comfortable" />
        <v-text-field
          type="password"
          v-model="form.password_hash"
          :label="isEdit ? 'Новый пароль' : 'Пароль'"
          :hint="isEdit ? 'Оставьте пустым, чтобы не менять пароль' : 'Обязательное поле'"
          persistent-hint
          autocomplete="new-password"
          variant="outlined"
          density="comfortable"
          class="mb-2"
        />
        <v-select
          v-model="form.role"
          :items="roles"
          label="Роль"
          variant="outlined"
          density="comfortable"
        />
        <v-switch
          v-model="form.is_active"
          :label="form.is_active ? 'Учётная запись активна' : 'Учётная запись отключена'"
          color="primary"
          hide-details
        />
        <v-alert v-if="errorMessage" type="error" variant="tonal" class="mt-3">
          {{ errorMessage }}
        </v-alert>
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
import { UserUtil } from '@/store/users.js'

const props = defineProps(['modelValue', 'user'])
const emit = defineEmits(['update:modelValue', 'save'])

const userStore = UserUtil()

function emptyForm() {
  return { id: null, username: '', login: '', role: 'user', password_hash: '', is_active: true }
}

const form = reactive(emptyForm())
const roles = ref([])
const errorMessage = ref('')

const isEdit = computed(() => Boolean(form.id))

watch(
  () => props.user,
  (newVal) => {
    // Полный сброс: приходящий пользователь не содержит пароля, и без сброса
    // в поле остался бы пароль, набранный для предыдущего пользователя.
    Object.assign(form, emptyForm())
    if (newVal) {
      for (const [key, value] of Object.entries(newVal)) {
        if (key in form && key !== 'password_hash') form[key] = value
      }
      form.is_active = newVal.is_active !== false
    }
    errorMessage.value = ''
  },
  { immediate: true },
)

watch(
  () => props.modelValue,
  async (opened) => {
    if (!opened) return
    try {
      roles.value = await userStore.getRoles()
    } catch (e) {
      roles.value = ['admin', 'moderator', 'user']
    }
  },
  { immediate: true },
)

function save() {
  errorMessage.value = ''

  if (!form.login.trim()) {
    errorMessage.value = 'Укажите логин'
    return
  }
  if (!isEdit.value && !form.password_hash) {
    errorMessage.value = 'Укажите пароль для нового пользователя'
    return
  }

  const payload = {
    id: form.id,
    login: form.login.trim(),
    username: form.username.trim(),
    role: form.role,
    is_active: form.is_active,
  }

  // При редактировании пустое поле означает «пароль не менять».
  if (form.password_hash) payload.password_hash = form.password_hash

  // Диалог закрывает родитель — только после успешного ответа сервера,
  // иначе ошибка валидации на бэкенде уничтожит введённые данные.
  emit('save', payload)
}

/** Родитель сообщает об ошибке сохранения, чтобы показать её прямо в форме. */
function showError(message) {
  errorMessage.value = message
}

defineExpose({ showError })
</script>
