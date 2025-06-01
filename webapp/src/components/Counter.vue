<script setup lang="ts">

import {DefaultApi, Configuration} from "../lib/server";
import {onMounted} from "vue";
import {ref} from "vue";

let response = ref("")
const configuration = new Configuration();
let server_api = new DefaultApi(configuration);


onMounted(async () => {
  let {status, data} = await server_api.countIncidents();
  response.value = String(data.count);
})

</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Number of records: {{ response }}</h1>
</template>

<style scoped>
</style>
