<template>
  <company_card_wrapper name="logo" flat>
    <q-card-section>
      <Image
        v-if="src != ''"
        :image-name="src"
        :height="height ?? '150px'"
        fit="contain"
        :position="$q.screen.lt.md ? '50% 50%' : 'left 50%'"
      />
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import Image from "../utils/Image.vue";

defineProps<{
  src: string;
  height: string;
}>();
</script>
