<template>
  <q-page v-touch-swipe.left.right="handleSwipe" padding>
    <Filter
      @filter-changed="
        () => {
          setNextRoute()
          setPrevRoute()
        }
      "
    />
    <div v-if="company != undefined && company.active" class="row">
      <Logo class="col-12 col-md-6" :src="company.logo" />
      <Name :id="company.id" class="col-12 col-md-6" :name="company.name" />
    </div>

    <div
      style="
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-auto-rows: 140px;
      "
    >
      <component
        :is="component.component"
        v-for="(component, index) in available_components"
        :key="index"
        class="q-ma-sm"
        :style="{
          'grid-column': 'span ' + component.cols + ' / span ' + component.cols,
          'grid-row': 'span ' + component.rows + ' / span ' + component.rows,
        }"
      />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { useQuasar } from "quasar"
import { onMounted, watch, onUnmounted, computed, h } from "vue"
import { useFilterStore } from "@/stores/modules/filter"
import { usePrepagesStore, type Prepage } from "@/stores/modules/prepages"
import { useCompaniesStore } from "@/stores/modules/companies"
import { useTagsStore } from "@/stores/modules/tags"
import { useSite_settingsStore } from "@/stores/modules/site_settings"
import { useRoute, useRouter } from "vue-router"

import Logo from "@/components/company/Logo.vue"
import Name from "@/components/company/Name.vue"
import Textblock from "@/components/company/Textblock.vue"
import Tags from "@/components/company/Tags.vue"
import CompanyImage from "@/components/company/CompanyImage.vue"

const filterStore = useFilterStore()
const prepagesStore = usePrepagesStore()
const companiesStore = useCompaniesStore()
const tagsStore = useTagsStore()
const settingsStore = useSite_settingsStore()

const route = useRoute()
const router = useRouter()

const company = computed(() => {
  return companiesStore.companyByName((route.params.name as string) ?? "")
})

const available_components = computed(() => {
  return [
    {
      name: "image_office",
      component: h(CompanyImage, { image: company.value?.logo ?? "" }),
      cols: 1,
      rows: 2,
    },
    {
      name: "image_product",
      component: h(CompanyImage, { image: company.value?.logo ?? "" }),
      cols: 1,
      rows: 2,
    },
    {
      name: "our_words",
      component: h(Tags, {
        title: "Our words",
        tags: tagsStore.getTagsByCategoryFromIds(
          "Division",
          company.value?.tags ?? new Set([]),
        ),
      }),
      cols: 2,
      rows: 1,
    },
    {
      name: "divisions",
      component: h(Tags, {
        title: "Division",
        tags: tagsStore.getTagsByCategoryFromIds(
          "Division",
          company.value?.tags ?? new Set([]),
        ),
      }),
      cols: 1,
      rows: 1,
    },
    {
      name: "looking for",
      component: h(Tags, {
        title: "Looking For",
        tags: tagsStore.getTagsByCategoryFromIds(
          "Looking For",
          company.value?.tags ?? new Set([]),
        ),
      }),
      cols: 1,
      rows: 1,
    },
    { name: "icons with text", component: undefined, cols: 1, rows: 2 },
    {
      name: "offering",
      component: h(Tags, {
        title: "Offering",
        tags: tagsStore.getTagsByCategoryFromIds(
          "Offering",
          company.value?.tags ?? new Set([]),
        ),
      }),
      cols: 1,
      rows: 1,
    },
    {
      name: "our perks",
      component: h(Tags, {
        title: "Our Perks",
        tags: tagsStore.getTagsByCategoryFromIds(
          "Our Perks",
          company.value?.tags ?? new Set([]),
        ),
      }),
      cols: 1,
      rows: 1,
    },
    {
      name: "desc",
      component: h(Textblock, { body: company.value?.description ?? "" }),
      cols: 1,
      rows: 2,
    },
  ]
})

onMounted(() => {
  window.addEventListener("keydown", arrowKeyHandler)
  setNextRoute()
  setPrevRoute()
})

watch(
  () => route.params.name,
  (name) => {
    if (name) {
      setNextRoute()
      setPrevRoute()
    }
  },
)

onUnmounted(() => {
  window.removeEventListener("keydown", arrowKeyHandler)
})

function arrowKeyHandler(e: any) {
  if (e.key == "ArrowRight") {
    next()
  } else if (e.key == "ArrowLeft") {
    prev()
  }
}

function handleSwipe({ direction }) {
  if (direction === "right") {
    prev()
  } else if (direction === "left") {
    next()
  }
}

function setNextRoute() {
  // Set next route
  if (currentIndex.value + 1 < filterStore.filteredCompanies.length) {
    settingsStore.session_settings.navigation.next =
      "/company/" + filterStore.filteredCompanies[currentIndex.value + 1].name
  } else {
    settingsStore.session_settings.navigation.next = undefined
  }
}

const $q = useQuasar()

function setPrevRoute() {
  //set prev route
  if (currentIndex.value > 0) {
    settingsStore.session_settings.navigation.prev =
      "/company/" + filterStore.filteredCompanies[currentIndex.value - 1].name
  } else {
    if (Object.values(prepagesStore.pageGroups).length !== 0) {
      const pageGroups = Object.values(prepagesStore.pageGroups)
      let lastPageGroupIndex = pageGroups.length

      if ($q.screen.lt.md) {
        let mobilePages = pageGroups[lastPageGroupIndex - 1].pages.filter(
          (p: Prepage) => p.mobile,
        )
        while (mobilePages.length == 0) {
          lastPageGroupIndex -= 1
          mobilePages = pageGroups[lastPageGroupIndex - 1].pages.filter(
            (p: Prepage) => p.mobile,
          )
        }
        if (mobilePages.length > 1) {
          settingsStore.session_settings.navigation.prev =
            "/prepage/" + lastPageGroupIndex + "?p=" + (mobilePages.length - 1)
          return
        }
      }
      settingsStore.session_settings.navigation.prev =
        "/prepage/" + pageGroups.length
    } else {
      settingsStore.session_settings.navigation.prev = undefined
    }
  }
}

function next() {
  const maybeNext: string | undefined = settingsStore.consumeNext()
  if (maybeNext) {
    router.push(maybeNext)
  }
}

function prev() {
  const maybePrev: string | undefined = settingsStore.consumePrev()
  if (maybePrev) {
    router.push(maybePrev)
  }
}

// Logic for swipe instructions on mobile
// has saved thing in localStorage
if ($q.screen.lt.md && !(localStorage.getItem("swipeInstructions") || false)) {
  $q.dialog({ component: SwipeOverlay })
}
</script>
