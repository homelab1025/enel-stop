<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {Configuration, DefaultApi} from "../lib/server";

let response = ref("")
const configuration = new Configuration();
let server_api = new DefaultApi(configuration);


onMounted(async () => {
  let {status, data} = await server_api.ping();
  response.value = String(data.ping);
})

</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">HOME: {{ response }}</h1>
</template>

<style scoped>
</style>
