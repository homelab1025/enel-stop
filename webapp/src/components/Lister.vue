<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)

</script>

<template>
  <h1>{{ msg }} - {{count}}</h1>

  LISTER
</template>

<style scoped>
</style>
