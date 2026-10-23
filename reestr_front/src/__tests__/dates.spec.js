import { describe, it, expect } from 'vitest'
import {
  addMonths,
  calculateContractPeriod,
  dateValueForPicker,
  isPlausibleDate,
  toApiDateTime,
} from '@/utils/dates.js'

describe('арифметика дат договора', () => {
  it('срок и дата окончания обратны друг другу на концах месяцев', () => {
    // Раньше 31.01 + 1 мес. = 28.02, а обратно выходило 0 — срок «усыхал» при каждом открытии.
    for (const [from, months] of [
      ['2026-01-31', 1],
      ['2026-03-31', 1],
      ['2026-08-31', 6],
      ['2024-02-29', 12],
      ['2018-03-23', 88],
      ['2026-04-28', 23],
    ]) {
      const to = addMonths(from, months)
      expect(calculateContractPeriod(from, to)).toBe(months)
    }
    expect(addMonths('2026-01-31', 1)).toBe('2026-02-28')
  })

  it('считает полные месяцы и не считает по мусору', () => {
    expect(calculateContractPeriod('2018-03-23T00:00:00', '2025-08-13T00:00:00')).toBe(88)
    expect(calculateContractPeriod('2026-05-01', '2026-04-01')).toBeNull()
    expect(calculateContractPeriod('0020-03-23', '2026-04-01')).toBeNull()
    expect(calculateContractPeriod(null, '2026-04-01')).toBeNull()
  })

  it('правдоподобный год и форматы для поля и API', () => {
    expect(isPlausibleDate('2026-04-01')).toBe(true)
    expect(isPlausibleDate('0019-08-07')).toBe(false)
    expect(isPlausibleDate('')).toBe(false)
    expect(dateValueForPicker('2028-03-28T00:00:00')).toBe('2028-03-28')
    expect(dateValueForPicker(null)).toBe('')
    expect(toApiDateTime('2028-03-28')).toBe('2028-03-28T00:00:00')
    expect(toApiDateTime('')).toBeNull()
  })
})
