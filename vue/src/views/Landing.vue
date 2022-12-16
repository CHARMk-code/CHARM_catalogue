<template>
  <sideLayout>
    <v-container>
      <v-row style="justify-content: center">
        <v-col full-width xs="12" md="6">
          <div class="mx-auto mb-4 text-center text-h5 text-md-h3">
            Browse the catalogue
          </div>
          <hoverCard
            v-if="first_prepage"
            class="mx-auto"
            to="/prepages/0"
            :background_image="base_URL + first_prepage.image"
          />
        </v-col>
        <v-col xs="12" md="6">
          <div class="mx-auto mb-4 d-block text-center text-h5 text-md-h3">
            Shortcuts
          </div>
          <v-container class="ma-0 pa-0 d-flex flex-wrap" style="gap: 16px">
            <shortcut
              v-for="shortcut in shortcutsStore.shortcuts"
              :key="shortcut.name"
              :icon="shortcut.icon"
              :name="shortcut.name"
              :desc="shortcut.desc"
              :link="shortcut.link"
            />
          </v-container>
        </v-col>
      </v-row>
    </v-container>
  </sideLayout>
</template>

<script lang="ts" setup>
import sideLayout from "@/views/sideLayout.vue";
import hoverCard from "@/components/landing/hoverCard.vue";
import shortcut from "@/components/landing/shortcut.vue";
import axios from "@/plugins/axios";
import { computed } from "vue";
import { usePrePagesStore } from "@/stores/modules/prepages";
import { useShortcutsStore } from "@/stores/modules/shortcuts";

const base_URL = axios.defaults.baseURL + "/manage/image/";
const prepagesStore = usePrePagesStore();
const shortcutsStore = useShortcutsStore();

const first_prepage = computed(() => {
  const active_prepages = prepagesStore.active_prepages;
  if (active_prepages.length > 0) {
    return active_prepages[0];
  } else {
    return undefined;
  }
})
</script>

<style scoped>
/*.v-card {
  transition: opacity 0.4s ease-in-out;
}

.v-card:not(.on-hover) {
  opacity: 0.4;
}*/
#hovereffect {
  transition: opacity 0.4s ease-in-out;
  opacity: 0.75;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
#hovereffect2 {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect2:not(.on-hover) {
  opacity: 0;
}
</style>
