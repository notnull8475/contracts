<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="500">
    <v-card>
      <v-card-title>{{ user?.id ? 'Редактировать пользователя' : 'Добавить пользователя' }}</v-card-title>
      <v-card-text>
        <v-text-field v-model="form.username" label="Имя"/>
        <v-text-field v-model="form.login" label="Логин"/>
        <v-text-field type="password" v-model="form.password_hash" label="Пароль"/>
        <v-select
            v-model="form.role"
            :items="roles"
            label="Роль"
            item-title="name"
            item-value="id"
        />
        <!--        item-title="name" &lt;!&ndash; если roles это [{id, name}] &ndash;&gt;-->
        <!--        item-value="id"   &lt;!&ndash; если roles это [{id, name}] &ndash;&gt;-->
      </v-card-text>
      <v-card-actions>
        <v-spacer/>
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="$emit('update:modelValue', false)">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import {reactive, ref, watch} from 'vue'
import {UserUtil} from '@/store/users.js' // путь поменяй под свой проект

const props = defineProps(['modelValue', 'user'])
const emit = defineEmits(['update:modelValue', 'save'])

const form = reactive({username: '', login: '', role: '', id: null, password_hash: ''})
const roles = ref([])

watch(() => props.user, (newVal) => {
  Object.assign(form, newVal || {username: '', login: '', role: '', id: null, password_hash: ''})
}, {immediate: true})

// Загружать роли при открытии диалога
watch(() => props.modelValue, async (opened) => {
  if (opened) {
    roles.value = await UserUtil().getRoles()
  }
}, {immediate: true})

function save() {
  emit('save', {...form})
  emit('update:modelValue', false)
}
</script>
