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
      <template v-if="item.icon != ''">
        <v-img
          class="ml-2 mr-4"
          contain
          max-height="36px"
          max-width="36px"
          :src="'/api/manage/image/' + item.icon"
        />
      </template>
      {{ item.name }}
    </template>

    <template v-slot:selection="{ item }">
      <template v-if="item.icon != ''">
        <v-avatar>
          <v-img
            max-height="36px"
            max-width="36px"
            :src="'/api/manage/image/' + item.icon"
          />
        </v-avatar>
      </template>
      <template v-else>
        <v-chip small>
          {{ item.name }}
        </v-chip>
      </template>
    </template>
  </v-select>
</template>

<script>
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
};
</script>
