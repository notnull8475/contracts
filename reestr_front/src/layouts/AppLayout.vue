<template>
  <v-app>
    <Header />

    <v-main class="registry-main">
      <v-container class="py-6 px-4 px-md-6" fluid>
        <router-view />
      </v-container>
    </v-main>

    <Footer />
  </v-app>
</template>
<script setup lang="ts">
import Header from '@/components/Header.vue'
import Footer from '@/components/Footer.vue'
</script>

<style scoped>
.registry-main {
  background: linear-gradient(180deg, #f4f8fb 0%, #eef3f7 60%, #e9eff4 100%);
  min-height: calc(100vh - 64px);
}
</style>
