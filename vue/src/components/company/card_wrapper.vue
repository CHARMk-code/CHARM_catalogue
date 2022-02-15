<template>
  <v-card min-width="296px" v-if="isVisible" :flat="flat">
    <slot />
  </v-card>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "Company_card_wrapper",
  props: { name: String, flat: { type: Boolean, default: false } },
  computed: {
    ...mapGetters({
      visibleCards: "site_settings/getCompanyCards",
    }),
    isVisible() {
      return this.visibleCards.some((c) =>
        c.name === this.name ? c.active : false
      );
    },
  },
};
</script>
