<template>
  <q-card v-if="isVisible" :flat="flat">
    <slot />
  </q-card>
</template>

<script lang="ts" setup>
import { useSite_settingsStore } from "@/stores/modules/site_settings";
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    name: string;
    flat?: boolean;
  }>(),
  { flat: false }
);

const isVisible = computed(() => {
  const site_settingsStore = useSite_settingsStore();
  const visibleCards = site_settingsStore.server_settings.company_view.cards;
  return visibleCards.some((card) =>
    card.name === props.name ? card.active : false
  );
});
</script>
