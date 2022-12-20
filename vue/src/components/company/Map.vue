<template>
  <company_card_wrapper class="mb-6" name="map">
    <v-card-title> Location </v-card-title>
    <v-card-text class="text-subtitle-1 text-center">
      <v-icon color="primary" large>mdi-map-marker</v-icon> Booth #{{
        booth_number
      }}
      <v-card
        v-if="props.map"
        flat
        :to="'/maps/' + mapsStore.getMapFromId(props.map.ref)"
      >
        <v-img
          class="ma-2"
          max-height="400px"
          contain
          :src="base_URL + props.map.image"
        />
      </v-card>
    </v-card-text>
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
