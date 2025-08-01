<template>
  <v-app>
    <Header />

    <!-- Динамическое содержимое страниц -->
    <v-main class="bg-gray-100">
      <v-container>
        <router-view />
      </v-container>
    </v-main>

    <!-- Ваш Footer -->
    <Footer />
  </v-app>
</template>
<script setup lang="ts">
import Header from '@/components/Header.vue'
import Footer from '@/components/Footer.vue'
</script>
