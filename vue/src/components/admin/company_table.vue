<template>
  <v-container> 
    <Table :headers='headers' :data="this.$store.companies"/>
  </v-container>
</template>

<script>
import Table from "@/components/table"

export default {
  name: "companies_table",
  components: {
    Table,
  },
  data() {
    return {
      headers: [
        {text: 'Name', value: 'name'},
        {text: 'Website', value: 'website'},
        {text: 'Tags', value: 'tags'},
        {text: 'Active', value: 'active'},
        {text: 'Actions', value: 'actions', sortable: false },
      ],
      data: [
        {
          id: 0,
          active: 'true',
          name: 'Volvo',
          desc: 'this is quite a long sentence since it is meant to be the whole description for the company. However it is not very descriptive, mostly from a lack of information about the actual company.',
          founded: '1903',
          contact: 'person@volvogroup.com',
          website: 'https://volvogroup.com',
          employees_sweden: '22450',
          employees_world: '45000',
          trivia: 'This is not a true fun fact/trivia',
          tags: ['a', 'd', 'e', 'f', 'h', 'i','k','m'],
        }
      ]
  }
  }
}
</script>



