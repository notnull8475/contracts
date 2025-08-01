<template>
  <router-view />
  <ToastContainer />
</template>

<script>
import ToastContainer from '@/components/ToastContainer.vue'
export default {
  name: 'App',
  components: { ToastContainer },
}
</script>
