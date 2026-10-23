import { describe, it, expect, beforeAll, vi } from 'vitest'
import { DOMWrapper, mount } from '@vue/test-utils'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { createPinia, setActivePinia } from 'pinia'
import ContractForm from '@/components/forms/ContractForm.vue'

// Vuetify рассчитывает размеры через ResizeObserver и matchMedia, в jsdom их нет.
beforeAll(() => {
  global.ResizeObserver = class {
    observe() {}
    unobserve() {}
    disconnect() {}
  }
  global.visualViewport = { addEventListener() {}, removeEventListener() {} }
  // В этой сборке Node нет localStorage; без заглушки перехватчик axios падает в logout().
  if (!global.localStorage) {
    const store = new Map()
    global.localStorage = {
      getItem: (k) => (store.has(k) ? store.get(k) : null),
      setItem: (k, v) => store.set(k, String(v)),
      removeItem: (k) => store.delete(k),
      clear: () => store.clear(),
    }
    window.localStorage = global.localStorage
  }
  window.matchMedia = () => ({
    matches: false,
    addEventListener() {},
    removeEventListener() {},
    addListener() {},
    removeListener() {},
  })
})

const vuetify = createVuetify({ components, directives })

const ORGANIZATIONS = [
  {
    id: 7,
    short_name_with_opf: 'ООО «Ромашка»',
    full_name_with_opf: 'ООО «Ромашка»',
    inn: 7701234567,
  },
]

/**
 * Поле «Срок действия (мес.)» — как его видит пользователь. Диалог рендерится
 * в body, а не внутри wrapper; берём последнее совпадение — текущий диалог.
 */
function periodInput() {
  const field = [...document.querySelectorAll('.v-field')]
    .filter((f) => f.textContent.includes('Срок действия'))
    .at(-1)
  return new DOMWrapper(field.querySelector('input'))
}

/** Одна клавиша в поле срока: только input, без фиксации значения. */
async function typePeriod(wrapper, value) {
  const input = periodInput()
  input.element.value = value
  await input.trigger('input')
  await wrapper.vm.$nextTick()
}

/** Ввод срока с фиксацией значения: setValue в test-utils шлёт input и change. */
async function enterPeriod(wrapper, value) {
  await periodInput().setValue(value)
  await wrapper.vm.$nextTick()
}

function mountForm(props = {}) {
  setActivePinia(createPinia())
  return mount(ContractForm, {
    global: { plugins: [vuetify] },
    attachTo: document.body,
    props: {
      modelValue: true,
      contract: null,
      organizationsOpt: ORGANIZATIONS,
      organizationsRaw: ORGANIZATIONS,
      respPersonsOpt: [],
      validityTypesOpt: [],
      statusesOpt: [],
      pricelistOpt: [],
      ...props,
    },
  })
}

describe('ContractForm — обязательные поля', () => {
  it('не сохраняет договор без номера и организации', async () => {
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    wrapper.vm.save()
    await wrapper.vm.$nextTick()

    expect(wrapper.emitted('save')).toBeUndefined()
    expect(document.body.textContent).toContain('Укажите номер договора')
    expect(document.body.textContent).toContain('Выберите организацию')
    wrapper.unmount()
  })

  it('сохраняет, когда номер и организация заполнены', async () => {
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    wrapper.vm.form.number = 'Д-2026/001'
    wrapper.vm.form.organization_id = 7
    await wrapper.vm.$nextTick()

    wrapper.vm.save()
    await wrapper.vm.$nextTick()

    const saved = wrapper.emitted('save')
    expect(saved).toHaveLength(1)
    expect(saved[0][0].contract.number).toBe('Д-2026/001')
    expect(saved[0][0].contract.organization_id).toBe(7)
    // Диалог закрывает родитель после ответа сервера, форма сама не закрывается.
    expect(wrapper.emitted('update:modelValue')).toBeUndefined()
    wrapper.unmount()
  })

  it('обрезает пробелы в номере', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = '   Д-2026/002   '
    wrapper.vm.form.organization_id = 7
    await wrapper.vm.$nextTick()

    wrapper.vm.save()
    expect(wrapper.emitted('save')[0][0].contract.number).toBe('Д-2026/002')
    wrapper.unmount()
  })
})

describe('ContractForm — организация, которой нет в справочнике', () => {
  it('показывает кнопку быстрого добавления организации', async () => {
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    // Пользователь набрал название, которого нет в справочнике.
    wrapper.vm.searchOrganization = 'ЗАО Неизвестное'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.orgAutocompleteItems).toHaveLength(0)

    // Меню автокомплита рендерится в overlay вне корня компонента.
    const autocomplete = wrapper.findComponent({ name: 'VAutocomplete' })
    autocomplete.vm.menu = true
    await new Promise((resolve) => setTimeout(resolve, 0))
    await wrapper.vm.$nextTick()

    expect(document.body.textContent).toContain('Добавить новую организацию')
    wrapper.unmount()
  })
})

describe('ContractForm — защита введённых данных', () => {
  it('нетронутая форма закрывается без подтверждения', async () => {
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(true)
    wrapper.vm.requestClose()

    expect(confirmSpy).not.toHaveBeenCalled()
    expect(wrapper.emitted('update:modelValue')[0]).toEqual([false])
    confirmSpy.mockRestore()
    wrapper.unmount()
  })

  it('открытый из списка договор с датами и сроком не считается изменённым', async () => {
    // Регресс: watcher по сроку переписывал date_to из «…T00:00:00» в «YYYY-MM-DD»
    // уже после снимка формы, и «Отмена» спрашивала про несохранённые изменения.
    const wrapper = mountForm({
      contract: {
        id: 1859,
        number: 'б/н 2026-04-04',
        organization_id: 7,
        date_from: '2026-04-28T00:00:00',
        date_to: '2028-03-28T00:00:00',
        contract_period: 23,
      },
    })
    await wrapper.vm.$nextTick()
    await wrapper.vm.$nextTick()

    const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(true)
    wrapper.vm.requestClose()

    expect(confirmSpy).not.toHaveBeenCalled()
    expect(wrapper.emitted('update:modelValue')[0]).toEqual([false])
    confirmSpy.mockRestore()
    wrapper.unmount()
  })

  it('срок из БД не пересчитывает дату окончания при открытии договора', async () => {
    // Регресс: watcher по сроку срабатывал на заполнение формы и заменял
    // фактическую дату окончания (13.08.2025) на date_from + 88 мес. (23.07.2025).
    const wrapper = mountForm({
      contract: {
        id: 486,
        number: '24/18Г',
        organization_id: 7,
        date_from: '2018-03-23T00:00:00',
        date_to: '2025-08-13T00:00:00',
        contract_period: 88,
      },
    })
    await wrapper.vm.$nextTick()
    await wrapper.vm.$nextTick()

    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    expect(wrapper.vm.form.contract_period).toBe(88)

    const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(true)
    wrapper.vm.requestClose()
    expect(confirmSpy).not.toHaveBeenCalled()
    confirmSpy.mockRestore()

    // А ввод срока пользователем по-прежнему пересчитывает дату окончания.
    await enterPeriod(wrapper, '12')
    expect(wrapper.vm.form.date_to).toBe('2019-03-23')
    wrapper.unmount()
  })

  it('стирание срока по одной цифре не переписывает дату окончания', async () => {
    // Регресс: пересчёт на каждую клавишу — «88» → «8» ставил 23.11.2018,
    // а пустое поле уже ничего не возвращало; «Сохранить» уносил чужую дату.
    const wrapper = mountForm({
      contract: {
        id: 486,
        number: '24/18Г',
        organization_id: 7,
        date_from: '2018-03-23T00:00:00',
        date_to: '2025-08-13T00:00:00',
        contract_period: 88,
      },
    })
    await wrapper.vm.$nextTick()

    await typePeriod(wrapper, '8') // промежуточное значение, без фиксации
    expect(wrapper.vm.form.contract_period).toBe(8)
    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    // Пустой срок при двух датах возвращается к сроку по датам, дата не трогается.
    await enterPeriod(wrapper, '')
    expect(wrapper.vm.form.contract_period).toBe(88)
    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    await enterPeriod(wrapper, '0')
    expect(wrapper.vm.form.contract_period).toBe(88)
    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    wrapper.unmount()
  })

  it('повторный ввод того же срока не делает форму изменённой', async () => {
    const wrapper = mountForm({
      contract: {
        id: 1859,
        number: 'б/н 2026-04-04',
        organization_id: 7,
        date_from: '2026-04-28T00:00:00',
        date_to: '2028-03-28T00:00:00',
        contract_period: 23,
      },
    })
    await wrapper.vm.$nextTick()
    await enterPeriod(wrapper, '23')

    const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(true)
    wrapper.vm.requestClose()
    expect(confirmSpy).not.toHaveBeenCalled()
    confirmSpy.mockRestore()
    wrapper.unmount()
  })

  it('введённая дата окончания не заменяется на «начало + срок»', async () => {
    // Регресс: ввод даты окончания пересчитывал срок (88), а watcher по сроку
    // тут же переписывал дату на 23.07.2025 — ввести 13.08.2025 было нельзя.
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    wrapper.vm.formattedDateFrom = '2018-03-23'
    await wrapper.vm.$nextTick()
    wrapper.vm.formattedDateTo = '2025-08-13'
    await wrapper.vm.$nextTick()
    await wrapper.vm.$nextTick()

    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    expect(wrapper.vm.form.contract_period).toBe(88)

    // Сдвиг даты начала пересчитывает срок, но дату окончания не трогает.
    wrapper.vm.formattedDateFrom = '2018-01-01'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    expect(wrapper.vm.form.contract_period).toBe(91)
    wrapper.unmount()
  })

  it('срок, введённый без даты начала, уходит на сервер и даёт дату после выбора начала', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = 'Д-2026/003'
    wrapper.vm.form.organization_id = 7
    await wrapper.vm.$nextTick()
    await enterPeriod(wrapper, '12')

    wrapper.vm.save()
    const payload = wrapper.emitted('save')[0][0].contract
    expect(payload.contract_period).toBe(12)
    expect(payload.date_to).toBeNull()

    // Дата начала появилась позже срока — срок не стирается, а даёт дату окончания.
    wrapper.vm.formattedDateFrom = '2026-01-01'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.contract_period).toBe(12)
    expect(wrapper.vm.form.date_to).toBe('2027-01-01')
    wrapper.unmount()
  })

  it('набор года с клавиатуры не создаёт дату окончания и не стирает срок', async () => {
    // Chrome отдаёт промежуточные «0002-01-01», «0020-01-01» при наборе года.
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()
    await enterPeriod(wrapper, '12')

    wrapper.vm.formattedDateFrom = '0002-01-01'
    await wrapper.vm.$nextTick()
    wrapper.vm.formattedDateFrom = '0020-01-01'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.date_to).toBeNull()
    expect(wrapper.vm.form.contract_period).toBe(12)

    wrapper.vm.formattedDateFrom = '2026-01-01'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.date_to).toBe('2027-01-01')
    expect(wrapper.vm.form.contract_period).toBe(12)
    wrapper.unmount()
  })

  it('дата с неправдоподобным годом не сохраняется', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = 'Д-2026/005'
    wrapper.vm.form.organization_id = 7
    wrapper.vm.formattedDateFrom = '0019-08-07'
    await wrapper.vm.$nextTick()

    wrapper.vm.save()
    await wrapper.vm.$nextTick()
    expect(wrapper.emitted('save')).toBeUndefined()
    expect(document.body.textContent).toContain('Проверьте год даты')
    wrapper.unmount()
  })

  it('срок из БД показывается как есть, очищенная дата начала его обнуляет', async () => {
    const wrapper = mountForm({
      contract: {
        id: 486,
        number: '24/18Г',
        organization_id: 7,
        date_from: '2018-03-23T00:00:00',
        date_to: '2025-08-13T00:00:00',
        contract_period: 90, // в Excel было не то, что по датам — как в списке
      },
    })
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.contract_period).toBe(90)

    wrapper.vm.formattedDateFrom = ''
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.contract_period).toBeNull()
    expect(wrapper.vm.form.date_to).toBe('2025-08-13')
    wrapper.unmount()
  })

  it('нулевой срок без даты окончания сохраняется, с датой окончания — считается по датам', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = 'Д-2026/006'
    wrapper.vm.form.organization_id = 7
    wrapper.vm.formattedDateFrom = '2026-01-01'
    await wrapper.vm.$nextTick()

    await enterPeriod(wrapper, '0')
    expect(wrapper.vm.form.contract_period).toBe(0)
    expect(wrapper.vm.form.date_to).toBeNull()
    wrapper.vm.save()
    expect(wrapper.emitted('save')[0][0].contract.contract_period).toBe(0)

    wrapper.vm.formattedDateTo = '2027-01-01'
    await wrapper.vm.$nextTick()
    await enterPeriod(wrapper, '0')
    expect(wrapper.vm.form.contract_period).toBe(12)
    wrapper.unmount()
  })

  it('дата окончания раньше начала не сохраняется, ошибка снимается после правки', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = 'Д-2026/007'
    wrapper.vm.form.organization_id = 7
    wrapper.vm.formattedDateFrom = '2026-05-01'
    wrapper.vm.formattedDateTo = '2026-04-01'
    await wrapper.vm.$nextTick()

    wrapper.vm.save()
    await wrapper.vm.$nextTick()
    expect(wrapper.emitted('save')).toBeUndefined()
    expect(document.body.textContent).toContain('Дата окончания раньше даты начала')

    wrapper.vm.formattedDateTo = '2026-06-01'
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.errors.date_to).toBe('')
    wrapper.unmount()
  })

  it('дробный и отрицательный срок не уходят на сервер как есть', async () => {
    const wrapper = mountForm()
    wrapper.vm.form.number = 'Д-2026/004'
    wrapper.vm.form.organization_id = 7
    await wrapper.vm.$nextTick()

    await enterPeriod(wrapper, '1.5')
    expect(wrapper.vm.form.contract_period).toBe(2)
    await enterPeriod(wrapper, '-3')
    expect(wrapper.vm.form.contract_period).toBeNull()

    wrapper.vm.save()
    expect(wrapper.emitted('save')[0][0].contract.contract_period).toBeNull()
    wrapper.unmount()
  })

  it('заполненная форма спрашивает подтверждение и не закрывается при отказе', async () => {
    const wrapper = mountForm()
    await wrapper.vm.$nextTick()

    wrapper.vm.form.number = 'Д-2026/003'
    await wrapper.vm.$nextTick()

    const confirmSpy = vi.spyOn(window, 'confirm').mockReturnValue(false)
    wrapper.vm.requestClose()

    expect(confirmSpy).toHaveBeenCalled()
    expect(wrapper.emitted('update:modelValue')).toBeUndefined()
    confirmSpy.mockRestore()
    wrapper.unmount()
  })
})

describe('ContractForm — состояние между договорами', () => {
  it('не переносит поля предыдущего договора в новый', async () => {
    const wrapper = mountForm({
      contract: {
        id: 42,
        number: 'Д-2025/999',
        organization_id: 7,
        address: 'ул. Ленина, 1',
        comment: 'важный комментарий',
        additional_agreement: 'ДС-1',
      },
    })
    await wrapper.vm.$nextTick()
    expect(wrapper.vm.form.address).toBe('ул. Ленина, 1')

    await wrapper.setProps({ contract: null })
    await wrapper.vm.$nextTick()

    expect(wrapper.vm.form.id).toBeNull()
    expect(wrapper.vm.form.address).toBe('')
    expect(wrapper.vm.form.comment).toBe('')
    expect(wrapper.vm.form.additional_agreement).toBe('')
    wrapper.unmount()
  })
})
