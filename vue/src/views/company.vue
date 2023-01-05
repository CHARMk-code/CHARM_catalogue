<template>
  <q-page padding v-if="company != undefined && company.active">
    <div class="row">
      <Logo class="col-12 col-md-6" :src="company.logo" />

      <Name class="col-12 col-md-6" :name="company.name" :id="company.id" />
    </div>
    <div :class="$q.screen.lt.md ? 'row' : 'column'">
      <div class="flex-break hidden"></div>
      <Textblock :body="company.description" />
      <Textblock
        :body="company.unique_selling_point"
        title="What Makes Us Special"
      />
      <Summerjob
        name="summerjob"
        :desc="company.summer_job_description"
        :link="company.summer_job_link"
        :deadline="company.summer_job_deadline"
      />
      <Layout />
      <Tags
        :tags="tagsStore.getDivisionsFromIds(company.tags)"
        name="tag_divisions"
        title="Divisions"
      />
      <Tags
        :tags="tagsStore.getLookingForFromIds(company.tags)"
        name="tag_looking_for"
        title="Looking For"
        getter_target="tags/getLookingForFromIds"
      />
      <Tags
        :tags="tagsStore.getOfferingsFromIds(company.tags)"
        name="tag_offering"
        title="Offering"
        getter_target="tags/getOffersFromIds"
      />
      <Tags
        :tags="tagsStore.getBusinessAreasFromIds(company.tags)"
        name="tag_business_areas"
        title="Business Areas"
        getter_target="tags/getBusinessAreasFromIds"
      />
      <Website :website="company.website" />
      <Contacts
        :contacts="company.contacts"
        :contact_email="company.contact_email"
      />
      <Map
        :map="mapsStore.getMapFromId(company.map_image)"
        :booth_number="company.booth_number"
      />
      <Trivia
        :talk_to_us_about="company.talk_to_us_about"
        :sweden="company.employees_sweden"
        :world="company.employees_world"
      />
      <Note :id="company.id" />
    </div>
  </q-page>
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
import { computed, onMounted, onUnmounted } from "vue";
import { useFilterStore } from "@/stores/modules/filter";
import { useRoute, useRouter } from "vue-router";
import { usePrepagesStore } from "@/stores/modules/prepages";
import { useCompaniesStore } from "@/stores/modules/companies";
import { useTagsStore } from "@/stores/modules/tags";
import { useMapsStore } from "@/stores/modules/maps";

const filterStore = useFilterStore();
const prepagesStore = usePrepagesStore();
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
        "/prepage/" + (prepagesStore.active_prepages.length - 1)
      );
    }
  }
}
</script>

<style lang="sass" scoped>
@media (max-width: $breakpoint-md-min)
  .row > *
    width: 100%
    margin: 6px

  .logo
    width: 100%


@media (min-width: $breakpoint-md-min)
  $x: 2
  .column > *
    width: 50%
    margin: 6px
  .flex-break
    flex: 1 0 100% !important
    width: 0!important

  @for $i from 1 through ($x - 1)
    .column > div:nth-child(#{$x}n + #{$i})
      order: #{$i}

  .column > div:nth-child(#{$x}n)
    order: #{$x}
  .column
    height: 100vh
    margin-left: -24px
    padding: 16px
</style>
