<script setup>
import {ref, onMounted} from 'vue';

const status_number = ref(null);

onMounted(async() =>{
  const response = await fetch("/aaa");
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  status_number.value = response.status;
})
</script>

<template>
  <div class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Just a friendly welcome: {{ status_number }}</div>

</template>
