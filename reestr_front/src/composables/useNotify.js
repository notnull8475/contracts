import { useToastStore } from '@/store/toast.js'

/**
 * Второй аргумент — уточнение к сообщению, а не длительность:
 * вызывающий код повсеместно передаёт пару «заголовок, подробности».
 */
function joinMessage(message, detail) {
  return detail ? `${message}: ${detail}` : message
}

export function useNotify() {
  const toast = useToastStore()
  const notifySuccess = (msg, detail) => toast.push(joinMessage(msg, detail), 'success')
  const notifyError = (msg, detail) => toast.push(joinMessage(msg, detail), 'error')
  return { notifySuccess, notifyError }
}
