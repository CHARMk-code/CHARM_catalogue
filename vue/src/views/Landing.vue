<template>
  <q-page padding>
    <div class="row full-height">
      <q-space />
      <div class="full-height col-12 col-md-4">
        <q-card
          v-if="first_prepage"
          id="prepage-card"
          class="q-mx-auto q-mb-lg"
        >
          <q-card-section class="col-grow">
            <Image
              fit="scale-down"
              height="100%"
              :image-name="first_prepage.image"
            />
          </q-card-section>
          <q-card-actions :align="'center'" class="text-h5 text-center">
            <q-btn
              no-caps
              color="primary"
              size="xl"
              label="Browse the catalogue"
              to="/prepage/1"
            ></q-btn>
          </q-card-actions>
        </q-card>
      </div>
      <q-space />
      <div class="col-12 col-md-4">
        <span class="text-h3 text-center block full-width">Shortcuts</span>
        <template v-for="shortcut in shortcutsStore.shortcuts" :key="shortcut.id">
          <div class="q-pa-sm full-width">
            <Shortcut
              :icon="shortcut.icon"
              :name="shortcut.name"
              :desc="shortcut.desc"
              :link="shortcut.link"
            />
          </div>
        </template>
      </div>
      <q-space />
    </div>
  </q-page>
</template>

<script lang="ts" setup>
import Shortcut from "@/components/landing/shortcut.vue";
import Image from "@/components/utils/Image.vue";
import { computed } from "vue";
import { usePrepagesStore } from "@/stores/modules/prepages";
import { useShortcutsStore } from "@/stores/modules/shortcuts";

const prepagesStore = usePrepagesStore();
const shortcutsStore = useShortcutsStore();

const first_prepage = computed(() => {
  if (Object.values(prepagesStore.pageGroups).length > 1)
    return prepagesStore.pageGroups[1].pages[0];
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
