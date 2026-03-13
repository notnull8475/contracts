<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-8 px-6">
      <h1 class="text-h4 font-weight-bold mb-2">Реестр договоров</h1>
      <p class="text-body-1 text-medium-emphasis mb-6">
        Единое рабочее пространство для договоров, организаций и ответственных лиц.
      </p>

      <v-row>
        <v-col cols="12" md="4">
          <v-card rounded="lg" variant="outlined" class="h-100">
            <v-card-title>Договоры</v-card-title>
            <v-card-text class="text-medium-emphasis">
              Контроль актуальности, сроков, типов и прикрепленных файлов.
            </v-card-text>
            <v-card-actions>
              <v-btn color="primary" variant="tonal" to="/contracts">Открыть</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>

        <v-col cols="12" md="4">
          <v-card rounded="lg" variant="outlined" class="h-100">
            <v-card-title>Организации</v-card-title>
            <v-card-text class="text-medium-emphasis">
              Реквизиты контрагентов, ИНН/ОГРН, адреса и данные руководителя.
            </v-card-text>
            <v-card-actions>
              <v-btn color="primary" variant="tonal" to="/organizations">Открыть</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>

        <v-col cols="12" md="4">
          <v-card rounded="lg" variant="outlined" class="h-100">
            <v-card-title>Ответственные</v-card-title>
            <v-card-text class="text-medium-emphasis">
              Учет ответственных сотрудников и связей с системными пользователями.
            </v-card-text>
            <v-card-actions>
              <v-btn color="primary" variant="tonal" to="/responsibleperson">Открыть</v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>
