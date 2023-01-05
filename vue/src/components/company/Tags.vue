<template>
  <company_card_wrapper :name="name">
    <q-card-section>
      <div class="text-h6">{{ title }}</div>
      <div>
        <template v-if="tags.length === 0">
          <span>No {{ title }}</span>
        </template>
        <template v-else>
          <TagGroup :tags="tags"></TagGroup>
        </template>
      </div>
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import axios from "@/plugins/axios";
import type { Tag } from "@/stores/modules/tags";
import TagGroup from "../Tag_group.vue";

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  title: string;
  name: string;
  tags: Tag[];
}>();
</script>
