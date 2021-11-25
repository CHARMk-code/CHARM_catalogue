<template>
  <v-main>
    <v-sheet v-if="company != undefined">
      <v-btn
        class="prev navigation"
        v-on:click="prev()"
        v-if="currentIndex > 0"
        icon
      >
        <v-chip x-large>
          <v-icon x-large>mdi-arrow-left</v-icon>
        </v-chip>
      </v-btn>

      <v-btn
        class="next navigation"
        v-on:click="next()"
        v-if="currentIndex < maxIndex - 1"
        icon
      >
        <v-chip x-large>
          <v-icon x-large>mdi-arrow-right</v-icon>
        </v-chip>
      </v-btn>
      <v-container>
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
          <v-spacer />
          <v-col>
            <div class="text-h2 font-weight-regular">{{ company.name }}</div>
          </v-col>
        </v-row>
        <v-row>
          <v-col>
            <Description :desc="company.description" />
          </v-col>
          <v-col>
            <!--<Maps />-->
          </v-col>
        </v-row>
        <v-row>
          <v-col>
            <!-- <BusinessAreas class="mb-6" :areas="company.business_area" /> -->
            <Tags
              :tags="company.tags"
              name="Tags"
              getter_target="tags/getTagsFromIds"
            />
            <Tags
              :tags="company.tags"
              name="Business Areas"
              getter_target="tags/getBusinessAreasFromIds"
            />
            <Tags
              :tags="company.tags"
              name="Looking for"
              getter_target="tags/getLookingForFromIds"
            />
            <Tags
              :tags="company.tags"
              name="Divisions"
              getter_target="tags/getDivisionsFromIds"
            />
            <Tags
              :tags="company.tags"
              name="Offering"
              getter_target="tags/getOffersFromIds"
            />
          </v-col>
          <v-col>
            <Trivia
              :trivia="company.trivia"
              :sweden="company.employees_sweden"
              :world="company.employees_world"
              :year="company.founded"
            />
          </v-col>
          <v-col>
            <Contacts class="mb-6" :contacts="company.contacts" />
            <Website :website="company.website" />
          </v-col>
        </v-row>
      </v-container>
    </v-sheet>
  </v-main>
</template>

<script>
import Logo from "@/components/company/Logo";
// import BusinessAreas from "@/components/company/Business_area";
import Trivia from "@/components/company/Trivia";
import Contacts from "@/components/company/Contacts";
import Description from "@/components/company/Description";
import Website from "@/components/company/Website";
import Tags from "@/components/company/Tags";
import tableEditDialog from "@/components/admin/table_edit_dialog";
import { mapGetters } from "vuex";

export default {
  name: "Company_View",
  data() {
    return { dialog: false };
  },
  components: {
    //Art,
    Logo,
    //BusinessAreas, //Tags?
    Trivia, //Did you know...
    Contacts, //name, email, position?
    Description, //Company description
    Website, //Company website
    Tags, //Tags
    //Offering, //Master thesis, summer job, Trainee, Oppotunities abroad, Internship, Recruiting events
    //Looking_for, //Bachelor, Master, Phd
    //Programs, // Tags for all programs
    //Maps, //Map view
    tableEditDialog,
  },
  computed: {
    ...mapGetters({
      isLoggedIn: "auth/isLoggedIn",
      filteredCompanies: "filter/filteredCompanies",
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
          items: this.tags
            .filter((t) => t.division)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Divisions",
          hint: "Programs the company are interested in",
        },
        {
          type: "select",
          model: "looking_for",
          items: this.tags
            .filter((t) => t.looking_for)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Looking For",
          hint: "Which level of education the company is looking for",
        },
        {
          type: "select",
          model: "business_area",
          items: this.tags
            .filter((t) => t.business_area)
            .map((t) => ({ text: t.name, value: t.id })),
          label: "Business areas",
          hint: "The companys business areas",
        },
        {
          type: "select",
          model: "offering",
          items: this.tags
            .filter((t) => t.offering)
            .map((t) => ({ text: t.name, value: t.id })),
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
  beforeCreate() {
    this.$store.dispatch("companies/getCompanies"); //move somewhere else
    this.$store.dispatch("tags/getTags"); //move somewhere else
    this.$store.dispatch("filter/filterCompanies");
  },
};
</script>
<style scoped>
.navigation {
  text-decoration: none;
  margin: 20px;
  position: absolute;
  top: 50%;
  z-index: 9999;
}
.navigation > * {
  top: -50%;
}
.next {
  right: 5%;
}
</style>

