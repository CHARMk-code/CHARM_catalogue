<template>
  <v-hover v-slot="{ hover }">
    <v-card center :to="to">
      <v-img contain :max-width="max_width" center :src="background_image">
        <v-container
          id="hovereffect2"
          fill-height
          :class="{ 'on-hover': hover, 'white--text': true }"
          :style="{
            'background-color': 'rgba(0, 0, 0, 0.70)',
          }"
        >
          <slot />
        </v-container>
      </v-img>
    </v-card>
  </v-hover>
</template>

<script>
export default {
  name: "hoverCard",
  props: ["to", "background_image", "max_width"],
  data() {
    return { hover: false };
  },
};
</script>

<style scoped>
/*.v-card {
  transition: opacity 0.4s ease-in-out;
}

.v-card:not(.on-hover) {
  opacity: 0.4;
}*/
#hovereffect {
  transition: opacity 0.4s ease-in-out;
  opacity: 0.75;
}

#hovereffect:not(.on-hover) {
  opacity: 0;
}
#hovereffect2 {
  transition: opacity 0.4s ease-in-out;
}

#hovereffect2:not(.on-hover) {
  opacity: 0;
}
</style>
