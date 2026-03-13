<template>
  <transition-group
    name="toast-fade"
    tag="div"
    class="fixed right-5 top-20 flex flex-col items-end gap-3 z-[9999]"
  >
    <div
      v-for="t in toasts"
      :key="t.id"
      class="rounded-lg shadow-lg px-5 py-3 text-white flex items-center gap-2 min-w-[280px]"
      :class="t.type === 'success' ? 'bg-[#2a9d8f]' : 'bg-[#c0392b]'"
    >
      <v-icon size="20">
        {{ t.type === 'success' ? 'mdi-check-circle' : 'mdi-alert-circle' }}
      </v-icon>
      <span>{{ t.message }}</span>
    </div>
  </transition-group>
</template>

<script setup>
import { useToastStore } from '@/store/toast.js'
const { toasts } = useToastStore()
</script>

<style scoped>
.toast-fade-enter-from,
.toast-fade-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
.toast-fade-enter-active,
.toast-fade-leave-active {
  transition: all 200ms ease;
}
</style>
