<template>
  <v-card min-width="296px" v-if="isVisible" :flat="flat">
    <slot />
  </v-card>
</template>

<script>
import { mapStores } from "pinia";
import { useSite_settingsStore } from "@/stores/modules/site_settings";

export default {
  name: "Company_card_wrapper",
  props: { name: String, flat: { type: Boolean, default: false } },
  computed: {
    ...mapStores(useSite_settingsStore)
    },
    isVisible() {
      const visibleCards = this.site_settingsStore.company_view.cards;
      return this.visibleCards.some((c) =>
        c.name === this.name ? c.active : false
      );
    },
  },
};
</script>
