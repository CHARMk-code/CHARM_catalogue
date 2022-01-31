<template>
  <v-container>
    <v-card>
      <v-card-title> Visible Elements </v-card-title>
      <v-combobox
        flat
        class="mx-4"
        v-model="selected"
        :filter="filter"
        :hide-nod-data="!search"
        :items="company_cards"
        :search-input.sync="search"
        hide-selected
        label="Select the elements should be visible for users"
        multiple
        solo
      >
        <template v-slot:selection="{ attrs, item, parent, selected }">
          <v-chip
            v-if="item === Object(item)"
            v-bind="attrs"
            :input-value="selected"
            label
            @click="parent.selectItem(item)"
          >
            <span class="pr-2">
              {{ item.text }}
            </span>
            <v-icon small @click="parent.selectItem(item)"> $delete </v-icon>
          </v-chip>
        </template>
        <template v-slot:item="{ index, item }">
          <v-chip label>
            {{ item.text }}
          </v-chip>
        </template>
      </v-combobox>
      <v-card-actions>
        <v-spacer />
        <v-btn large text @click="reset_popup = !reset_popup"> Reset </v-btn>
        <v-dialog persistent v-model="reset_popup" max-width="500px">
          <v-card absolute>
            <v-card-title> Reset to Default </v-card-title>
            <v-card-text>
              This will reset which company cards are shown to user to the
              default settings
              <v-card color="red lighten-4" v-if="error.length > 0">
                <v-card-title> Error: while reseting </v-card-title>
                <v-card-text> {{ error }} </v-card-text>
              </v-card>
            </v-card-text>
            <v-card-actions>
              <v-btn color="primary" :loading="reseting" @click="reset()">
                Reset
              </v-btn>

              <v-btn @click="reset_popup = false"> Cancel </v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
        <v-btn class="mx-2" large color="primary" @click="save()"> Save </v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script>
import { mapGetters } from "vuex";

export default {
  name: "Companies",
  components: {},
  data() {
    return {
      reset_popup: false,
      search: "",
      reseting: false,
      error: "",
    };
  },
  computed: {
    ...mapGetters({
      company_cards: "site_settings/getCompanyCards",
    }),
    selected: {
      get() {
        return this.company_cards.filter((c) => c.active);
      },
      set(new_cards) {
        this.$store.dispatch("site_settings/setCompanyCards", new_cards);
      },
    },
  },
  methods: {
    filter(item, queryText, itemText) {
      if (item.header) return false;

      const hasValue = (val) => (val != null ? val : "");

      const text = hasValue(itemText);
      const query = hasValue(queryText);

      return (
        text.toString().toLowerCase().indexOf(query.toString().toLowerCase()) >
        -1
      );
    },
    save() {
      this.$store.dispatch("site_settings/saveCompanyCards", this.selected);
    },
    reset() {
      this.reseting = true;
      this.$store
        .dispatch("site_settings/resetCompanyCards")
        .then(() => {
          this.reseting = false;
          this.reset_popup = false;
        })
        .catch((err) => {
          this.reseting = false;
          this.error = err;
        });
    },
  },
};
</script>
