<template>
  <company_card_wrapper name="name" flat>
    <q-card-section style="height: 150px">
      <q-checkbox
        class="absolute-right"
        checked-icon="mdi-star"
        unchecked-icon="mdi-star-outline"
        v-model="favorite"
        :color="favorite ? 'primary' : 'grey'"
        size="xl"
        left-label
      >
        <span class="text-h3">{{ name }}</span>
        <q-tooltip>
          Save as favorite (This is only stored in your browser and will not be
          transferred between browsers)
        </q-tooltip>
      </q-checkbox>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import { useFavoritesStore } from "@/stores/modules/favorites";
import { computed } from "vue";

const props = defineProps<{
  name: string;
  id: number;
}>();

const favoritesStore = useFavoritesStore();

const favorite = computed({
  get() {
    return useFavoritesStore().isFavorite(props.id);
  },
  set(v) {
    if (v) {
      favoritesStore.addFavorite(props.id);
    } else {
      favoritesStore.removeFavorite(props.id);
    }
  },
});
</script>
