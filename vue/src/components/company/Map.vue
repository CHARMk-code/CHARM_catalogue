<template>
  <company_card_wrapper name="map">
    <q-card-section>
      <div class="float-left text-h6">Location</div>
      <div class="float-right text-subtitle q-pt-sm">
        <q-icon color="primary" name="mdi-map-marker"></q-icon>
        Booth #{{ boothNumber }}
      </div>
    </q-card-section>
    <q-card-section>
      <q-responsive :ratio="1" style="position: relative;">
        <MapViewer :fair-map-id="fairMapId" no-selection :selected-marker="boothNumber"/>
      </q-responsive>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import MapViewer from "../map/MapViewer.vue";
import { useFairMapsStore } from "@/stores/modules/fairMaps";

const props = defineProps<{
  boothNumber: number;
}>();


const fairMapId = useFairMapsStore().currentState.selectedMap ?? 1

// const mapsStore = useMapsStore();
</script>
