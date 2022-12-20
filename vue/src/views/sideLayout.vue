<template>
  <v-btn
    v-if="button_left"
    class="navigation hidden-sm-and-down"
    elevation="4"
    x-large
    v-on:click="$emit('prev')"
    icon
  >
    <v-icon x-large>mdi-arrow-left</v-icon>
  </v-btn>
  <v-btn
    v-if="button_right"
    class="navigation hidden-sm-and-down"
    v-on:click="$emit('next')"
    style="right: 0%"
    x-large
    elevation="4"
    icon
  >
    <v-icon x-large>mdi-arrow-right</v-icon>
  </v-btn>
  <v-container
    class="ma-0 pa-0 d-flex justify-space-between"
    style="
      position: absolute;
      top: 0;
      width: 100vw;
      height: 100vh;
      max-width: 100%;
    "
  >
    <img
      class="ma-0 pa-0 hidden-sm-and-down flex-shrink-1"
      max-height="100%"
      v-if="leftLayout != undefined"
      :src="base_URL + leftLayout.image"
    />
    <div class="ma-0 pt-16 flex-grow-1">
      <slot />
    </div>
    <img
      class="ma-0 pa-0 hidden-sm-and-down flex-shrink-1"
      max-height="100%"
      v-if="rightLayout != undefined"
      :src="base_URL + rightLayout.image"
    />
  </v-container>
</template>

<script lang="ts" setup>
import { useLayoutsStore } from "@/stores/modules/layouts";
import { computed } from "@vue/reactivity";
import axios from "@/plugins/axios";

const props = withDefaults(
  defineProps<{
    button_left?: boolean;
    button_right?: boolean;
  }>(),
  {
    button_left: false,
    button_right: false,
  }
);

const emit = defineEmits<{
  (e: "next"): void;
  (e: "prev"): void;
}>();

const layoutsStore = useLayoutsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const leftLayout = computed(() => layoutsStore.getSide("left"));
const rightLayout = computed(() => layoutsStore.getSide("right"));
</script>

<style scoped>
.navigation {
  text-decoration: none;
  margin: 50px;
  position: fixed;
  z-index: 9999999;
  top: 50%;
}
.navigation > * {
  top: -50%;
}
</style>
