<script setup lang="ts">

import {DefaultApi, Configuration, type Incident} from "../lib/server";
import {onMounted} from "vue";
import {ref} from "vue";

let response = ref("")
let incidents = ref(<Incident[]>[]);
let searchCounty = ref("");
let error = ref<string | null>(null);

const configuration = new Configuration();
let server_api = new DefaultApi(configuration);

onMounted(async () => {
  let db_size = await server_api.countIncidents();
  response.value = String(db_size.data.total_count);

  let all_incidents = await server_api.getAllIncidents();
  incidents.value = all_incidents.data.incidents;
})

const handleCountySearch = () => {
  // Clear any previous error message
  error.value = null;

  // Check if the search county input is not empty after trimming whitespace
  if (searchCounty.value.trim()) {
    searchIncidentsByCounty(searchCounty.value.trim());
  } else {
    // If input is empty, show all incidents again
    error.value = "County name cannot be empty. Displaying all incidents.";
    searchIncidentsByCounty();
  }
}

const searchIncidentsByCounty = async (countyName?: string) => {
  const incidentsByCounty = await server_api.getAllIncidents(countyName);
  incidents.value = incidentsByCounty.data.incidents;
  response.value = String(incidentsByCounty.data.total_count); // Update total count for filtered results
};

</script>

<template>
  <h1 class="text-4xl font-semibold tracking-tight text-gray-600 pt-4">Total number of incidents: {{ response }}</h1>
  <div class="card grid h-10">
    <label class="input">
      Judet
      <input type="text" class="grow" placeholder="" v-model="searchCounty" @keyup.enter="handleCountySearch"/>
    </label></div>
  <div class="divider"></div>

<!--  <div class="join">-->
<!--    <input-->
<!--        class="join-item btn btn-square"-->
<!--        type="radio"-->
<!--        name="options"-->
<!--        aria-label="1"-->
<!--    />-->
<!--    <input class="join-item btn btn-square" type="radio" name="options" aria-label="2" />-->
<!--    <input class="join-item btn btn-square" type="radio" name="options" aria-label="3" />-->
<!--    <input class="join-item btn btn-square" type="radio" name="options" aria-label="4" />-->
<!--  </div>-->

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
        <td>{{ item.datetime }}</td>
        <td>{{ item.description }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
</style>
