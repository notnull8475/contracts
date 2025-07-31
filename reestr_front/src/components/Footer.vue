<template>
  <!--  <v-footer color="surface-light" name="footer" app>-->
  <!--    <v-btn class="mx-auto" text="Get data" variant="text" @click="print('footer')"></v-btn>-->
  <!--  </v-footer>-->
  <v-footer color="#0F7CB7FF" dark padless>
    <v-container>
      <!--      <v-row>-->
      <!--        <v-col cols="12" md="6">-->
      <!--          <h3 class="headline mb-4">ООО "Инфо-Оберег"</h3>-->
      <!--        </v-col>-->
      <!--        <v-col cols="12" md="6" class="text-md-right">-->
      <!--          <v-img-->
      <!--            src="/logo.png"-->
      <!--            contain-->
      <!--            height="80"-->
      <!--            max-width="200"-->
      <!--            class="ml-auto"-->
      <!--          ></v-img>-->
      <!--        </v-col>-->
      <!--      </v-row>-->
      <!--      <v-divider class="my-4"></v-divider>-->
      <div class="text-center">
        &copy; 2017 — {{ new Date().getFullYear() }}
      </div>
    </v-container>
  </v-footer>
</template>
<script setup lang="ts">
</script>