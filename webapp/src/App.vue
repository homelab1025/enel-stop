<script setup lang="ts">
import {RouterLink, RouterView} from 'vue-router'
import ThemeSwitcher from "./components/ThemeSwitcher.vue";
</script>

<template>
  <div class="flex h-screen">
    <ul class="menu rounded-box w-56 p-2">
      <li>
        <RouterLink to="/">Home</RouterLink>
      </li>
      <li>
        <RouterLink to="/list">Lister</RouterLink>
      </li>
      <li>
        <RouterLink to="/map">Map</RouterLink>
      </li>
      <li class="mt-auto"><ThemeSwitcher/></li>
    </ul>
    <div class="flex-1 p-4 overflow-auto">
      <RouterView/>
    </div>
  </div>
</template>

<style>
</style>
