import 'bootstrap/dist/css/bootstrap.css';
import BootstrapVue from 'bootstrap-vue';
import Vue from 'vue';
import App from './App.vue';
import router from './router';
import Multiselect from 'vue-multiselect';
import ListElem from './components/ListElem.vue'


Vue.use(BootstrapVue);
Vue.component('multiselect', Multiselect)
Vue.component('listelem', ListElem)
Vue.config.productionTip = false;

new Vue({
  router,
  render: h => h(App),
}).$mount('#app');
