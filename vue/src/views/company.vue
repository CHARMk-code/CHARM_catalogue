<template>
  <v-main>
    <v-sheet v-if="company != undefined">
      {{ company.name }}
      <Logo :src="company.logo" />
      <BusinessAreas :areas="company.business_area" />
      <Trivia :trivia="company.trivia" />
      <Founded :year="company.founded" />
      <Contacts :contacts="company.contacts" />
      <Employees
        :sweden="company.employees_sweden"
        :world="company.employees_world"
      />
    </v-sheet>
  </v-main>
</template>

<script>
import Logo from "@/components/company/Logo";
import BusinessAreas from "@/components/company/Business_area";
import Trivia from "@/components/company/Trivia";
import Founded from "@/components/company/Founded";
import Contacts from "@/components/company/Contacts";
import Employees from "@/components/company/Employees";
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
    Founded, //year
    Contacts, //name, email, position?
    Employees, //Sweden worldwide
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
  },
};
</script>
