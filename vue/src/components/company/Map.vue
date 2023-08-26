<template>
  <company_card_wrapper name="map">
    <q-card-section>
      <div class="text-h6">Location</div>
    </q-card-section>
    <q-card-section class="text-subtitle-1 text-center">
      <q-icon color="primary" name="mdi-map-marker"></q-icon>
      Booth #{{ boothNumber }}
    </q-card-section>
    <q-card-section
      v-if="props.map"
      flat
    >
      <Image :ratio="1" fit="contain" :image-name="LOL" />
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import Image from "../utils/Image.vue";

const props = defineProps<{
  boothNumber: number;
  map: undefined;
}>();

// const mapsStore = useMapsStore();
</script>
