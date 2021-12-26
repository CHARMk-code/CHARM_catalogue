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
          <v-tooltip right max-width="300px">
            <template v-slot:activator="{ on, attrs }">
              <div v-on="on">
                <v-checkbox
                  class="large"
                  v-bind="attrs"
                  @change="favoriteChange"
                  v-model="favorite"
                  on-icon="mdi-star"
                  off-icon="mdi-star-outline"
                />
              </div>
            </template>
            <span>
              Save as favourite (This is only stored in your browser and will
              not be transfered between browsers)
            </span>
          </v-tooltip>
          <v-col>
            <div class="text-h2 font-weight-regular">{{ company.name }}</div>
          </v-col>
          <v-col> </v-col>
        </v-row>
        <v-row>
          <v-col>
            <Textblock :body="company.description" />
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
              :talk_to_us_about="company.talk_to_us_about"
              :sweden="company.employees_sweden"
              :world="company.employees_world"
              :year="company.founded"
            />
            <Note :id="company.id" />
          </v-col>
          <v-col>
            <Contacts class="mb-6" :contacts="company.contacts" />
            <Website :website="company.website" />
            <Map :map="company.map_image" />
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
import Textblock from "@/components/company/Textblock";
import Website from "@/components/company/Website";
import Tags from "@/components/company/Tags";
import tableEditDialog from "@/components/admin/table_edit_dialog";
import Note from "@/components/company/Note";
import Map from "@/components/company/Map";
import { mapGetters } from "vuex";

export default {
  name: "Company_View",
  data() {
    return { dialog: false, favorite: false };
  },
  components: {
    //Art,
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
    favoriteChange() {
      if (this.favorite) {
        this.$store.commit("favorites/addFavorite", this.company.id);
      } else {
        this.$store.commit("favorites/removeFavorite", this.company.id);
      }
    },
  },
  watch: {
    company: function (company) {
      this.favorite = this.$store.getters["favorites/favorites"].has(
        company.id
      );
    },
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

