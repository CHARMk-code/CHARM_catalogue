<template>
  <q-select
    class="q-ma-sm"
    filled
    multiple
    :options="tags"
    v-model="selected"
    :label="label"
    clearable
  >
    <template #option="{ opt, itemProps }">
      <q-item v-bind="itemProps">
        <q-item-section avatar v-if="opt.icon && opt.icon.length > 0">
          <Tag_group :tags="[opt]"></Tag_group>
        </q-item-section>
        <q-item-section> {{ opt.name }}</q-item-section>
      </q-item>
    </template>

    <template #selected-item="{ index, opt }">
      <Tag_group v-if="index < props.maxShown" :tags="[opt]"></Tag_group>
      <span
        v-if="index === props.maxShown"
        class="text-grey text-caption align-self-center"
        >(+ {{ selected.length - maxShown }} others)</span
      >
    </template>
  </q-select>
</template>

<script lang="ts" setup>
import axios from "@/plugins/axios";
import { useTagsStore, type Tag } from "@/stores/modules/tags";
import { onMounted, ref, watch, type Ref } from "vue";
import Tag_group from "@/components/Tag_group.vue";

const tagsStore = useTagsStore();

const base_URL = axios.defaults.baseURL + "/manage/image/";

const props = defineProps<{
  tags: Tag[];
  selected_tags: Tag[];
  label: string;
  maxShown: number;
}>();

const emit = defineEmits<{
  (e: "update:selected_tags", v: Tag[]): void;
  (e: "change"): void;
}>();

const selected: Ref<Tag[]> = ref([]);
selected.value = props.selected_tags;

watch(selected, (tags: Tag[]) => {
  emit("update:selected_tags", tags);
});
</script>
