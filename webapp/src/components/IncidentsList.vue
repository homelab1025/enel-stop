<script setup lang="ts">

import {Configuration, DefaultApi, type Incident} from "../lib/server";
import {onMounted, ref} from "vue";

let total_count = ref("")
let incidents = ref(<Incident[]>[]);
let searchCounty = ref("");
let currentPage = ref(1);

const configuration = new Configuration();
let server_api = new DefaultApi(configuration);

onMounted(async () => {
  let db_size = await server_api.countIncidents();
  total_count.value = String(db_size.data.total_count);

  let all_incidents = await server_api.getAllIncidents();
  incidents.value = all_incidents.data.incidents;
})

const handleCountySearch = () => {
  updateIncidentsTable();
}

async function updateIncidentsTable() {
  let county = searchCounty.value.trim().toUpperCase() || null;
  let incidentsSearch = await server_api.getAllIncidents(county, 50 * (currentPage.value - 1));
  incidents.value = incidentsSearch.data.incidents;
}

const previousPage = async () => {
  if (currentPage.value > 1) {
    currentPage.value = currentPage.value - 1;
  }

  await updateIncidentsTable();
}

const nextPage = async () => {
  currentPage.value = currentPage.value + 1;
  await updateIncidentsTable();
}

</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight pt-4">Total number of incidents: {{ total_count }}</h1>
  <div class="card grid h-10">
    <label class="input">
      Judet
      <input type="text" class="grow" placeholder="" v-model="searchCounty" @keyup.enter="handleCountySearch"/>
    </label>
  </div>

  <div class="join flex justify-center">
    <button class="join-item btn btn-sm" @mouseup="previousPage">«</button>
    <button class="join-item btn btn-sm">Page {{ currentPage }}</button>
    <button class="join-item btn btn-sm" @mouseup="nextPage">»</button>
  </div>
  <div class="divider"/>

  <div class="card">
    <table class="table table-fixed table-pin-rows">
      <thead>
      <tr>
        <th>Id</th>
        <th>Judet</th>
        <th>Localitate</th>
        <th>Data</th>
        <th>Descriere</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="item in incidents">
        <td>{{ item.id }}</td>
        <td>{{ item.county }}</td>
        <td>{{ item.location }}</td>
        <td>{{ item.day }}</td>
        <td>{{ item.description }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
</style>
