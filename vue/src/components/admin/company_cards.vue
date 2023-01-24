<template>
  <q-card>
    <q-card-section>
      <div class="text-h5">Company Page Layout</div>
      <div>
        Choose which cards should be displayed when looking at companies as a
        user
      </div>
    </q-card-section>
    <q-card-section>
      <q-select
        filled
        v-model="selected"
        :options="company_cards"
        option-label="name"
        option-value="id"
        label="Select the elements that should be visible for users"
        multiple
        use-chips
      >
      </q-select>
    </q-card-section>

    <q-card-actions>
      <q-btn label="Reset" @click="reset_popup = !reset_popup">
        <q-dialog v-model="reset_popup" width="500px">
          <q-card>
            <q-card-section class="row items-center">
              <div class="text-h5">Reset Company Page Layout?</div>
              This will reset which company cards are shown to user to the
              default settings. Are you sure?

              <div color="error" v-if="error.length > 0">
                <div class="text-bold">Error: while resetting</div>
                {{ error }}
              </div>
            </q-card-section>

            <q-card-actions :align="'right'">
              <q-btn flat label="Cancel" v-close-popup />
              <q-btn
                flat
                label="Reset"
                color="primary"
                @click="reset()"
                :loading="resetting"
              />
            </q-card-actions>
          </q-card>
        </q-dialog>
      </q-btn>
      <q-btn color="primary" label="Save" @click="save()" />
    </q-card-actions>
  </q-card>
</template>

<script lang="ts" setup>
import {
  useSite_settingsStore,
  type Card,
} from "@/stores/modules/site_settings";
import { computed, ref } from "vue";

let reset_popup = ref(false);
let search = "";
let resetting = ref(false);
let error = "";

let site_settingsStore = useSite_settingsStore();

const company_cards = site_settingsStore.settings.company_view.cards;

// const selected = ref([]);
const selected = computed<Card[]>({
  get: () => company_cards.filter((c) => c.active),
  set: (active_cards) => site_settingsStore.setCompanyCards(active_cards),
});

const filter = (item, queryText, itemText) => {
  if (item.header) return false;

  const hasValue = (val: string) => (val != null ? val : "");

  const text = hasValue(itemText);
  const query = hasValue(queryText);

  return (
    text.toString().toLowerCase().indexOf(query.toString().toLowerCase()) > -1
  );
};

const save = () => {
  useSite_settingsStore().saveCompanyCards();
};
const reset = () => {
  resetting.value = true;
  useSite_settingsStore()
    .resetCompanyCards()
    .then(() => {
      resetting.value = false;
      reset_popup.value = false;
    })
    .catch((err) => {
      resetting.value = false;
      error = err;
    });
};
</script>
