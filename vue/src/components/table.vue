<template>
  <v-data-table 
    :headers="headers"
    :items="items"
    class="elevation-1">
     
  </v-data-table>
</template>

<script>
export default {
  name: "Table",
  data () {
    return {
      headers: [
        {text: "Name", value: "name"},
        {text: "description", value: "desc"},
        {text: "employees", value: "employ"},
      ],
      items: [
        {
          name: "Volvo",
          desc: "A car company",
          empl: "a lot",
        }
      ]
    }
  }

}
</script>
