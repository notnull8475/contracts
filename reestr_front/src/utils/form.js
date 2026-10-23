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
