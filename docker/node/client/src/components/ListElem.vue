<template>
  <div>
      <p>{{title}}</p>
    
    <div> Matched Tags </div>
    <div v-for="(item) in match" :key=item.name>
      <p style="color:red;"> {{item.name}}</p>
    </div>
    <div> Other Tags </div>
    <div v-for="(item) in noMatch" :key=item.name>
      <p>{{item.name}} </p>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  name: 'ListElem',
  data() {
    return {
      company_tags: [],
      match: [],
      noMatch:[],
    }
  },
  props: {
      title: String,
      selected_tags: Array,
      company_id: Number,
  },
  methods: {
        get_init() {
      axios.get('http://localhost:5000/api/tag/get?company_filter=' + this.company_id)
        .then((res) => {
          const match = []
          const noMatch = []
          this.company_tags = res.data;
          this. company_tags.forEach(tag => {
            if (this.selected_tags.includes(tag.id))
            {
              match.push({name:tag.name})
            } else {
              noMatch.push({name:tag.name})
            }
          });
          this.match = match.sort()
          this.noMatch = noMatch.sort()

        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
      }
    },
  created() {
    this.get_init();
  }, 
};
</script>
