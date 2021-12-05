<template>
  <v-main>
    <v-container>
      <v-row>
        <searchCard />
        <Table
          class="mx-auto"
          noSearch="true"
          name="Results"
          :headers="headers"
          :data="modifiedFilteredCompanies"
          :editable="false"
          @click_row="onRowClick"
        >
          <template v-slot:item.tags="{ item }">
            <template v-for="tag in item.tags">
              <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
                tag.name
              }}</v-chip>
            </template>
          </template>
          <template v-slot:item.divisions="{ item }">
            <template v-for="tag in item.divisions">
              <template v-if="tag.icon != ''">
                <v-avatar size="24px" class="ma-1" x-small :key="tag.id">
                  <v-img
                    max-height="32px"
                    max-width="32px"
                    :src="base_URL + tag.icon"
                  />
                </v-avatar>
              </template>
              <template v-else>
                <v-chip small :key="tag.id">
                  {{ item.name }}
                </v-chip>
              </template>
            </template>
          </template>
          <template v-slot:item.looking_for="{ item }">
            <template v-for="tag in item.looking_for">
              <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
                tag.name
              }}</v-chip>
            </template>
          </template>
          <template v-slot:item.offering="{ item }">
            <template v-for="tag in item.offerings">
              <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
                tag.name
              }}</v-chip>
            </template>
          </template>
          <template v-slot:item.business_area="{ item }">
            <template v-for="tag in item.business_area">
              <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
                tag.name
              }}</v-chip>
            </template>
          </template>
          <template v-slot:item.id="{ item }">
            <v-checkbox
              :input-value="$store.getters['favorites/favorites'].has(item.id)"
              on-icon="mdi-star"
              off-icon="mdi-star-outline"
            />
            <!--  TODO: Should make start clickable to changes status-->
          </template>
        </Table>
      </v-row>
    </v-container>
  </v-main>
</template>

<script>
import Vue from "vue";
import { mapGetters } from "vuex";
import searchCard from "@/components/search/search_card";
import Table from "@/components/table";
export default {
  name: "Search",
  components: {
    searchCard,
    Table,
  },
  data() {
    return {
      headers: [
        { text: "Name", value: "name" },
        { text: "Programs", value: "divisions" },
        { text: "Business areas", value: "business_area" },
        { text: "Looking for", value: "looking_for" },
        { text: "Offering", value: "offering" },
        { text: "Liked", value: "id" },
      ],
    };
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },

    ...mapGetters({ filteredCompanies: "filter/filteredCompanies" }),
    modifiedFilteredCompanies() {
      const companies = Array.from(this.filteredCompanies);
      const modified = companies.map((c) => ({
        ...c,
        divisions: this.$store.getters["tags/getDivisionsFromIds"](c.tags),
        looking_for: this.$store.getters["tags/getLookingForFromIds"](c.tags),
        offerings: this.$store.getters["tags/getOffersFromIds"](c.tags),
        business_area: this.$store.getters["tags/getBusinessAreasFromIds"](
          c.tags
        ),
        tags: this.$store.getters["tags/getTagsFromIds"](c.tags),
      }));
      return modified;
    },
  },
  methods: {
    tagsFromIDs(tags) {
      return tags.map((id) => {
        return this.$store.getters["tags/getTagFromId"](id);
      });
    },
    onRowClick(row) {
      this.$router.push("/company/" + row.name);
    },
  },
};
</script>
