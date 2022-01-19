<template>
  <v-hover v-slot="{ hover }">
    <v-card center :to="to">
      <v-img contain center :src="background_image">
        <v-container
          id="hovereffect"
          fill-height
          :class="{ 'on-hover': hover, 'white--text': true, 'd-block': true }"
          :style="{
            'background-color': 'rgba(0, 0, 0, 0.35)',
          }"
        >
        </v-container>
      </v-img>
    </v-card>
  </v-hover>
</template>

<script>
export default {
  name: "hoverCard",
  props: ["to", "background_image"],
  data() {
    return { hover: false };
  },
};
</script>

<style scoped>
#hovereffect {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
</style>
