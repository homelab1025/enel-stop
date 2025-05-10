<script setup>
import {RouterLink, RouterView} from 'vue-router'
</script>

<template>
  <div class="flex h-screen">
    <ul class="menu bg-base-200 rounded-box w-56">
      <li><a>
        <RouterLink to="/">Home</RouterLink>
      </a></li>
      <li><a>
        <RouterLink to="/count">Counter</RouterLink>
      </a></li>
      <li><a>
        <RouterLink to="/about">About</RouterLink>
      </a></li>
    </ul>
    <div class="flex-1 bg-blue-100 p-4">
      <RouterView/>
    </div>
  </div>

</template>

<style scoped>

</style>
