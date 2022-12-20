<template>
  <v-autocomplete
    multiple
    chips
    item-title="name"
    item-value="id"
    v-model="selected"
    :label="label"
    :items="tags"
    :maxWidth="400"
    :menuProps="{ maxHeight: 300 }"
    clearable
    persistent-clear
    variant="outlined"
  >
    <template v-slot:item="{ item, props }">
      <v-list-item v-bind="props">
        <template v-if="item.raw.icon" #prepend>
          <TagComp :tag="item.raw"></TagComp>
        </template>
      </v-list-item>
    </template>

    <template v-slot:chip="{ index, item }">
      <TagComp v-if="index < props.maxShown" :tag="item.raw"></TagComp>
      <span
        v-if="index === props.maxShown"
        class="text-grey text-caption align-self-center"
        >(+ {{ selected_tagIds.length - maxShown }} others)</span
      >
    </template>
  </v-autocomplete>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { computed, isReadonly, readonly, ref, type Ref } from "vue";
import TagComp from "@/components/Tag.vue";

const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  tags: Tag[];
  selected_tagIds: number[];
  label: string;
  maxShown: number;
}>();

const emit = defineEmits<{
  (e: "update:selected_tagIds", v: number[]): void;
  (e: "change"): void;
}>();

function printnow(a) {
  console.log(a);
}

const selected = computed({
  get() {
    return props.selected_tagIds;
  },
  set(v) {
    emit("update:selected_tagIds", v);
    emit("change");
  },
});
</script>
