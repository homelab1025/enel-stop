<script setup lang="ts">

import {DefaultApi, Configuration, type Incident} from "../lib/server";
import {onMounted} from "vue";
import {ref} from "vue";

let response = ref("")
let incidents = ref(<Incident[]>[]);

const configuration = new Configuration();
let server_api = new DefaultApi(configuration);

onMounted(async () => {
  let db_size = await server_api.countIncidents();
  response.value = String(db_size.data.count);

  let all_incidents = await server_api.getAllIncidents();
  incidents.value = all_incidents.data;
})

</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Number of records: {{ response }}</h1>
  <div class="overflow-x-auto">
    <table class="table table-xs">
      <thead>
      <tr>
        <th>Id</th>
        <th>Judet</th>
        <th>Localitate</th>
        <th>Data</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="item in incidents">
        <th>{{ item.id }}</th>
        <td>{{ item.county }}</td>
        <td>{{ item.location }}</td>
        <td>{{ item.datetime }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
</style>
