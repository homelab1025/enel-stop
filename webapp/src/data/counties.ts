// Centralized county data and helpers for map visualizations and lookups
// Coordinates are approximate centers (county seats) for Romanian counties including Bucharest

export type CountyCenter = { key: string; lat: number; lon: number; label: string; aliases?: string[] };

// Normalize county names: trim, lowercase, remove diacritics and punctuation
export function normalizeCountyName(name: string): string {
  const s = (name || '').trim().toLowerCase();
  // Replace Romanian diacritics
  const map: Record<string, string> = {
    'ă': 'a', 'â': 'a', 'î': 'i', 'ș': 's', 'ş': 's', 'ț': 't', 'ţ': 't', 'é': 'e', 'á': 'a', 'ó': 'o', 'ú': 'u', 'í': 'i'
  };
  const replaced = s.replace(/[ăâîșşțţéáóúí]/g, (c) => map[c] || c);
  // Remove dots and extra spaces
  return replaced.replace(/[,.;:]/g, '').replace(/\s+/g, ' ').trim();
}

export const COUNTY_CENTERS: CountyCenter[] = [
  { key: 'alba', lat: 46.077, lon: 23.58, label: 'Alba' },
  { key: 'arad', lat: 46.186, lon: 21.312, label: 'Arad' },
  { key: 'arges', lat: 44.856, lon: 24.869, label: 'Argeș', aliases: ['arges'] },
  { key: 'bacau', lat: 46.567, lon: 26.913, label: 'Bacău', aliases: ['bacau'] },
  { key: 'bihor', lat: 47.047, lon: 21.918, label: 'Bihor' },
  { key: 'bistrita-nasaud', lat: 47.135, lon: 24.49, label: 'Bistrița-Năsăud', aliases: ['bistrita nasaud', 'bistrita'] },
  { key: 'botosani', lat: 47.748, lon: 26.669, label: 'Botoșani', aliases: ['botosani'] },
  { key: 'brasov', lat: 45.658, lon: 25.601, label: 'Brașov', aliases: ['brasov'] },
  { key: 'braila', lat: 45.271, lon: 27.957, label: 'Brăila', aliases: ['braila'] },
  { key: 'bucuresti', lat: 44.4268, lon: 26.1025, label: 'București', aliases: ['bucharest', 'mun bucuresti', 'municipiul bucuresti', 'ilfov-bucuresti'] },
  { key: 'buzau', lat: 45.152, lon: 26.823, label: 'Buzău', aliases: ['buzau'] },
  { key: 'caras-severin', lat: 45.3, lon: 21.889, label: 'Caraș-Severin', aliases: ['caras severin', 'caras'] },
  { key: 'calarasi', lat: 44.206, lon: 27.312, label: 'Călărași', aliases: ['calarasi'] },
  { key: 'cluj', lat: 46.77, lon: 23.59, label: 'Cluj' },
  { key: 'constanta', lat: 44.159, lon: 28.634, label: 'Constanța', aliases: ['constanta'] },
  { key: 'covasna', lat: 45.867, lon: 25.79, label: 'Covasna' },
  { key: 'dambovita', lat: 44.927, lon: 25.456, label: 'Dâmbovița', aliases: ['dambovita'] },
  { key: 'dolj', lat: 44.317, lon: 23.8, label: 'Dolj' },
  { key: 'galati', lat: 45.436, lon: 28.053, label: 'Galați', aliases: ['galati'] },
  { key: 'giurgiu', lat: 43.903, lon: 25.969, label: 'Giurgiu' },
  { key: 'gorj', lat: 45.046, lon: 23.274, label: 'Gorj' },
  { key: 'harghita', lat: 46.36, lon: 25.801, label: 'Harghita' },
  { key: 'hunedoara', lat: 45.876, lon: 22.913, label: 'Hunedoara' },
  { key: 'ialomita', lat: 44.563, lon: 27.366, label: 'Ialomița', aliases: ['ialomita'] },
  { key: 'iasi', lat: 47.158, lon: 27.601, label: 'Iași', aliases: ['iasi'] },
  { key: 'ilfov', lat: 44.5, lon: 26.2, label: 'Ilfov' },
  { key: 'maramures', lat: 47.66, lon: 23.57, label: 'Maramureș', aliases: ['maramures'] },
  { key: 'mehedinti', lat: 44.63, lon: 22.66, label: 'Mehedinți', aliases: ['mehedinti'] },
  { key: 'mures', lat: 46.54, lon: 24.56, label: 'Mureș', aliases: ['mures'] },
  { key: 'neamt', lat: 46.93, lon: 26.37, label: 'Neamț', aliases: ['neamt'] },
  { key: 'olt', lat: 44.43, lon: 24.365, label: 'Olt' },
  { key: 'prahova', lat: 44.95, lon: 26.01, label: 'Prahova' },
  { key: 'satu mare', lat: 47.79, lon: 22.89, label: 'Satu Mare', aliases: ['satumare'] },
  { key: 'salaj', lat: 47.182, lon: 23.057, label: 'Sălaj', aliases: ['salaj'] },
  { key: 'sibiu', lat: 45.793, lon: 24.121, label: 'Sibiu' },
  { key: 'suceava', lat: 47.651, lon: 26.255, label: 'Suceava' },
  { key: 'teleorman', lat: 43.97, lon: 25.33, label: 'Teleorman' },
  { key: 'timis', lat: 45.75, lon: 21.23, label: 'Timiș', aliases: ['timis'] },
  { key: 'tulcea', lat: 45.18, lon: 28.8, label: 'Tulcea' },
  { key: 'valcea', lat: 45.1, lon: 24.37, label: 'Vâlcea', aliases: ['valcea', 'vilcea'] },
  { key: 'vaslui', lat: 46.64, lon: 27.73, label: 'Vaslui' },
  { key: 'vrancea', lat: 45.698, lon: 27.183, label: 'Vrancea' },
];

export const COUNTY_COORDS: Record<string, { lat: number; lon: number; label: string }> = (() => {
  const map: Record<string, { lat: number; lon: number; label: string }> = {};
  for (const item of COUNTY_CENTERS) {
    map[normalizeCountyName(item.key)] = { lat: item.lat, lon: item.lon, label: item.label };
    if (item.aliases) {
      for (const a of item.aliases) {
        map[normalizeCountyName(a)] = { lat: item.lat, lon: item.lon, label: item.label };
      }
    }
    // Also support forms with 'judetul' or 'jud.' prefixes
    map[normalizeCountyName(`judetul ${item.key}`)] = { lat: item.lat, lon: item.lon, label: item.label };
    map[normalizeCountyName(`jud ${item.key}`)] = { lat: item.lat, lon: item.lon, label: item.label };
  }
  return map;
})();
