<template>
  <v-select
    multiple
    chips
    return-object
    item-text="name"
    item-value="id"
    @change="onChange"
    :label="label"
    :items="tags"
  >
    <template v-slot:item="{ item, attrs }">
      <v-simple-checkbox :value="attrs.inputValue" />
      <v-img
        class="ml-2 mr-4"
        contain
        max-height="36px"
        max-width="36px"
        :src="'/api/manage/image/' + item.icon"
      />
      {{ item.name }}
    </template>

    <template v-slot:selection="{ item }">
      <template v-if="item.icon != ''">
        <v-avatar>
          <v-img
            max-height="36px"
            max-width="36px"
            :src="base_URL + item.icon"
          />
        </v-avatar>
      </template>
      <template v-else>
        <v-chip small>
          {{ item.text }}
        </v-chip>
      </template>
    </template>
  </v-select>
</template>

<script>
import Vue from "vue";
export default {
  name: "Tag_selector",
  props: {
    value: Array,
    tags: { type: Array },
    label: { type: String },
  },
  methods: {
    onChange(v) {
      this.$emit("change", v);
    },
  },
  computed: {
    base_URL() {
      return Vue.prototype.$axios.defaults.baseURL + "/manage/image/";
    },
  },
};
</script>
