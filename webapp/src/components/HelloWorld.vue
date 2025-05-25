<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">HOME: {{ msg }} - {{ count }}</h1>
</template>

<style scoped>
</style>
