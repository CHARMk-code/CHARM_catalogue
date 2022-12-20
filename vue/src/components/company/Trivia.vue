<template>
  <company_card_wrapper
    name="didyouknow"
    v-if="!(talk_to_us_about == '' && sweden == -1 && world == -1)"
  >
    <v-card-title> Did you know...</v-card-title>
    <v-card-text>
      <v-card flat v-if="talk_to_us_about != ''">
        <v-card-title class="text-subtitle-1 ma-0 pa-0">
          Talk to us about:
        </v-card-title>
        <v-card-text>
          {{ talk_to_us_about }}
        </v-card-text>
      </v-card>

      <v-card flat v-if="world != -1">
        <v-card-title class="text-subtitle-1 ma-0 pa-0">
          Employees:
        </v-card-title>
        <v-card-text>
          {{ sweden }} in sweden and {{ world }} in the world
        </v-card-text>
      </v-card>
    </v-card-text>
  </company_card_wrapper>
</template>

<script lang="ts" setup>
import company_card_wrapper from "@/components/company/card_wrapper.vue";

const props = defineProps<{
  talk_to_us_about: string;
  sweden: number;
  world: number;
}>();
</script>
