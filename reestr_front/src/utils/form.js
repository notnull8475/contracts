/** Очищенное поле type="number" даёт '' — бэкенд ждёт число или null. */
export function blankToNull(value) {
  return value === '' || value == null ? null : value
}

/**
 * Срок в месяцах: целое неотрицательное число или null (бэкенд ждёт i32 >= 0).
 * Дробное округляется, отрицательное и мусор считаются пустым полем.
 */
export function normalizePeriod(value) {
  const raw = blankToNull(value)
  if (raw === null) return null
  const n = Math.round(Number(raw))
  if (!Number.isFinite(n) || n < 0) return null
  return n || 0 // -0 → 0
}

/**
 * Полная дата «YYYY-MM-DD» с правдоподобным годом. Поле type="date" в Chrome
 * отдаёт промежуточные значения при наборе года («0002-01-01», «0020-01-01»),
 * и считать по ним ничего нельзя — так в реестр и попадали даты вида «0019-…».
 */
export function isPlausibleDate(value) {
  const m = /^(\d{4})-\d{2}-\d{2}/.exec(String(value ?? ''))
  if (!m) return false
  const year = Number(m[1])
  return year >= 1900 && year <= 2200
}
