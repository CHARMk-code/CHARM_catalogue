<template>
  <q-page padding class="row">
    <q-card
      class="col-8"
      square
      flat
      style="height: calc(100vh - 100px); border-right: 1px solid lightgray"
    >
      <AdminMapViewer
        :fair-map-id="1"
        style="height: 100%"
        :selected-marker="selectedMarker"
        enable-drag
      />
    </q-card>

    <q-card class="col-4 q-pa-sm" flat style="height: calc(100vh - 100px)">
      <p style="font-size: 1.5em">Selected Marker</p>
      <q-select
        v-model="selectedCompanyIdName"
        label="Selected Company"
        use-input
        :options="filteredCompanies"
        virtual-scroll-slice-size="5"
        @filter="filterCompanies"
      />
      <div v-if="selectedHasMarker" class="column q-gutter-xs">
        <div>
          <strong>Company booth number:</strong>
          {{ selectedCompany?.booth_number }}
        </div>
        <div>
          <strong>Saved Location: </strong>
          {{
            selectedMarker?.position.map(
              (n) => Math.round((n + Number.EPSILON) * 100) / 100,
            )
          }}
        </div>
        <div>
          <strong>New Location: </strong>
          {{
            newMarkerPosition?.map(
              (n) => Math.round((n + Number.EPSILON) * 100) / 100,
            )
          }}
        </div>
        <q-btn
          color="primary"
          label="Remove Company Marker"
          @click="removeSelectedMarker"
        ></q-btn>
        <q-btn
          color="primary"
          label="Update Company Marker"
          @click="updateSelectedMarker"
        ></q-btn>
      </div>
      <q-btn
        v-show="selectedMarker && !selectedHasMarker"
        color="primary"
        label="Create Marker For Company"
      ></q-btn>

      Global map styling <br />
      lägga till/ändra backgrundsbild<br />
      circleSize<br />
      maxZoom<br />
      mapSize<br />
      konfigurera en defaultMarkerStyling<br />
      export fairMapConfiguration<br />

      Kunna lägga till och redigera alla attribut för en MapGeometry (förutom
      id)
      <div v-show="selectedMarker">
        <q-btn
          color="primary"
          label="Save changes"
          @click="saveNewMarkerLocation()"
        />
      </div>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import AdminMapViewer from "@/components/admin/map/AdminMapViewer.vue"
import { Company, useCompaniesStore } from "@/stores/modules/companies"
import { useFairMapsStore, type MapGeometry } from "@/stores/modules/fairMaps"
import { computed, ref, watch } from "vue"

const companiesStore = useCompaniesStore()
const fairMapsStore = useFairMapsStore()

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

const selectedCompany = computed<(Company & MapGeometry) | undefined>(() => {
  console.log(selectedCompanyIdName)
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

    if(existingMarker) {
      return existingMarker
    }

    const newMapGeom: MapGeometry = {
      id: selectedCompanyIdName.value.value,
      position: [0,0],
      type: "company",
      refId: selectedCompanyIdName.value.value,
      styling: { backgroundColor: [255, 0, 0, 1]},
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

const newMarkerPosition = computed(
  () => fairMapsStore.currentState.selectedMarker?.newPosition,
)

function saveNewMarkerLocation() {
  if (newMarkerPosition.value) {
    const markerId = fairMapsStore.currentState.selectedMarker?.refId
    const fairMapId = fairMapsStore.currentState.selectedMap
    if (markerId && fairMapId) {
      const selectedMarker = fairMapsStore.findMarker(fairMapId, markerId)

      if (selectedMarker) {
        selectedMarker.position = newMarkerPosition.value
        console.log(fairMapId)
        fairMapsStore.saveFairMap(fairMapId)
      }
    }
  }
}

const newMarker = ref<MapGeometry | undefined>(undefined)

// function initiateCompanyMarkerPlacement() {
//   const companyId = companyLinkedToNewMarker.value.value;
//   const newMarkerGeometry: MapGeometry = {
//     id: companyId,
//     position: [0, 0],
//     type: "company",
//     refId: companyId,
//     styling: { backgroundColor: [255, 0, 0, 1] },
//   };
//
//   const fairMapId = fairMapsStore.currentState.selectedMap ?? NaN;
//   fairMapsStore.fairMaps.get(fairMapId)?.mapGeometry.push(newMarkerGeometry);
//
//   newMarker.value = newMarkerGeometry;
// }
</script>
