import { describe, it, expect, beforeAll, vi } from 'vitest'
import { mount } from '@vue/test-utils'
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
