<template>
  <company_card_wrapper name="map">
    <q-card-section>
      <div class="text-h6">Location</div>
    </q-card-section>
    <q-card-section class="text-subtitle-1 text-center">
      <q-icon color="primary" name="mdi-map-marker"></q-icon>
      Booth #{{ booth_number }}
    </q-card-section>
    <q-card-section
      v-if="props.map"
      flat
      :to="'/maps/' + mapsStore.getMapFromId(props.map.ref)"
    >
      <q-img
        class="ma-2"
        height="400px"
        width="400px"
        fit="contain"
        :src="base_URL + props.map.image"
      />
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import axios from "@/plugins/axios";
import { useMapsStore, type Company_Map } from "@/stores/modules/maps";

const props = defineProps<{
  booth_number: number;
  map: Company_Map | undefined;
}>();
const base_URL = axios.defaults.baseURL + "/manage/image/";

const mapsStore = useMapsStore();
</script>
