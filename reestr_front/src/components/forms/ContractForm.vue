<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="onDialogToggle"
    max-width="1100"
    scrollable
    persistent
  >
    <v-card rounded="lg">
      <v-card-title>{{ form.id ? 'Редактировать договор' : 'Добавить договор' }}</v-card-title>

      <v-card-text>
        <v-row dense>
          <v-col cols="12" md="4">
            <v-text-field
              v-model="form.number"
              label="Номер договора *"
              variant="outlined"
              density="comfortable"
              :error="!!errors.number"
              :error-messages="errors.number"
              :hint="duplicateNumberHint"
              :persistent-hint="!!duplicateNumberHint"
              @blur="checkDuplicateNumber"
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model="formattedDateFrom"
              label="Дата начала договора"
              type="date"
              :error="!!errors.date_from"
              :error-messages="errors.date_from"
              variant="outlined"
              density="comfortable"
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model="formattedDateTo"
              label="Дата окончания договора"
              type="date"
              :error="!!errors.date_to"
              :error-messages="errors.date_to"
              variant="outlined"
              density="comfortable"
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model.number="form.contract_period"
              label="Срок действия (мес.)"
              type="number"
              min="0"
              step="1"
              variant="outlined"
              density="comfortable"
              hint="Считается по датам; можно задать вручную — дата окончания пересчитается"
              persistent-hint
              @change="onPeriodInput"
              @keydown.enter="commitPeriodAndSave"
            />
          </v-col>

          <v-col cols="12" md="8">
            <v-autocomplete
              v-model="form.organization_id"
              :items="orgAutocompleteItems"
              label="Организация *"
              item-title="display"
              item-value="id"
              v-model:search="searchOrganization"
              placeholder="Введите название или ИНН"
              clearable
              variant="outlined"
              density="comfortable"
              :error="!!errors.organization_id"
              :error-messages="errors.organization_id"
              :menu-props="{ maxHeight: '220px' }"
            >
              <template #selection="{ item }">
                {{ item?.raw?.display ?? organizationDisplayLabel }}
              </template>
              <template #no-data>
                <div class="pa-3 d-flex flex-column ga-2">
                  <span class="text-body-2 text-medium-emphasis"
                    >Организация не найдена в справочнике</span
                  >
                  <v-btn
                    color="primary"
                    size="small"
                    variant="tonal"
                    prepend-icon="mdi-plus"
                    @click="openQuickOrganizationDialog"
                  >
                    Добавить новую организацию
                  </v-btn>
                </div>
              </template>
            </v-autocomplete>
          </v-col>

          <v-col cols="12" md="6">
            <v-select
              v-model="form.type_of_validity"
              :items="validityTypesOpt"
              label="Тип договора"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Не задан"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-select
              v-model="form.responsible_person_id"
              label="Ответственное лицо"
              :items="respPersonsOpt"
              item-title="lastname"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Не задано"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.address"
              label="Адрес"
              variant="outlined"
              density="comfortable"
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12" md="6">
            <v-text-field
              v-model="form.additional_agreement"
              label="Доп. соглашение (текстом)"
              hint="Свободная заметка. Полноценные соглашения — в разделе ниже"
              variant="outlined"
              density="comfortable"
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12">
            <v-textarea
              v-model="form.comment"
              label="Комментарий"
              variant="outlined"
              density="comfortable"
              rows="2"
              auto-grow
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-select
              v-model="form.pricelist_id"
              :items="pricelistOpt"
              label="Позиция прайса"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Выберите..."
              @update:model-value="onPricelistSelected"
            >
              <template #selection="{ item }">
                {{ item.raw.name }} — {{ formatPrice(item.raw.price) }}
              </template>
              <template #item="{ item, props: itemProps }">
                <v-list-item v-bind="itemProps">
                  <template #append>
                    <span class="text-caption text-medium-emphasis">{{
                      formatPrice(item.raw.price)
                    }}</span>
                  </template>
                </v-list-item>
              </template>
            </v-select>
          </v-col>

          <v-col cols="12" md="4">
            <v-text-field
              v-model.number="form.price"
              label="Цена (₽)"
              type="number"
              step="0.01"
              min="0"
              variant="outlined"
              density="comfortable"
              :hint="isCustomPrice ? 'Спец. цена' : undefined"
              persistent-hint
              @keydown.enter="save"
            />
          </v-col>

          <v-col cols="12" md="4">
            <v-select
              v-model="form.contract_status_id"
              :items="statusesOpt"
              label="Статус"
              item-title="name"
              item-value="id"
              variant="outlined"
              density="comfortable"
              clearable
              placeholder="Не задан"
            >
              <template #selection="{ item }">
                <v-chip :color="item.raw.color" size="small" variant="tonal">{{
                  item.title
                }}</v-chip>
              </template>
              <template #item="{ item, props: itemProps }">
                <v-list-item v-bind="itemProps">
                  <template #prepend>
                    <v-chip :color="item.raw.color" size="x-small" class="mr-2" />
                  </template>
                </v-list-item>
              </template>
            </v-select>
          </v-col>
        </v-row>

        <v-divider class="my-3" />

        <v-alert v-if="!form.id" type="info" variant="tonal" density="compact" class="mb-3">
          Файлы можно выбрать сейчас — они загрузятся сразу после сохранения договора. Доп.
          соглашения и история появятся после сохранения.
        </v-alert>

        <h4 class="text-subtitle-1 mb-2">Файлы</h4>

        <v-row dense>
          <v-col cols="12" md="9">
            <v-file-input
              v-model="newFile"
              label="Выберите файл"
              accept=".pdf,.doc,.docx,.xls,.xlsx,.jpg,.jpeg,.png"
              prepend-icon="mdi-paperclip"
              :loading="uploading"
              variant="outlined"
              density="comfortable"
              show-size
              clearable
            />
          </v-col>
          <v-col cols="12" md="3">
            <v-btn
              block
              color="primary"
              :disabled="!selectedUploadFile || uploading"
              :loading="uploading"
              @click="handleFileUpload"
            >
              {{ form.id ? 'Загрузить файл' : 'Добавить файл' }}
            </v-btn>
          </v-col>
        </v-row>

        <v-list v-if="form.id" density="compact">
          <v-list-item v-for="file in files" :key="file.id">
            <template #prepend><v-icon>mdi-file-document</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-download"
                size="small"
                variant="text"
                @click="downloadFile(file.id)"
              />
              <v-btn
                icon="mdi-delete"
                size="small"
                variant="text"
                color="error"
                @click="removeFile(file.id)"
              />
            </template>
          </v-list-item>
          <v-list-item v-if="!files.length">
            <v-list-item-title class="text-medium-emphasis">Файлы не прикреплены</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-list v-else density="compact">
          <v-list-item v-for="(file, idx) in pendingFiles" :key="`${file.name}-${idx}`">
            <template #prepend><v-icon>mdi-file-plus</v-icon></template>
            <v-list-item-title>{{ file.name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-close"
                size="small"
                variant="text"
                color="error"
                @click="removePendingFile(idx)"
              />
            </template>
          </v-list-item>
          <v-list-item v-if="!pendingFiles.length">
            <v-list-item-title class="text-medium-emphasis"
              >Файлы в очередь не добавлены</v-list-item-title
            >
          </v-list-item>
        </v-list>

        <v-divider class="my-3" />

        <h4 class="text-subtitle-1 mb-2">УПД (счета-фактуры, акты)</h4>

        <v-row dense>
          <v-col cols="12" md="9">
            <v-file-input
              v-model="newUpdFile"
              label="Выберите файл УПД"
              accept=".pdf,.doc,.docx,.xls,.xlsx,.jpg,.jpeg,.png"
              prepend-icon="mdi-file-document-outline"
              :loading="uploadingUpd"
              variant="outlined"
              density="comfortable"
              show-size
              clearable
            />
          </v-col>
          <v-col cols="12" md="3">
            <v-btn
              block
              color="warning"
              :disabled="!selectedUploadUpdFile || uploadingUpd"
              :loading="uploadingUpd"
              @click="handleUpdUpload"
            >
              {{ form.id ? 'Загрузить УПД' : 'Добавить УПД' }}
            </v-btn>
          </v-col>
        </v-row>

        <v-list v-if="form.id" density="compact">
          <v-list-item v-for="file in updFiles" :key="file.id">
            <template #prepend><v-icon color="warning">mdi-file-document-outline</v-icon></template>
            <v-list-item-title>{{ file.original_name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.file_size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-download"
                size="small"
                variant="text"
                @click="downloadFile(file.id)"
              />
              <v-btn
                icon="mdi-delete"
                size="small"
                variant="text"
                color="error"
                @click="removeUpdFile(file.id)"
              />
            </template>
          </v-list-item>
          <v-list-item v-if="!updFiles.length">
            <v-list-item-title class="text-medium-emphasis">УПД не прикреплены</v-list-item-title>
          </v-list-item>
        </v-list>

        <v-list v-else density="compact">
          <v-list-item v-for="(file, idx) in pendingUpdFiles" :key="`${file.name}-${idx}`">
            <template #prepend><v-icon color="warning">mdi-file-plus</v-icon></template>
            <v-list-item-title>{{ file.name }}</v-list-item-title>
            <v-list-item-subtitle>{{ formatFileSize(file.size) }}</v-list-item-subtitle>
            <template #append>
              <v-btn
                icon="mdi-close"
                size="small"
                variant="text"
                color="error"
                @click="removePendingUpdFile(idx)"
              />
            </template>
          </v-list-item>
          <v-list-item v-if="!pendingUpdFiles.length">
            <v-list-item-title class="text-medium-emphasis"
              >УПД в очередь не добавлены</v-list-item-title
            >
          </v-list-item>
        </v-list>

        <v-divider class="my-3" />

        <v-expansion-panels v-if="form.id" class="mb-3">
          <v-expansion-panel>
            <v-expansion-panel-title class="text-subtitle-2">
              <v-icon size="small" class="mr-2">mdi-history</v-icon>
              История изменений
              <v-chip v-if="historyItems.length" size="x-small" class="ml-2">{{
                historyItems.length
              }}</v-chip>
            </v-expansion-panel-title>
            <v-expansion-panel-text>
              <v-progress-linear v-if="historyLoading" indeterminate color="primary" class="mb-2" />
              <v-list v-if="historyItems.length" density="compact">
                <v-list-item v-for="h in historyItems" :key="h.id">
                  <template #prepend>
                    <v-icon size="small" :color="historyColor(h.action)">{{
                      historyIcon(h.action)
                    }}</v-icon>
                  </template>
                  <v-list-item-title class="text-body-2">{{
                    historyLabel(h.action)
                  }}</v-list-item-title>
                  <v-list-item-subtitle v-if="h.description">{{
                    h.description
                  }}</v-list-item-subtitle>
                  <template #append>
                    <span class="text-caption text-medium-emphasis">{{
                      formatHistoryDate(h.created_at)
                    }}</span>
                  </template>
                </v-list-item>
              </v-list>
              <p v-else-if="!historyLoading" class="text-caption text-medium-emphasis">
                Нет записей
              </p>
            </v-expansion-panel-text>
          </v-expansion-panel>
        </v-expansion-panels>

        <template v-if="form.id">
          <h4 class="text-subtitle-1 mb-2">Дополнительные соглашения</h4>

          <v-btn
            size="small"
            color="primary"
            variant="tonal"
            prepend-icon="mdi-plus"
            class="mb-2"
            @click="openSupplementaryForm()"
          >
            Добавить соглашение
          </v-btn>

          <v-list density="compact">
            <v-list-item v-for="sa in supplementaryAgreements" :key="sa.id">
              <template #prepend><v-icon>mdi-file-document-outline</v-icon></template>
              <v-list-item-title>
                {{ sa.number || 'Без номера' }}
                <span v-if="sa.date_from"> от {{ formatDate(sa.date_from) }}</span>
              </v-list-item-title>
              <v-list-item-subtitle>
                {{ sa.description || '—' }}
                <span v-if="sa.price"> · {{ formatPrice(sa.price) }}</span>
              </v-list-item-subtitle>
              <template #append>
                <v-btn
                  icon="mdi-pencil"
                  size="small"
                  variant="text"
                  @click="openSupplementaryForm(sa)"
                />
                <v-btn
                  icon="mdi-delete"
                  size="small"
                  variant="text"
                  color="error"
                  @click="removeSupplementaryAgreement(sa.id)"
                />
              </template>
            </v-list-item>
            <v-list-item v-if="!supplementaryAgreements.length">
              <v-list-item-title class="text-medium-emphasis">Нет доп соглашений</v-list-item-title>
            </v-list-item>
          </v-list>
        </template>
      </v-card-text>

      <v-card-actions>
        <v-btn v-if="form.id" color="error" @click="deleteItem">Удалить</v-btn>
        <v-spacer />
        <v-btn color="primary" @click="save">Сохранить</v-btn>
        <v-btn text @click="requestClose">Отмена</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <v-dialog v-model="quickOrgDialog" max-width="680">
    <v-card rounded="lg">
      <v-card-title>Новая организация</v-card-title>
      <v-card-text>
        <v-row dense>
          <v-col cols="12" md="8">
            <v-text-field
              v-model="quickOrg.short_name_with_opf"
              label="Краткое наименование"
              variant="outlined"
              density="comfortable"
              :error="Boolean(quickOrgErrors.short_name_with_opf)"
              :error-messages="quickOrgErrors.short_name_with_opf"
            />
          </v-col>
          <v-col cols="12" md="4">
            <v-text-field
              v-model="quickOrg.inn"
              label="ИНН"
              variant="outlined"
              density="comfortable"
              :error="Boolean(quickOrgErrors.inn)"
              :error-messages="quickOrgErrors.inn"
            />
          </v-col>
          <v-col cols="12">
            <v-text-field
              v-model="quickOrg.full_name_with_opf"
              label="Полное наименование"
              variant="outlined"
              density="comfortable"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="quickOrg.legal_address"
              label="Юридический адрес"
              variant="outlined"
              density="comfortable"
            />
          </v-col>
          <v-col cols="12" md="6">
            <v-text-field
              v-model="quickOrg.fact_address"
              label="Фактический адрес"
              variant="outlined"
              density="comfortable"
            />
          </v-col>
        </v-row>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn text @click="quickOrgDialog = false">Отмена</v-btn>
        <v-btn color="primary" :loading="quickOrgSaving" @click="saveQuickOrganization"
          >Сохранить</v-btn
        >
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed, reactive, ref, toRefs, watch } from 'vue'
import { ContractUtil } from '@/store/contracts'
import { blankToNull, normalizePeriod } from '@/utils/form.js'
import {
  addMonths,
  calculateContractPeriod,
  dateValueForPicker,
  isPlausibleDate,
  toApiDateTime,
} from '@/utils/dates.js'
import { OrganizationUtil } from '@/store/organizations'
import { SupplementaryAgreementUtil } from '@/store/supplementaryAgreements'
import { useToastStore } from '@/store/toast'

const props = defineProps([
  'modelValue',
  'contract',
  'organizationsOpt',
  'organizationsRaw',
  'respPersonsOpt',
  'validityTypesOpt',
  'statusesOpt',
  'pricelistOpt',
])
const emit = defineEmits([
  'update:modelValue',
  'save',
  'delete',
  'organization-added',
  'open-supplementary-form',
])
const {
  modelValue,
  contract,
  organizationsOpt,
  organizationsRaw,
  respPersonsOpt,
  validityTypesOpt,
  statusesOpt,
  pricelistOpt,
} = toRefs(props)

const contractStore = ContractUtil()
const organizationStore = OrganizationUtil()
const saStore = SupplementaryAgreementUtil()
const toast = useToastStore()

/** Дефолты формы — единый источник, чтобы поля не «протекали» между договорами. */
function emptyForm() {
  return {
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
    contract_status_id: null,
    file_link: null,
    price: null,
    pricelist_id: null,
  }
}

const form = reactive(emptyForm())

const newFile = ref(null)
const files = ref([])
const pendingFiles = ref([])
const uploading = ref(false)
const newUpdFile = ref(null)
const updFiles = ref([])
const pendingUpdFiles = ref([])
const uploadingUpd = ref(false)
const searchOrganization = ref('')
const quickOrgDialog = ref(false)
const quickOrgSaving = ref(false)
const supplementaryAgreements = ref([])
const supplementaryFormDialog = ref(false)
const selectedSupplementary = ref(null)
const isCustomPrice = ref(false)
const historyItems = ref([])
const historyLoading = ref(false)
const errors = reactive({ number: '', organization_id: '', date_from: '', date_to: '' })
const duplicateNumberHint = ref('')
/** Снимок формы на момент открытия — чтобы отличить заполненную форму от нетронутой. */
const pristineSnapshot = ref('')

const quickOrg = reactive({
  short_name_with_opf: '',
  inn: '',
  full_name_with_opf: '',
  legal_address: '',
  fact_address: '',
  management_post: '',
  management_name: '',
  ogrn: '',
  opf_full: '',
  opf_short: '',
})

const quickOrgErrors = reactive({
  short_name_with_opf: '',
  inn: '',
})

const filteredOrganizations = computed(() => {
  const source =
    Array.isArray(organizationsRaw.value) && organizationsRaw.value.length
      ? organizationsRaw.value
      : organizationsOpt.value

  if (!source) return []

  const term = (searchOrganization.value || '').toLowerCase()
  if (!term) return source

  return source.filter((org) => {
    const shortName = (org.short_name_with_opf || '').toLowerCase()
    const fullName = (org.full_name_with_opf || '').toLowerCase()
    const inn = String(org.inn || '')
    return shortName.includes(term) || fullName.includes(term) || inn.includes(term)
  })
})

const orgAutocompleteItems = computed(() =>
  filteredOrganizations.value.map((org) => ({
    ...org,
    display: org.inn ? `${org.short_name_with_opf} (ИНН ${org.inn})` : org.short_name_with_opf,
  })),
)

/** Подпись выбранной организации, если слот selection не отдал raw */
watch(
  () => form.organization_id,
  (value) => {
    if (value) errors.organization_id = ''
  },
)

watch(
  () => form.number,
  (value) => {
    if (value?.trim()) errors.number = ''
  },
)

const organizationDisplayLabel = computed(() => {
  const id = form.organization_id
  if (id == null || id === '') return ''
  return orgAutocompleteItems.value.find((o) => o.id === id)?.display ?? ''
})

const selectedUploadFile = computed(() => resolveSelectedFile(newFile.value))
const selectedUploadUpdFile = computed(() => resolveSelectedFile(newUpdFile.value))

// Срок и даты пересчитываются только от ввода пользователя, а не от изменения
// состояния: watcher'ы на form.* срабатывали и на заполнение формы из БД, и друг
// на друга (ввод даты окончания → срок → дата окончания заменялась на
// «начало + N месяцев»), и «Сохранить» молча портил дату.
//
// Инвариант: если есть обе даты, срок равен разнице между ними в месяцах;
// срок сам по себе хранится только пока даты окончания нет.
const formattedDateFrom = computed({
  get() {
    return dateValueForPicker(form.date_from)
  },
  set(value) {
    form.date_from = value || null
    errors.date_from = ''
    // Набор года с клавиатуры даёт «0002-…», «0020-…» — по ним не считаем.
    if (form.date_from && !isPlausibleDate(form.date_from)) return
    // Срок ввели раньше даты начала — теперь из него можно получить дату окончания.
    if (form.date_from && !form.date_to && normalizePeriod(form.contract_period) > 0) {
      onPeriodInput()
      return
    }
    syncPeriodFromDates()
  },
})

const formattedDateTo = computed({
  get() {
    return dateValueForPicker(form.date_to)
  },
  set(value) {
    form.date_to = value || null
    errors.date_to = ''
    if (form.date_to && !isPlausibleDate(form.date_to)) return
    syncPeriodFromDates()
  },
})

/** Срок считается по датам; без любой из них срока нет. */
function syncPeriodFromDates() {
  form.contract_period = calculateContractPeriod(form.date_from, form.date_to)
}

/**
 * Пользователь зафиксировал срок (change/Enter, не каждая клавиша — иначе
 * стирание «88» через «8» переписывало дату окончания промежуточным значением):
 * дата окончания = дата начала + срок в месяцах. Пустой или нулевой срок
 * возвращается к значению по датам, дату окончания не трогает.
 */
function onPeriodInput() {
  const period = normalizePeriod(form.contract_period)
  // Пустой срок, а также нулевой при двух датах, — срок определяется датами.
  if (period === null || (period === 0 && form.date_to)) {
    syncPeriodFromDates()
    return
  }
  if (period !== form.contract_period) form.contract_period = period
  if (!period || !isPlausibleDate(form.date_from)) return
  form.date_to = addMonths(form.date_from, period)
}

/** Enter в поле срока: сначала применить срок, потом сохранить. */
function commitPeriodAndSave() {
  onPeriodInput()
  save()
}

watch(
  () => contract.value,
  (newVal) => {
    isCustomPrice.value = false

    // Сначала полный сброс на дефолты: приходящий объект может не содержать
    // часть полей, и без сброса в форме остались бы значения прошлого договора.
    Object.assign(form, emptyForm())
    if (newVal) {
      for (const [key, value] of Object.entries(newVal)) {
        if (key in form) form[key] = value
      }
    }
    // Даты держим как «YYYY-MM-DD»: API отдаёт «…T00:00:00», и повторный ввод
    // того же срока записывал бы ту же дату в другом виде, делая форму «изменённой».
    form.date_from = dateValueForPicker(form.date_from) || null
    form.date_to = dateValueForPicker(form.date_to) || null
    // Срок показываем как в БД, даже если он не сходится с датами (импорт):
    // иначе список и карточка показывали бы разные числа без единой правки.

    newFile.value = null
    pendingFiles.value = []
    newUpdFile.value = null
    pendingUpdFiles.value = []
    files.value = []
    updFiles.value = []
    supplementaryAgreements.value = []
    historyItems.value = []
    errors.number = ''
    errors.organization_id = ''
    errors.date_from = ''
    errors.date_to = ''
    duplicateNumberHint.value = ''
    pristineSnapshot.value = JSON.stringify(form)

    if (newVal?.id) {
      loadFiles(newVal.id)
      loadUpdFiles(newVal.id)
      loadSupplementaryAgreements(newVal.id)
      loadHistory(newVal.id)
    }
  },
  { immediate: true },
)

async function loadFiles(contractId) {
  try {
    files.value = await contractStore.getContractFiles(contractId)
  } catch (e) {
    console.error('Failed to load files:', e)
  }
}

async function handleFileUpload() {
  const file = selectedUploadFile.value

  if (!file) {
    toast.push('Сначала выберите файл', 'error')
    return
  }

  if (form.id) {
    uploading.value = true
    try {
      const uploadedFile = await contractStore.uploadFile(form.id, file)
      files.value.push(uploadedFile)
      toast.push('Файл загружен', 'success')
    } catch (e) {
      toast.push(e.message, 'error')
    } finally {
      uploading.value = false
      newFile.value = null
    }
    return
  }

  pendingFiles.value.push(file)
  newFile.value = null
  toast.push('Файл добавлен в очередь', 'success')
}

function resolveSelectedFile(fileInput) {
  if (!fileInput) return null
  if (fileInput instanceof File) return fileInput
  if (Array.isArray(fileInput)) return fileInput[0] || null
  if (fileInput?.target?.files?.length) return fileInput.target.files[0]
  if (fileInput?.length && fileInput[0] instanceof File) return fileInput[0]
  return null
}

function removePendingFile(index) {
  pendingFiles.value.splice(index, 1)
}

async function loadUpdFiles(contractId) {
  try {
    updFiles.value = await contractStore.getContractFiles(contractId, 'upd')
  } catch (e) {
    console.error('Failed to load UPD files:', e)
  }
}

async function handleUpdUpload() {
  const file = selectedUploadUpdFile.value

  if (!file) {
    toast.push('Сначала выберите файл УПД', 'error')
    return
  }

  if (form.id) {
    uploadingUpd.value = true
    try {
      const uploadedFile = await contractStore.uploadFile(form.id, file, 'upd')
      updFiles.value.push(uploadedFile)
      toast.push('УПД загружен', 'success')
    } catch (e) {
      toast.push(e.message, 'error')
    } finally {
      uploadingUpd.value = false
      newUpdFile.value = null
    }
    return
  }

  pendingUpdFiles.value.push(file)
  newUpdFile.value = null
  toast.push('УПД добавлен в очередь', 'success')
}

function removePendingUpdFile(index) {
  pendingUpdFiles.value.splice(index, 1)
}

async function removeUpdFile(fileId) {
  try {
    await contractStore.deleteFile(fileId)
    updFiles.value = updFiles.value.filter((f) => f.id !== fileId)
    toast.push('УПД удален', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

async function loadSupplementaryAgreements(contractId) {
  try {
    supplementaryAgreements.value = await saStore.getByContract(contractId)
  } catch (e) {
    console.error('Failed to load supplementary agreements:', e)
  }
}

async function loadHistory(contractId) {
  historyLoading.value = true
  try {
    historyItems.value = await contractStore.getContractHistory(contractId)
  } catch (e) {
    console.error('Failed to load history:', e)
  } finally {
    historyLoading.value = false
  }
}

function historyIcon(action) {
  const map = {
    sa_added: 'mdi-file-plus',
    sa_updated: 'mdi-file-edit',
    sa_deleted: 'mdi-file-remove',
    price_changed: 'mdi-currency-rub',
    status_changed: 'mdi-flag',
    contract_updated: 'mdi-pencil',
  }
  return map[action] || 'mdi-circle'
}

function historyColor(action) {
  const map = {
    sa_added: 'success',
    sa_updated: 'info',
    sa_deleted: 'error',
    price_changed: 'warning',
    status_changed: 'primary',
    contract_updated: 'secondary',
  }
  return map[action] || 'grey'
}

function historyLabel(action) {
  const map = {
    sa_added: 'Добавлено доп. соглашение',
    sa_updated: 'Изменено доп. соглашение',
    sa_deleted: 'Удалено доп. соглашение',
    price_changed: 'Изменена цена',
    status_changed: 'Изменён статус',
    contract_updated: 'Обновлён договор',
  }
  return map[action] || action
}

function formatHistoryDate(dateStr) {
  if (!dateStr) return ''
  return new Date(dateStr).toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function openSupplementaryForm(agreement = null) {
  selectedSupplementary.value = agreement ? { ...agreement } : null
  emit('open-supplementary-form', { contractId: form.id, agreement: selectedSupplementary.value })
}

async function removeSupplementaryAgreement(id) {
  try {
    await saStore.delete(id)
    supplementaryAgreements.value = supplementaryAgreements.value.filter((sa) => sa.id !== id)
    if (form.id) loadHistory(form.id)
    toast.push('Соглашение удалено', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

function onPricelistSelected(pricelistId) {
  if (!pricelistId) {
    // Позицию сняли — цена становится ручной, значение оставляем пользователю.
    isCustomPrice.value = true
    return
  }
  const item = props.pricelistOpt?.find((p) => p.id === pricelistId)
  if (item) {
    const price = parseFloat(item.price)
    form.price = Number.isNaN(price) ? null : price
    isCustomPrice.value = false
  }
}

function formatDate(value) {
  if (!value) return ''
  return new Date(value).toLocaleDateString()
}

function formatPrice(value) {
  if (value === null || value === undefined) return '—'
  const num = parseFloat(value)
  return isNaN(num) ? '—' : num.toLocaleString('ru-RU', { style: 'currency', currency: 'RUB' })
}

function openQuickOrganizationDialog() {
  quickOrg.short_name_with_opf = searchOrganization.value || ''
  quickOrg.inn = ''
  quickOrg.full_name_with_opf = ''
  quickOrg.legal_address = ''
  quickOrg.fact_address = ''
  quickOrgErrors.short_name_with_opf = ''
  quickOrgErrors.inn = ''
  quickOrgDialog.value = true
}

async function saveQuickOrganization() {
  quickOrgErrors.short_name_with_opf = ''
  quickOrgErrors.inn = ''

  if (!quickOrg.short_name_with_opf?.trim()) {
    quickOrgErrors.short_name_with_opf = 'Введите краткое наименование'
  }

  const innValue = String(quickOrg.inn || '').trim()
  if (!/^\d{10,12}$/.test(innValue)) {
    quickOrgErrors.inn = 'ИНН должен содержать 10-12 цифр'
  }

  if (quickOrgErrors.short_name_with_opf || quickOrgErrors.inn) {
    return
  }

  quickOrgSaving.value = true
  try {
    const payload = {
      ...quickOrg,
      short_name_with_opf: quickOrg.short_name_with_opf.trim(),
      inn: Number(innValue),
    }

    const created = await organizationStore.addOrganization(payload)
    form.organization_id = created.id
    searchOrganization.value = created.short_name_with_opf || ''
    quickOrgDialog.value = false
    emit('organization-added', created)
    toast.push('Организация добавлена в справочник', 'success')
  } catch (error) {
    toast.push(error.message || 'Не удалось добавить организацию', 'error')
  } finally {
    quickOrgSaving.value = false
  }
}

async function downloadFile(fileId) {
  try {
    await contractStore.downloadFile(fileId)
  } catch (e) {
    toast.push('Не удалось скачать файл', 'error')
  }
}

async function removeFile(fileId) {
  try {
    await contractStore.deleteFile(fileId)
    files.value = files.value.filter((f) => f.id !== fileId)
    toast.push('Файл удален', 'success')
  } catch (e) {
    toast.push(e.message, 'error')
  }
}

function formatFileSize(bytes) {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

/**
 * Номер и организация обязательны на уровне БД (NOT NULL). Раньше пустые значения
 * молча подменялись на «б/н <дата>» и организацию-заглушку «Не указано» —
 * пользователь не знал, что создал неполный договор.
 */
function validate() {
  errors.number = form.number?.trim() ? '' : 'Укажите номер договора'
  errors.organization_id = form.organization_id ? '' : 'Выберите организацию'
  const yearError = (d) => (d && !isPlausibleDate(d) ? 'Проверьте год даты' : '')
  errors.date_from = yearError(form.date_from)
  errors.date_to = yearError(form.date_to)
  if (!errors.date_from && !errors.date_to && form.date_from && form.date_to) {
    if (dateValueForPicker(form.date_to) < dateValueForPicker(form.date_from)) {
      errors.date_to = 'Дата окончания раньше даты начала'
    }
  }
  return !errors.number && !errors.organization_id && !errors.date_from && !errors.date_to
}

/**
 * Договор с уже занятым номером — не ошибка (продления и дубли встречаются),
 * поэтому только предупреждаем, не блокируя сохранение.
 */
async function checkDuplicateNumber() {
  duplicateNumberHint.value = ''
  const number = form.number?.trim()
  if (!number) return

  try {
    const res = await contractStore.getPaginatedContracts({ search: number, per_page: 10 })
    const clash = (res?.items || []).find((c) => c.number === number && c.id !== form.id)
    if (clash) duplicateNumberHint.value = `Договор с таким номером уже есть (#${clash.id})`
  } catch (e) {
    // Проверка вспомогательная: недоступность поиска не должна мешать вводу.
    console.error('Не удалось проверить номер на дубль', e)
  }
}

function isDirty() {
  return (
    JSON.stringify(form) !== pristineSnapshot.value ||
    pendingFiles.value.length > 0 ||
    pendingUpdFiles.value.length > 0
  )
}

function requestClose() {
  if (isDirty() && !confirm('Закрыть форму? Несохранённые изменения будут потеряны.')) return
  emit('update:modelValue', false)
}

/** Диалог persistent, поэтому закрытие приходит только от кнопок — но страхуемся. */
function onDialogToggle(value) {
  if (value) {
    emit('update:modelValue', true)
    return
  }
  requestClose()
}

function save() {
  if (!validate()) {
    toast.push('Заполните обязательные поля', 'error')
    return
  }

  const payload = {
    ...form,
    number: form.number.trim(),
    date_from: toApiDateTime(form.date_from),
    date_to: toApiDateTime(form.date_to),
    // Сеттеры дат, обработчик срока и заполнение формы держат срок в согласии
    // с датами, поэтому уходит то, что видит пользователь.
    contract_period: normalizePeriod(form.contract_period),
    price: blankToNull(form.price),
  }

  // Диалог закрывает родитель после успешного ответа сервера: при ошибке
  // сохранения форма должна остаться на экране вместе с введёнными данными.
  emit('save', {
    contract: payload,
    pendingFiles: [...pendingFiles.value],
    pendingUpdFiles: [...pendingUpdFiles.value],
  })
}

function deleteItem() {
  if (!confirm('Удалить договор? Вместе с ним удалятся файлы, доп. соглашения и история.')) return
  emit('delete', form.id)
  emit('update:modelValue', false)
}
</script>
