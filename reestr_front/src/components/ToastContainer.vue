<template>
  <transition-group
      name="toast-fade"
      tag="div"
      class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center gap-4 z-[9999]">
    <div
        v-for="t in toasts"
        :key="t.id"
        class="rounded-md shadow-lg px-6 py-3 text-white flex items-center gap-2"
        :class="t.type === 'success' ? 'bg-green-600' : 'bg-red-600'"
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
