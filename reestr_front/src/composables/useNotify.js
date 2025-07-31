import { useToastStore } from '@/store/toast.js'

export function useNotify() {
    const toast = useToastStore()
    const notifySuccess = (msg, ms = 3000) => toast.push(msg, 'success', ms)
    const notifyError   = (msg, ms = 3000) => toast.push(msg, 'error',   ms)
    return { notifySuccess, notifyError }
}
