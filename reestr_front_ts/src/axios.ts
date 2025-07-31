import axios, { AxiosInstance, AxiosResponse, AxiosError } from 'axios';
import { useAuthStore } from '@/stores/auth.js';

// Создаем тип для конфигурации API клиента
const apiClient: AxiosInstance = axios.create({
  baseURL: 'http://localhost:8080', // URL вашего бэкенда
  // baseURL: import.meta.env.VITE_API_BASE_URL,
  // baseURL: 'https://image.alchemyidea.ru',
  headers: {
    'Content-Type': 'application/json'
  }
});

// Глобальный обработчик ошибок
apiClient.interceptors.response.use(
  (response: AxiosResponse) => response, // Указываем тип AxiosResponse
  (error: AxiosError) => {
    if (error.response && error.response.status === 401) {
      const authStore = useAuthStore();
      authStore.logout(); // Разлогиниваем пользователя
      window.location.href = '/login'; // Перенаправляем на страницу входа
    }
    return Promise.reject(error);
  }
);

export default apiClient;