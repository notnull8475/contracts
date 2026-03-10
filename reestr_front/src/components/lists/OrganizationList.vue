<template>
  <v-table v-if="organizations && organizations.length">
    <thead>
    <tr>
      <th>ID</th>
      <th>Краткое наименование</th>
      <th>Полное наименование</th>
      <th>ИНН</th>
      <th>ОГРН</th>
      <th>Фактический адрес</th>
      <th>Юридический адрес</th>
      <th>Руководитель</th>
      <th>ОПФ</th>
      <th class="text-right">Действия</th>
    </tr>
    </thead>
    <tbody>
    <tr v-for="org in organizations" :key="org.id">
      <td>{{ org.id }}</td>
      <td class="text-wrap">{{ org.short_name_with_opf }}</td>
      <td class="text-wrap" style="max-width:200px;">
        {{ org.full_name_with_opf || '-' }}
      </td>
      <td>{{ org.inn }}</td>
      <td>{{ org.ogrn || '-' }}</td>
      <td class="text-wrap" style="max-width:180px;">
        {{ org.fact_address || '-' }}
      </td>
      <td class="text-wrap" style="max-width:180px;">
        {{ org.legal_address || '-' }}
      </td>
      <td class="text-wrap" style="max-width:150px;">
        <div v-if="org.management_name">
          <strong>{{ org.management_name }}</strong><br>
          <small class="text-grey-600">{{ org.management_post }}</small>
        </div>
        <span v-else>-</span>
      </td>
      <td class="text-wrap">
        {{ org.opf_short || org.opf_full || '-' }}
      </td>
      <td class="text-right">
        <v-btn icon size="small" @click="$emit('edit', org)" variant="text">
          <v-icon>mdi-pencil</v-icon>
        </v-btn>
        <v-btn
          icon
          size="small"
          color="error"
          @click="$emit('delete', org.id)"
          variant="text"
        >
          <v-icon>mdi-delete</v-icon>
        </v-btn>
      </td>
    </tr>
    </tbody>
  </v-table>
  <v-alert v-else-if="organizations && !organizations.length" type="info" class="mt-4">
    Нет данных для отображения
  </v-alert>
</template>

<script setup>
defineProps({
  organizations: {
    type: Array,
    default: () => []
  }
})
defineEmits(['edit', 'delete'])
</script>

<style scoped>
.text-wrap {
  word-wrap: break-word;
  word-break: break-word;
  white-space: normal;
}
.v-table td {
  vertical-align: top;
  padding: 12px 8px;
}
.text-grey-600 {
  color: rgb(107, 114, 128);
}
</style>