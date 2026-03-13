<template>
  <div class="not-found-wrap">
    <v-card rounded="xl" elevation="4" class="not-found-card pa-6 text-center">
      <v-icon size="72" color="warning">mdi-alert-circle-outline</v-icon>
      <h1 class="text-h4 font-weight-bold mt-3 mb-2">404 — страница не найдена</h1>
      <p class="text-body-1 text-medium-emphasis mb-6">
        Возможно, ссылка устарела или страница была перемещена.
      </p>
      <div class="d-flex justify-center ga-2">
        <v-btn color="primary" to="/contracts" prepend-icon="mdi-file-document-multiple">
          К договорам
        </v-btn>
        <v-btn variant="tonal" to="/about" prepend-icon="mdi-home-outline">На главную</v-btn>
      </div>
    </v-card>
  </div>
</template>

<style scoped>
.not-found-wrap {
  min-height: calc(100vh - 140px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.not-found-card {
  width: 100%;
  max-width: 620px;
}
</style>
