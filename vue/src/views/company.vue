<template>
  <sideLayout
    @next="next"
    @prev="prev"
    v-bind:button_left="true"
    v-bind:button_right="currentIndex < maxIndex - 1"
  >
    <v-container
      v-if="company != undefined && company.active == true"
      v-touch="{
        right: () => prev(),
        left: () => next(),
      }"
    >
      <v-row>
        <v-col align="center" xs="12" sm="12" md="auto">
          <Logo :src="company.logo" />
        </v-col>
        <v-col align-content="start">
          <Name :name="company.name" :id="company.id" />
        </v-col>
      </v-row>

      <v-row>
        <v-col>
          <Textblock :body="company.description" class="mb-6" />
          <Textblock
            :body="company.unique_selling_point"
            title="What Makes Us Special"
            class="mb-6"
          />
          <Summerjob
            name="summerjob"
            :desc="company.summer_job_description"
            :link="company.summer_job_link"
            :deadline="company.summer_job_deadline"
            class="mb-6"
          />
          <Layout class="mb-6" />
          <v-row>
            <v-col
              class="pa-0 mb-0 d-flex flex-wrap justify-space-between"
              style="margin: 0 -3px"
            >
              <Tags
                :tags="tagsStore.getDivisionsFromIds(company.tags)"
                name="tag_divisions"
                title="Divisions"
                class="mb-6 mx-3 flex-grow-1"
              />
              <Tags
                :tags="tagsStore.getLookingForFromIds(company.tags)"
                name="tag_looking_for"
                title="Looking For"
                getter_target="tags/getLookingForFromIds"
                class="mb-6 mx-3 flex-grow-1"
              />
              <Tags
                :tags="tagsStore.getOfferingsFromIds(company.tags)"
                name="tag_offering"
                title="Offering"
                getter_target="tags/getOffersFromIds"
                class="mb-6 mx-3 flex-grow-1"
              />
              <Tags
                :tags="tagsStore.getBusinessAreasFromIds(company.tags)"
                name="tag_business_areas"
                title="Business Areas"
                getter_target="tags/getBusinessAreasFromIds"
                class="mb-6 mx-3 flex-grow-1"
              />
            </v-col>
          </v-row>
          <Website :website="company.website" class="mb-6" />
          <Contacts
            :contacts="company.contacts"
            :contact_email="company.contact_email"
          />
        </v-col>
        <v-col>
          <Map
            :map="mapsStore.getMapFromId(company.map_image)"
            :booth_number="company.booth_number"
            class="mb-6"
          />
          <Trivia
            :talk_to_us_about="company.talk_to_us_about"
            :sweden="company.employees_sweden"
            :world="company.employees_world"
            class="mb-6"
          />
          <Note :id="company.id" class="mb-6" />
        </v-col>
      </v-row>
    </v-container>
  </sideLayout>
</template>

<script lang="ts" setup>
import Logo from "@/components/company/Logo.vue";
import Name from "@/components/company/Name.vue";
import Trivia from "@/components/company/Trivia.vue";
import Contacts from "@/components/company/Contacts.vue";
import Textblock from "@/components/company/Textblock.vue";
import Website from "@/components/company/Website.vue";
import Tags from "@/components/company/Tags.vue";
import Note from "@/components/company/Note.vue";
import Map from "@/components/company/Map.vue";
import Summerjob from "@/components/company/summerjob.vue";
import Layout from "@/components/company/Layout.vue";
import sideLayout from "@/views/sideLayout.vue";
import { computed, onMounted, onUnmounted } from "vue";
import { useFilterStore } from "@/stores/modules/filter";
import { useRoute, useRouter } from "vue-router";
import { usePrePagesStore } from "@/stores/modules/prepages";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useTagsStore } from "@/stores/modules/tags";
import { useMapsStore } from "@/stores/modules/maps";

const filterStore = useFilterStore();
const prepagesStore = usePrePagesStore();
const companiesStore = useCompaniesStore();
const tagsStore = useTagsStore();
const mapsStore = useMapsStore();

const route = useRoute();
const router = useRouter();

const company = computed(() => {
  return companiesStore.companyByName(route.params.name);
});

const currentIndex = computed(() => {
  return filterStore.filteredCompanies
    .map((x) => x.id)
    .indexOf(company.value.id);
});

const maxIndex = filterStore.filteredCompanies.length;

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler);
});

onUnmounted(() => {
  window.removeEventListener("keydown", arrowKeyHandler);
});

function arrowKeyHandler(e: any) {
  if (e.key == "ArrowRight") {
    next();
  } else if (e.key == "ArrowLeft") {
    prev();
  }
}

function next() {
  // this.$store.commit("layouts/updateCenter");
  const index = currentIndex.value + 1;
  if (index < filterStore.filteredCompanies.length) {
    router.push("/company/" + filterStore.filteredCompanies[index].name);
  }
}

function prev() {
  // this.$store.commit("layouts/updateCenter");
  const index = currentIndex.value - 1;
  if (index >= 0) {
    return router.push("/company/" + filterStore.filteredCompanies[index].name);
  } else {
    if (prepagesStore.active_prepages.length !== 0) {
      return router.push(
        "/prepages/" + (prepagesStore.active_prepages.length - 1)
      );
    }
  }
}
</script>
