<script setup lang="ts">
import {RouterLink, RouterView} from 'vue-router'
</script>

<template>
  <div class="flex h-screen">
    <ul class="menu bg-base-200 rounded-box w-48">
      <li>
        <RouterLink to="/">Home</RouterLink>
      </li>
      <li>
        <RouterLink to="/list">Lister</RouterLink>
      </li>
    </ul>
    <div class="flex-1 bg-blue-100 p-4">
      <RouterView/>
    </div>
  </div>
</template>

<style>
</style>
