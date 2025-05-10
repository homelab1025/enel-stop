<script setup>
import {RouterLink, RouterView} from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <div class="flex h-[500px]">
    <div class="w-[200px] bg-blue-300 p-4">
      <ul class="flex">
        <RouterLink class="mr-3" to="/">Home</RouterLink>
        <RouterLink class="mr-3" to="/about">About</RouterLink>
        <RouterLink class="mr-3" to="/count">Counter</RouterLink>
      </ul>
    </div>
    <div class="flex-1 bg-blue-100 p-4">
      <RouterView/>
    </div>
  </div>

</template>

<style scoped>

</style>
