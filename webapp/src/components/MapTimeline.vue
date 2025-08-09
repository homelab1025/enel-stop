<script setup lang="ts">
import {onMounted, ref, watch, computed} from 'vue'
import {Configuration, DefaultApi, type Incident} from "../lib/server";
import { normalizeCountyName, COUNTY_COORDS } from "../data/counties";

import * as Leaflet from 'leaflet'
import 'leaflet/dist/leaflet.css'

const configuration = new Configuration();
const server_api = new DefaultApi(configuration);

const daysBack = 60; // timeline covers the last 60 days, ending at latest known incident day

// Dynamic dates array built from latest incident end_date
const dates = ref<Date[]>([]);

const sliderIndex = ref(0);
// Only commit the selected index when the user releases the slider (change event)
const committedIndex = ref(0);
const selectedDate = computed(() => {
  if (dates.value.length === 0) return new Date();
  const idx = Math.max(0, Math.min(committedIndex.value, dates.value.length - 1));
  return dates.value[idx];
});
// Use midday UTC to avoid timezone day-shift issues when converting to ISO string
const selectedDayStr = computed(() => {
  const d = selectedDate.value;
  const safe = new Date(Date.UTC(d.getUTCFullYear(), d.getUTCMonth(), d.getUTCDate(), 12, 0, 0));
  return safe.toISOString().substring(0, 10);
});

const loading = ref(false);
const error = ref<string | null>(null);
const incidents = ref<Incident[]>([]);
const filteredIncidents = computed(() => incidents.value.filter((it) => it.day === selectedDayStr.value));

let map: any = null;
let markersLayer: any = null;

function commitSlider() {
  committedIndex.value = sliderIndex.value;
}

function buildDatesFromEnd(end: Date) {
  const arr = Array.from({ length: daysBack + 1 }, (_, i) => {
    const d = new Date(end);
    d.setDate(end.getDate() - (daysBack - i));
    return d;
  });
  dates.value = arr;
  sliderIndex.value = arr.length - 1;
  committedIndex.value = sliderIndex.value;
}

onMounted(async () => {
  // Initialize map
  map = Leaflet.map('map', {
    center: [45.9432, 24.9668], // Romania center-ish
    zoom: 7,
  });
  Leaflet.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; OpenStreetMap contributors'
  }).addTo(map);
  markersLayer = Leaflet.layerGroup().addTo(map);

  // Fetch latest end_date from count_incidents to set timeline end
  try {
    const resp = await server_api.countIncidents();
    const endDateStr = resp.data?.end_date; // format: YYYY-MM-DD
    let end = new Date();
    if (endDateStr && typeof endDateStr === 'string') {
      // Parse as midday UTC to avoid timezone shifting
      end = new Date(endDateStr + 'T12:00:00Z');
    }
    buildDatesFromEnd(end);
  } catch (e: any) {
    // Fallback to today if API fails
    buildDatesFromEnd(new Date());
  }

  await fetchIncidents();
  await refreshMarkers();
});

async function fetchIncidents() {
  if (dates.value.length === 0) return; // wait until dates initialized
  loading.value = true;
  error.value = null;
  try {
    // Fetch incidents directly filtered by selected day via API query param
    const resp = await server_api.getAllIncidents(undefined, undefined, undefined, selectedDayStr.value);
    incidents.value = resp.data.incidents;
  } catch (e: any) {
    error.value = e?.message ?? 'Failed to load incidents';
  } finally {
    loading.value = false;
  }
}

watch(selectedDayStr, async () => {
  // Refetch incidents for the newly selected day, then refresh markers
  await fetchIncidents();
  await refreshMarkers();
});



async function refreshMarkers() {
  if (!markersLayer) return;
  markersLayer.clearLayers();

  // Group incidents by county for the selected day
  const countsByCounty = new Map<string, number>();
  for (const it of filteredIncidents.value) {
    const key = (it.county || '').trim();
    if (!key) continue;
    countsByCounty.set(key, (countsByCounty.get(key) || 0) + 1);
  }

  const counties = Array.from(countsByCounty.keys());
  const layersCreated: any[] = [];

  for (const county of counties) {
    const norm = normalizeCountyName(county);
    const coords = COUNTY_COORDS[norm];
    if (!coords) {
      console.error("There are no known coordinates of the county.", { county, normalized: norm });
      continue; // Skip unknown counties to avoid runtime errors
    }
    const marker = Leaflet.marker([coords.lat, coords.lon]);
    const count = countsByCounty.get(county) || 0;
    marker.bindTooltip(`<strong>${county}</strong><br/>${count} incident(s)<br/><small>${coords.label}</small>`, { direction: 'top' });
    marker.addTo(markersLayer);
    layersCreated.push(marker);
  }

  // Fit bounds if we have markers
  const layers = markersLayer.getLayers();
  if (layers.length > 0) {
    const group = Leaflet.featureGroup(layers);
    map.fitBounds(group.getBounds().pad(0.2));
  }
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <h1 class="text-3xl font-semibold pt-4">Incidents Map</h1>

    <div class="card p-4 gap-2">
      <div class="flex items-center gap-4">
        <label class="font-medium">Day:</label>
        <input type="range" class="range range-primary flex-1" min="0" :max="dates.length - 1" v-model.number="sliderIndex" @change="commitSlider" />
        <span class="badge badge-lg">{{ selectedDayStr }}</span>
      </div>
      <div class="text-sm opacity-70">Drag the timeline to change the day and update incidents on the map.</div>
    </div>

    <div v-if="loading" class="alert">
      Loading incidents...
    </div>
    <div v-if="error" class="alert alert-error">
      {{ error }}
    </div>

    <div class="flex gap-4">
      <!-- Map on the left -->
      <div class="flex-1">
        <div id="map" class="w-full h-[70vh] rounded-box border border-base-300"></div>
      </div>

      <!-- Incidents list on the right -->
      <aside class="w-full max-w-md h-[70vh] rounded-box border border-base-300 overflow-auto p-3">
        <div class="flex items-center justify-between mb-2">
          <h2 class="text-xl font-semibold">Incidents</h2>
          <span class="badge">{{ filteredIncidents.length }}</span>
        </div>
        <div class="text-sm opacity-70 mb-3">Selected day: {{ selectedDayStr }}</div>

        <div v-if="filteredIncidents.length === 0" class="text-sm opacity-70">No incidents for this day.</div>

        <ul class="space-y-2">
          <li v-for="it in filteredIncidents" :key="it.id" class="p-2 rounded border border-base-200">
            <div class="font-medium">#{{ it.id }} — {{ it.county }} <span v-if="it.location">/ {{ it.location }}</span></div>
            <div class="text-sm opacity-70 truncate" :title="it.description">{{ it.description }}</div>
          </li>
        </ul>
      </aside>
    </div>

    <div class="text-sm opacity-70">
      Showing {{ filteredIncidents.length }} incident(s) for {{ selectedDayStr }}
    </div>
  </div>
</template>

<style scoped>
#map { height: 70vh; }
</style>
