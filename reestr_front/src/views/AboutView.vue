<template>
  <v-card rounded="lg" elevation="1" class="mb-4">
    <v-card-text class="py-8 px-6">
      <h1 class="text-h4 font-weight-bold mb-2">Реестр договоров</h1>
      <p class="text-body-1 text-medium-emphasis mb-6">
        Единое рабочее пространство для договоров, организаций и ответственных лиц.
      </p>

      <v-progress-linear v-if="loading" indeterminate color="primary" class="mb-4" />

      <v-row class="mb-2">
        <v-col cols="12" sm="6" md="3">
          <v-card rounded="lg" variant="tonal" color="primary" class="h-100">
            <v-card-item
              prepend-icon="mdi-file-document-multiple"
              title="Договоры"
              :subtitle="String(metrics.contracts)"
            />
          </v-card>
        </v-col>
        <v-col cols="12" sm="6" md="3">
          <v-card rounded="lg" variant="tonal" color="success" class="h-100">
            <v-card-item
              prepend-icon="mdi-check-decagram"
              title="Актуальные"
              :subtitle="String(metrics.activeContracts)"
            />
          </v-card>
        </v-col>
        <v-col cols="12" sm="6" md="3">
          <v-card rounded="lg" variant="tonal" color="info" class="h-100">
            <v-card-item
              prepend-icon="mdi-domain"
              title="Организации"
              :subtitle="String(metrics.organizations)"
            />
          </v-card>
        </v-col>
        <v-col cols="12" sm="6" md="3">
          <v-card rounded="lg" variant="tonal" color="warning" class="h-100">
            <v-card-item
              prepend-icon="mdi-account-tie"
              title="Ответственные"
              :subtitle="String(metrics.responsiblePersons)"
            />
          </v-card>
        </v-col>
      </v-row>

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

<script setup>
import { onMounted, reactive, ref } from 'vue'
import { ContractUtil } from '@/store/contracts.js'
import { useAuthStore } from '@/store/auth.js'

const loading = ref(false)
const metrics = reactive({
  contracts: 0,
  activeContracts: 0,
  organizations: 0,
  responsiblePersons: 0,
})

const contractStore = ContractUtil()
const authStore = useAuthStore()

onMounted(loadDashboard)

/**
 * Счётчики приходят одним лёгким запросом. Раньше ради четырёх чисел грузились
 * полные списки договоров и организаций (больше мегабайта), а «актуальные»
 * считались по несуществующему полю и всегда показывали 0.
 */
async function loadDashboard() {
  if (!authStore.token) return

  loading.value = true
  try {
    const summary = await contractStore.getSummary()
    metrics.contracts = summary.contracts ?? 0
    metrics.activeContracts = summary.active_contracts ?? 0
    metrics.organizations = summary.organizations ?? 0
    metrics.responsiblePersons = summary.responsible_persons ?? 0
  } catch (error) {
    console.error('Ошибка загрузки метрик', error)
  } finally {
    loading.value = false
  }
}
</script>
