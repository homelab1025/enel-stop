<script setup>
import {ref, onMounted} from 'vue';

const status_number = ref(null);
const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;

onMounted(async() =>{
  const apiUrl = `${apiBaseUrl}/ping`;
  console.log(apiUrl);
  const response = await fetch(apiUrl);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  status_number.value = response.status;
})
</script>

<template>
  <div class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Just a friendly welcome: {{ status_number }}</div>

</template>
