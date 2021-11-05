<template>
  <v-main>
    <searchCard />
    <Table
      name="Results"
      :headers="headers"
      :data="modifiedFilteredCompanies"
      :editable="false"
      @click_row="onRowClick"
    >
      <template v-slot:item.website="{ item }">
        <a :href="item.website">{{ item.website }}</a>
      </template>

      <template v-slot:item.tags="{ item }">
        <template v-for="tag in tagsFromIDs(item.tags)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.divisions="{ item }">
        <template v-for="tag in tagsFromIDs(item.divisions)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.looking_for="{ item }">
        <template v-for="tag in tagsFromIDs(item.looking_for)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.offering="{ item }">
        <template v-for="tag in tagsFromIDs(item.offering)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
      <template v-slot:item.business_area="{ item }">
        <template v-for="tag in tagsFromIDs(item.business_area)">
          <v-chip class="mr-1 mb-1" x-small :key="tag.id">{{
            tag.name
          }}</v-chip>
        </template>
      </template>
    </Table>
  </v-main>
</template>

<script>
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
        { text: "Website", value: "website" },
        { text: "Programs", value: "divisions" },
        { text: "Business areas", value: "business_area" },
        { text: "Looking for", value: "looking_for" },
        { text: "offering", value: "offering" },
        { text: "Other tags", value: "" },
      ],
    };
  },
  computed: {
    ...mapGetters({ filteredCompanies: "filter/filteredCompanies" }),
    modifiedFilteredCompanies() {
      const companies = Array.from(this.filteredCompanies);
      const modified = companies.map((c) => ({
        ...c,
        divisions: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.division)
          .map((t) => t.id),
        looking_for: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.looking_for)
          .map((t) => t.id),
        offering: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.offering)
          .map((t) => t.id),
        business_area: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter((t) => t.business_area)
          .map((t) => t.id),
        tags: c.tags
          .map((id) => this.$store.getters["tags/getTagFromId"](id))
          .filter(
            (t) =>
              !(t.divisions || t.looking_for || t.offering || t.business_area)
          )
          .map((t) => t.id),
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
