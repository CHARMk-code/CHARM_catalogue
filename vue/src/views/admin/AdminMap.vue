<template>
  <q-page padding class="row">
    <q-card
      class="col-8"
      square
      flat
      style="height: calc(100vh - 100px); border-right: 1px solid lightgray"
    >
      <AdminMapViewer
        :fair-map-id="fairMapId"
        style="height: 100%"
        :selected-marker="selectedMarker"
        enable-drag
      />
    </q-card>

    <q-card class="col-4 q-pa-sm" flat style="height: calc(100vh - 100px)">
      <q-card-section>
        <p style="font-size: 1.5em">Global map settings</p>
        <div class="row q-pb-md">
          <q-file
            v-model="selectedBackgroundFile"
            dense
            label="upload new background image"
            filled
            accept="image/*"
          >
            <template #prepend>
              <q-icon name="attach_file" />
            </template>
          </q-file>
        </div>
        <div class="row"></div>
        <div class="row q-pb-sm">
          <span class="text-weight-medium"> Size of map </span>
          <span class="text-caption">
            (preferably same aspect-ratio as background image)
          </span>
          <q-input
            v-model="mapHeight"
            class="col q-pr-sm"
            type="number"
            dense
            label="height"
          ></q-input>
          <q-input
            v-model="mapWidth"
            class="col q-pl-sm"
            type="number"
            dense
            label="width"
          ></q-input>
        </div>
        <div class="row q-py-sm">
          <span class="text-weight-medium"> Maximum zoom </span>
          <q-input
            v-model="mapMaximumZoom"
            type="number"
            dense
            label="Maximum zoom"
          ></q-input>
        </div>
      </q-card-section>
      <q-card-section>
        <p style="font-size: 1.5em">Selected Marker</p>
        <q-select
          v-model="selectedCompanyIdName"
          label="Selected Company"
          use-input
          :options="filteredCompanies"
          virtual-scroll-slice-size="5"
          class="q-pb-md"
          @filter="filterCompanies"
        />
        <div v-if="selectedHasMarker" class="column q-gutter-xs">
          <div>
            <strong>Company booth number:</strong>
            {{ selectedCompany?.booth_number }}
          </div>
          <div class="q-pb-md">
            <strong>Saved Location: </strong>
            {{
              selectedMarker?.position.map(
                (n) => Math.round((n + Number.EPSILON) * 100) / 100,
              )
            }}
          </div>
          <!-- <q-btn -->
          <!--   color="primary" -->
          <!--   label="Remove Company Marker" -->
          <!--   @click="removeSelectedMarker" -->
          <!-- ></q-btn> -->
        </div>
        <q-btn
          v-show="selectedMarker && !selectedHasMarker"
          color="primary"
          label="Create Marker For Company"
        ></q-btn>
      </q-card-section>

      <q-card-section>
        <q-btn color="primary" label="Save changes" @click="saveMapChanges()" />
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import AdminMapViewer from "@/components/admin/map/AdminMapViewer.vue"
import axios from "@/plugins/axios";
import { Company, useCompaniesStore } from "@/stores/modules/companies"
import { useFairMapsStore, type MapGeometry } from "@/stores/modules/fairMaps"
import { computed, ref, watch } from "vue"

const companiesStore = useCompaniesStore()
const fairMapsStore = useFairMapsStore()

const fairMapId = fairMapsStore.currentState.selectedMap ?? 1

////////////////////////bin/
// Global settings logic
const mapHeight = computed({
  get() {
    return fairMapsStore.fairMaps.get(fairMapId)?.styling.mapSize[1]
  },
  set(height) {
    if (Number.isNaN(Number(height))) return
    const fairMap = fairMapsStore.fairMaps.get(fairMapId)
    if (fairMap) {
      fairMap.styling.mapSize[1] = Number(height)
    }
  },
})

const mapWidth = computed({
  get() {
    return fairMapsStore.fairMaps.get(fairMapId)?.styling.mapSize[0]
  },
  set(width) {
    if (Number.isNaN(Number(width))) return
    const fairMap = fairMapsStore.fairMaps.get(fairMapId)
    if (fairMap) {
      fairMap.styling.mapSize[0] = Number(width)
    }
  },
})

const mapMaximumZoom = computed({
  get() {
    return fairMapsStore.fairMaps.get(fairMapId)?.styling.maxZoom
  },
  set(maxZoom) {
    if (Number.isNaN(Number(maxZoom))) return
    const fairMap = fairMapsStore.fairMaps.get(fairMapId)
    if (fairMap) {
      fairMap.styling.maxZoom = maxZoom
    }
  },
})

const selectedBackgroundFile = ref<File>()

//////////////////////////////////
// Select company dropdown logic
const companyOptions = [...companiesStore.companies.values()]
  .map((company) => ({
    label: company.name,
    value: company.id,
  }))
  .sort((a, b) => (a.label < b.label ? -1 : a.label > b.label ? 1 : 0))
const filteredCompanies = ref(
  [...companiesStore.companies.values()].map((company) => ({
    label: company.name,
    value: company.id,
  })),
)
function filterCompanies(
  val: string,
  update: (callbackFn: () => void) => void,
) {
  update(() => {
    const needle = val.toLocaleLowerCase()
    filteredCompanies.value = companyOptions.filter(
      (v) => v.label.toLocaleLowerCase().indexOf(needle) > -1,
    )
  })
}

const selectedCompanyIdName = ref()

watch(
  () => fairMapsStore.currentState.selectedMarker?.refId,
  (newRefId) => {
    selectedCompanyIdName.value = companyOptions.find(
      (option) => option.value === newRefId,
    )
  },
)

const selectedCompany = computed<(Company & MapGeometry)>(() => {
  if (selectedCompanyIdName.value) {
    return companiesStore.companies.get(selectedCompanyIdName.value.value)
  }
  return undefined
})

const selectedMarker = computed(() => {
  const fairMapId = fairMapsStore.currentState.selectedMap
  if (selectedCompanyIdName.value && fairMapId) {
    const existingMarker = fairMapsStore.findMarker(
      fairMapId,
      selectedCompanyIdName.value.value,
    )

    if (existingMarker) {
      return existingMarker
    }

    const newMapGeom: MapGeometry = {
      id: selectedCompanyIdName.value.value,
      position: [0, 0],
      type: "company",
      refId: selectedCompanyIdName.value.value,
      styling: { backgroundColor: [255, 0, 0, 1] },
    }
    fairMapsStore.addOrReplaceMarker(fairMapId, newMapGeom)

    return newMapGeom
  }
  return undefined
})

const selectedHasMarker = computed(
  () => selectedCompanyIdName.value && selectedMarker.value,
)

//////////////////////
// Modifying Selecteds Marker
function removeSelectedMarker() {
  const fairMapId = fairMapsStore.currentState.selectedMap
  if (selectedMarker.value && fairMapId) {
    fairMapsStore.removeMarker(fairMapId, selectedMarker.value?.refId)
  }
}

async function saveMapChanges() {
  const fairMapId = fairMapsStore.currentState.selectedMap ?? 1
  const selectedMarker = fairMapsStore.currentState.selectedMarker
  if (selectedMarker) {
    const savedMarker = fairMapsStore.findMarker(
      fairMapId,
      selectedMarker.refId,
    )

    if (savedMarker) {
      savedMarker.position = selectedMarker.newPosition
    }
  }

  if (selectedBackgroundFile.value) {
    const file = selectedBackgroundFile.value
    const formData = new FormData()
    formData.append("file", file)
    await axios.post("/v2/image/", formData)
   
    const fairMap = fairMapsStore.fairMaps.get(fairMapId)
    if (fairMap) fairMap.background = file.name
  }

  await fairMapsStore.saveFairMap(fairMapId)

  location.reload()
}
</script>
