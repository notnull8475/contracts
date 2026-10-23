import dayjs from 'dayjs'

/**
 * Даты договора в форме — строки «YYYY-MM-DD». API отдаёт «YYYY-MM-DDTHH:mm:ss»
 * без зоны; разбираем только календарную часть, без UTC-сдвига.
 */
const YMD_HEAD = /^(\d{4})-(\d{2})-(\d{2})/

/**
 * Полная дата с правдоподобным годом. Поле type="date" в Chrome отдаёт
 * промежуточные значения при наборе года («0002-01-01», «0020-01-01»),
 * и считать по ним ничего нельзя — так в реестр и попадали даты вида «0019-…».
 */
export function isPlausibleDate(value) {
  const m = YMD_HEAD.exec(String(value ?? ''))
  if (!m) return false
  const year = Number(m[1])
  return year >= 1900 && year <= 2200
}

/** Значение для поля type="date": «YYYY-MM-DD» или ''. */
export function dateValueForPicker(value) {
  if (value == null || value === '') return ''
  const s = String(value).trim()
  const head = YMD_HEAD.exec(s)
  if (head) return head[0]
  const d = dayjs(s)
  return d.isValid() ? d.format('YYYY-MM-DD') : ''
}

/** NaiveDateTime в API ожидается как YYYY-MM-DDTHH:mm:ss */
export function toApiDateTime(ymdOrIso) {
  if (ymdOrIso == null || ymdOrIso === '') return null
  const s = String(ymdOrIso).trim()
  if (/^\d{4}-\d{2}-\d{2}$/.test(s)) return `${s}T00:00:00`
  return s
}

function calendar(value) {
  const head = YMD_HEAD.exec(String(value ?? ''))
  return head ? dayjs(head[0]) : null
}

/** Дата + N месяцев; конец месяца подрезается (31.01 + 1 → 28.02). */
export function addMonths(value, months) {
  const d = calendar(value)
  return d ? d.add(months, 'month').format('YYYY-MM-DD') : null
}

/**
 * Срок договора в полных месяцах между датами. dayjs считает так, что
 * addMonths и calculateContractPeriod обратны друг другу и на концах месяцев
 * (28.02 − 31.01 = 1 месяц), иначе срок «усыхал» при каждом открытии карточки.
 */
export function calculateContractPeriod(fromDate, toDate) {
  if (!isPlausibleDate(fromDate) || !isPlausibleDate(toDate)) return null
  const start = calendar(fromDate)
  const end = calendar(toDate)
  if (end.isBefore(start)) return null
  return end.diff(start, 'month')
}
