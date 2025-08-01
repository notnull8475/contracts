import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useToastStore = defineStore('toast', () => {
  const toasts = ref([]) // { id, type, message }

  function push(message, type = 'success', duration = 3000) {
    const id = Date.now() + Math.random()
    toasts.value.push({ id, type, message })

    // гарантированно удаляем
    setTimeout(() => {
      const idx = toasts.value.findIndex((t) => t.id === id)
      if (idx !== -1) toasts.value.splice(idx, 1)
    }, duration)
  }

  return { toasts, push }
})
