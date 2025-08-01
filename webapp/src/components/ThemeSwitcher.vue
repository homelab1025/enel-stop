<template>
  <div class="theme-switcher">
    <select v-model="selectedTheme" class="select select-bordered w-full max-w-xs">
      <option disabled selected>Select Theme</option>
      <option v-for="theme in themes" :key="theme" :value="theme">
        {{ theme }}
      </option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

// Define the available themes. This array should match the themes you enabled in your CSS.
const themes = ['light', 'wireframe'];

// Get the initial theme from local storage or default to the first theme
const storedTheme = localStorage.getItem('theme') || themes[0];
const selectedTheme = ref(storedTheme);

// Watch for changes to selectedTheme and update the HTML attribute and local storage
watch(selectedTheme, (newTheme) => {
  document.documentElement.setAttribute('data-theme', newTheme);
  localStorage.setItem('theme', newTheme);
});

// Set the initial theme on component mount
document.documentElement.setAttribute('data-theme', storedTheme);
</script>