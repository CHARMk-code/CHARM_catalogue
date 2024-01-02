<template>
  <q-card class="q-ma-md">
    <q-layout id="mapLayoutContainer" container view="hHh lpR fFf">
      <q-drawer
        v-model="markerInformationSlider"
        behavior="desktop"
        bordered
        side="right"
      >
        <q-toolbar>
          <q-btn
            flat
            round
            dense
            icon="mdi-window-close"
            color="primary"
            @click="
              () => {
                markerInformationSlider = false;
              }
            "
          />
          <q-toolbar-title v-if="markerType === 'other'"
            >Marker Information</q-toolbar-title
          >
          <q-toolbar-title v-if="markerType === 'company'"
            >Company Information</q-toolbar-title
          >
        </q-toolbar>
        <CompanyMarkerInformation :company-id="markerId" />
        <!-- <OtherMarkerInformation v-else /> -->
      </q-drawer>

      <q-page-container >
          <q-page style="height: calc(100vh - 100px)">
              <MapViewer :fair-map-id="1" style="height: 100%" @marker-clicked="markerClicked"/>
        </q-page>
      </q-page-container>
    </q-layout>
  </q-card>
</template>

<script lang="ts" setup>
import CompanyMarkerInformation from "@/components/map/CompanyMarkerInformation.vue";
import MapViewer from "@/components/map/MapViewer.vue";
import type { GeometryTypes } from "@/stores/modules/fairMaps";
import type Feature from "ol/Feature";
import { ref, type Ref } from "vue";

const markerInformationSlider = ref(false);

const markerType: Ref<GeometryTypes> = ref("other");

const markerId: Ref<number> = ref(0);

function markerClicked(feature: Feature) {
  markerInformationSlider.value = true;
  markerType.value = "company";
  markerId.value = feature.get("refId");
  console.log(markerId, markerType);
}
</script>

<style scoped lang="scss">
#mapLayoutContainer {
  height: calc(100vh - 90px);
}
</style>
