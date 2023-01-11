<template>
  <company_card_wrapper name="logo" flat>
    <q-card-section>
      <Image
        v-if="src != ''"
        :imageName="src"
        height="150px"
        fit="contain"
        :position="$q.screen.lt.md ? '50% 50%' : 'left 50%'"
      />
    </q-card-section>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";
import Image from "../utils/Image.vue";

const props = defineProps<{
  src: string;
}>();
</script>
