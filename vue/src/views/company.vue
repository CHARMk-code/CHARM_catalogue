<template>
  <sideLayout>
    <v-sheet v-if="company != undefined">
      <v-container>
        <!-- Edit Company -->
        <v-btn class="mb-4" v-on:click="editRow(company)" v-if="isLoggedIn">
          Edit
        </v-btn>
        <v-dialog persistent v-model="dialog" max-width="900px">
          <tableEditDialog
            @close_dialog="closeDialog()"
            @save_row="saveRow"
            name="Company"
            :row="company"
            :row_meta="row_meta"
          />
        </v-dialog>

        <v-row>
          <v-col>
            <Logo :src="company.logo" />
          </v-col>
          <v-col>
            <Name :name="company.name" :id="company.id" />
          </v-col>
        </v-row>

        <v-row>
          <v-col>
            <Textblock :body="company.description" class="mb-6" />
            <Layout class="mb-6" />
            <v-row>
              <v-col
                class="pa-0 mb-0 d-flex flex-wrap justify-space-between"
                style="margin: 0 -3px"
              >
                <Tags
                  :tags="company.tags"
                  name="Divisions"
                  getter_target="tags/getDivisionsFromIds"
                  class="mb-6 mx-3 flex-grow-1"
                />
                <Tags
                  :tags="company.tags"
                  name="Looking for"
                  getter_target="tags/getLookingForFromIds"
                  class="mb-6 mx-3 flex-grow-1"
                />
                <Tags
                  :tags="company.tags"
                  name="Offering"
                  getter_target="tags/getOffersFromIds"
                  class="mb-6 mx-3 flex-grow-1"
                />
                <Tags
                  :tags="company.tags"
                  name="Business Areas"
                  getter_target="tags/getBusinessAreasFromIds"
                  class="mb-6 mx-3 flex-grow-1"
                />
              </v-col>
            </v-row>
            <Website :website="company.website" class="mb-6" />
            <Contacts :contacts="company.contacts" />
          </v-col>
          <v-col>
            <Map :map="company.map_image" class="mb-6" />
            <Trivia
              :talk_to_us_about="company.talk_to_us_about"
              :sweden="company.employees_sweden"
              :world="company.employees_world"
              :year="company.founded"
              class="mb-6"
            />
            <Note :id="company.id" class="mb-6" />
          </v-col>
        </v-row>
      </v-container>
    </v-sheet>
  </sideLayout>
</template>

<script>
import Logo from "@/components/company/Logo";
import Name from "@/components/company/Name";
import Trivia from "@/components/company/Trivia";
import Contacts from "@/components/company/Contacts";
import Textblock from "@/components/company/Textblock";
import Website from "@/components/company/Website";
import Tags from "@/components/company/Tags";
import tableEditDialog from "@/components/admin/table_edit_dialog";
import Note from "@/components/company/Note";
import Map from "@/components/company/Map";
import Layout from "@/components/company/Layout";
import sideLayout from "@/views/sideLayout";
import { mapGetters } from "vuex";

export default {
  name: "Company_View",
  data() {
    return { dialog: false };
  },
  components: {
    sideLayout,
    Name,
    Logo,
    Trivia, //Did you know...
    Contacts, //name, email, position?
    Textblock, //Company description
    Website, //Company website
    Tags, //Tags
    //Maps, //Map view
    tableEditDialog,
    Note,
    Map,
    Layout,
  },
  computed: {
    ...mapGetters({
      isLoggedIn: "auth/isLoggedIn",
      filteredCompanies: "filter/filteredCompanies",
      divisions: "tags/divisions",
      looking_for: "tags/looking_fors",
      business_areas: "tags/business_areas",
      offerings: "tags/offers",
      isInFavorites: "favorites/isInFavorites",
    }),
    company() {
      const matching_companies = this.$store.getters["companies/companyByName"](
        this.$route.params.name
      );
      if (matching_companies.length == 1) {
        console.log("match");
        return matching_companies[0];
      } else {
        console.log("No match");
        return undefined;
      }
    },
    currentIndex() {
      return this.filteredCompanies.map((x) => x.id).indexOf(this.company.id);
    },
    maxIndex() {
      return this.filteredCompanies.length;
    },
    tags() {
      return this.company.tags;
    },
    row_meta() {
      return [
        {
          type: "checkbox",
          model: "active",
          on_icon: "mdi-eye",
          off_icon: "mdi-eye-off",
          label: "Active (required for row to be visible)",
        },
        {
          type: "text",
          model: "name",
          label: "Company name",
          displayname: true,
        },
        { type: "image", model: "logo", label: "Company Logo" },
        {
          type: "textarea",
          model: "description",
          label: "Company description",
        },
        { type: "text", model: "founded", label: "Founded" },
        { type: "text", model: "Contacts", label: "Contacts" },
        { type: "text", model: "website", label: "Website" },
        {
          type: "text",
          model: "employees_sweden",
          label: "Number of Employees in Sweden",
        },
        {
          type: "text",
          model: "employees_world",
          label: "Number of Employees in the whole world",
        },
        { type: "text", model: "trivia", label: "Trivia" },
        {
          type: "select",
          model: "divisions",
          items: this.divisions,
          label: "Divisions",
          hint: "Programs the company are interested in",
        },
        {
          type: "select",
          model: "looking_for",
          items: this.looking_for,
          label: "Looking For",
          hint: "Which level of education the company is looking for",
        },
        {
          type: "select",
          model: "business_area",
          items: this.business_areas,
          label: "Business areas",
          hint: "The companys business areas",
        },
        {
          type: "select",
          model: "offering",
          items: this.offerings,
          label: "offering",
          hint: "Which type of jobs the company is offering",
        },
      ];
    },
  },
  methods: {
    editRow(row) {
      this.editedRow = row;
      this.creating = false;
      this.dialog = true;
    },
    closeDialog() {
      this.dialog = false;
      this.creating = true;
      this.editedRow = {};
    },
    saveRow(row) {
      this.$store.dispatch("companies/modifyCompany", row);
    },
    next() {
      this.$router.push(
        "/company/" + this.filteredCompanies[this.currentIndex + 1].name
      );
    },
    prev() {
      this.$router.push(
        "/company/" + this.filteredCompanies[this.currentIndex - 1].name
      );
    },
  },
};
</script>
