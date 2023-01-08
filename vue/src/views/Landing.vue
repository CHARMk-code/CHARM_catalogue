<template>
  <q-page padding>
    <div class="row">
      <div class="col-12 col-md-6">
        <q-card
          class="q-mx-auto"
          :style="{ width: $q.screen.lt.md ? '75%' : '100%' }"
          v-if="first_prepage"
        >
          <Image
            :height="$q.screen.lt.md ? '400px' : ''"
            fit="contain"
            :imageName="first_prepage.image"
          />
          <q-card-actions :align="'center'" class="text-h5 text-center">
            <q-btn
              no-caps
              color="primary"
              size="lg"
              label="Browse the catalogue"
              to="/prepage/1"
            ></q-btn>
          </q-card-actions>
        </q-card>
      </div>
      <div class="col-12 col-md-6">
        <span class="text-h4 text-center block full-width">Shortcuts</span>
        <template v-for="shortcut in shortcutsStore.shortcuts">
          <div class="q-pa-sm full-width">
            <shortcut
              :icon="shortcut.icon"
              :name="shortcut.name"
              :desc="shortcut.desc"
              :link="shortcut.link"
            />
          </div>
        </template>
      </div>
    </div>
  </q-page>
</template>

<script lang="ts" setup>
import hoverCard from "@/components/landing/hoverCard.vue";
import shortcut from "@/components/landing/shortcut.vue";
import Image from "@/components/utils/Image.vue";
import { computed } from "vue";
import { usePrepagesStore } from "@/stores/modules/prepages";
import { useShortcutsStore } from "@/stores/modules/shortcuts";

const prepagesStore = usePrepagesStore();
const shortcutsStore = useShortcutsStore();

const first_prepage = computed(() => {
  const active_prepages = prepagesStore.active_prepages;
  if (active_prepages.length > 0) {
    return active_prepages[0];
  } else {
    return undefined;
  }
});
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
