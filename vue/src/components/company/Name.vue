<template>
  <company_card_wrapper name="name" flat>
    <div class="d-flex flex-row-reverse">
      <div class="pl-6">
        <v-tooltip right max-width="300px">
          <template v-slot:activator="{ props }">
            <v-checkbox
              v-bind="props"
              class="large"
              v-model="favorite"
              on-icon="mdi-star"
              off-icon="mdi-star-outline"
            />
          </template>
          <span>
            Save as favorite (This is only stored in your browser and will not
            be transferred between browsers)
          </span>
        </v-tooltip>
      </div>
      <div
        class="text-h5 text-md-h3 font-weight-regular"
        style="align-self: center"
      >
        {{ name }}
      </div>
    </div>
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
    return favoritesStore.isFavorite(props.id);
  },
  set() {
    if (favoritesStore.isFavorite(props.id)) {
      favoritesStore.addFavorite(props.id);
    } else {
      favoritesStore.removeFavorite(props.id);
    }
  },
});
</script>
