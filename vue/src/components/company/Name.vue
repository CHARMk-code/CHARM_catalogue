<template>
  <company_card_wrapper name="name" flat>
    <div class="d-flex flex-row-reverse">
      <div class="pl-6">
        <v-tooltip right max-width="300px">
          <template v-slot:activator="{ on, attrs }">
            <div v-on="on">
              <v-checkbox
                class="large"
                v-bind="attrs"
                @change="favoriteChange"
                v-model="favorite"
                on-icon="mdi-star"
                off-icon="mdi-star-outline"
              />
            </div>
          </template>
          <span>
            Save as favourite (This is only stored in your browser and will not
            be transfered between browsers)
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

<script>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

export default {
  name: "Company_Name",
  props: ["name", "id"],
  components: {
    company_card_wrapper,
  },
  data() {
    return {
      favorite: false,
    };
  },
  methods: {
    favoriteChange() {
      if (this.favorite) {
        this.$store.commit("favorites/addFavorite", this.id);
      } else {
        this.$store.commit("favorites/removeFavorite", this.id);
      }
    },
  },
  mounted() {
    this.favorite = this.$store.getters["favorites/favorites"].has(this.id);
  },
};
</script>
