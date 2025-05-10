
<script setup>
import {ref, onMounted} from 'vue';

const incidents = ref(null);
const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;

onMounted(async() =>{
  const apiUrl = `${apiBaseUrl}/incidents/count`;
  console.log(apiUrl);
  const response = await fetch(apiUrl);
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  incidents.value = await response.text();
})
</script>

<template>
  <div class="about">
    <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Right now there are {{incidents}} incidents in the database.</h1>
  </div>
</template>

<style>

</style>
