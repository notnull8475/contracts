<template>
  <div class="not-found">
    <h1 class="title">404 - Страница не найдена</h1>
    <p class="message">Упс! Страница, которую вы искали, не найдена.</p>
    <p class="sub-message">Возможно, она была удалена, изменила название или временно недоступна.</p>
    <router-link to="/" class="home-button">Вернуться на главную страницу</router-link>
  </div>
</template>

<script>
export default {
  name: 'NotFound',
}
</script>

<style scoped>
.not-found {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  text-align: center;
  font-family: Arial, sans-serif;
}

.title {
  font-size: 2.5rem;
  color: #333;
  margin-bottom: 1rem;
}

.not-found-image {
  max-width: 300px;
  margin: 2rem 0;
}

.message {
  font-size: 1.2rem;
  color: #666;
  margin-bottom: 0.5rem;
}

.sub-message {
  font-size: 1rem;
  color: #888;
  margin-bottom: 2rem;
}

.home-button {
  display: inline-block;
  padding: 10px 20px;
  background-color: #4CAF50;
  color: white;
  text-decoration: none;
  border-radius: 5px;
  font-size: 1rem;
  transition: background-color 0.3s;
}

.home-button:hover {
  background-color: #45a049;
}
</style>