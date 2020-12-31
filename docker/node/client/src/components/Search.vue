<template>
  <div>
    <div class="container">
      <div class="row">
        <div class="col-sm-10">
          <multiselect v-model="value" tag-placeholder="Add this as new tag"
          placeholder="Search" label="name" track-by="id"
          :options="options" :multiple="true" :taggable="true"
          ></multiselect>
          <p>
            Ignore crowd sourced data
            <input type="checkbox" id="checkbox" v-model="checked">
          </p>
          <button type="button" v-on:click = "search()" class="btn btn-primary">Search</button>

        </div>
      </div>
      <div class="row">
        <ul id="example-2">
          <!-- eslint-disable-next-line -->
          <div v-for="(item) in results">
            <listelem 
            v-bind:title = item.name
            v-bind:selected_tags  = all_selected_tag
            v-bind:company_id = item.id
            >
            </listelem>
          </div>
        </ul>
      </div>
    </div>
  </div>
</template>

<script>
import Multiselect from 'vue-multiselect';
import ListElem from './ListElem.vue'
import axios from 'axios';

export default {
  name: 'Search',
  components: {
    Multiselect,  
  },
  data() {
    return {
      checked: false,
      options: [],
      value: [],
      company: [],
      tag: [],
      results: [],
      all_selected_tag: [],
    };
  },
  methods: {
    get_init() {
      axios.get('http://localhost:5000/api/company/get')
        .then((res) => {
          this.company = res.data;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
      axios.get('http://localhost:5000/api/tag/get')
        .then((res) => {
          res.data.forEach((tag)=>
          {
            this.options.push({name: tag.name, id: tag.id});
          })
          this.tag = res.data;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
    search() {
      let selected =  this.value;
      let all_tag = {};

      // Helper function to get all relevant tags
      function rec_tag(id,tag){
        let local_tags = {};
        local_tags[id] = "";
        // Recurisvly find tag in the same group
        let rec_tags = tag.filter((r) => r.parent_tag == id);
        rec_tags = rec_tags.map((a) => rec_tag(a.id,tag));

        // Merge result of recurision
        rec_tags.forEach((subresult) => {
            local_tags = Object.assign({},local_tags,subresult);
        })
        return local_tags;
      }
      selected = selected.map((a) => rec_tag(a.id,this.tag));

      // Merge result of recurision
      selected.forEach((subresult) => {
          all_tag = Object.assign({},all_tag,subresult);
      })

      // Generats  query string
      this.all_selected_tag = []
      let query_string = "[";
      Object.keys(all_tag).forEach((id) => {
        query_string += String(id) + ",";
        this.all_selected_tag.push(parseInt(id));
      })
      query_string += "]";
      if (this.checked)
      {
        query_string += "&crowd=2"
      }

      axios.get('http://localhost:5000/api/tag/match?tags=' + query_string)
        .then((res) => {
          const ret = {};
          res.data.forEach((value) => {
            const key = value[0];
            ret[key] = {
              id: key,
              count: ret[key] && ret[key].count ? ret[key].count
              + value[2] / value[1] : value[2] / value[1],
            };
          });
          const resultArray = [];
          // eslint-disable-next-line
          Object.keys(ret).forEach((key) => {
            // Weighting bas on score
            resultArray.push([ret[key].id, ret[key].count]);
          });

          // eslint-disable-next-line
          resultArray.sort((a, b) => {
            // eslint-disable-next-line
            return (a[0] > b[0]) ? -1 : (a[0] < b[0]) ? 1 : 0;
          });
          // Render results
          // eslint-disable-next-line
          this.results = resultArray.map((a) => {  
            const filtered = this.company.filter((r) => r.id === a[0]);
            return filtered[0];
          });
        }).catch((error) => {
        // eslint-disable-next-line
          console.error(error);
        });
    },
  },
  created() {
    this.get_init();
  },
};
</script>
