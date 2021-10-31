<template>
  <v-main>
    <v-sheet v-if="company != undefined">
      <v-container>
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
            <Maps />
          </v-col>
        </v-row>
        <v-row>
          <v-col>
            <BusinessAreas :areas="company.business_area" />
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
import BusinessAreas from "@/components/company/Business_area";
import Trivia from "@/components/company/Trivia";
import Contacts from "@/components/company/Contacts";
import Description from "@/components/company/Description";
import Website from "@/components/company/Website";

export default {
  name: "Company_View",
  data() {
    return {};
  },
  components: {
    //Art,
    Logo,
    BusinessAreas, //Tags?
    Trivia, //Did you know...
    Contacts, //name, email, position?
    Description, //Company description
    Website, //Company website
    //Offering, //Master thesis, summer job, Trainee, Oppotunities abroad, Internship, Recruiting events
    //Looking_for, //Bachelor, Master, Phd
    //Programs, // Tags for all programs
  },
  computed: {
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
  },
  beforeCreate() {
    this.$store.dispatch("companies/getCompanies"); //move somewhere else
    this.$store.dispatch("tags/getTags"); //move somewhere else
  },
};
</script>
