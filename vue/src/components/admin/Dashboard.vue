<template>
  <div>
    <v-col align-self="center">
      <v-container justify="center">
        <v-sheet>
          <v-list>
            <v-row>
              <template v-for="link in links">
                <v-col :key="link.n">
                  <v-card :key="link.name" :to="routepath + link.name">
                    <v-card-content>
                      <v-list-item-icon>
                        <v-icon color="primary" v-text="link.icon"></v-icon>
                      </v-list-item-icon>
                      <v-list-item-text class="text-capitalize">
                        {{ link.name }}
                      </v-list-item-text>
                    </v-card-content>
                  </v-card>
                </v-col>
                <v-responsive
                  v-if="link.n === 2"
                  :key="`width-${link.n}`"
                  width="100%"
                ></v-responsive>
              </template>
            </v-row>
          </v-list>
        </v-sheet>
      </v-container>
    </v-col>
  </div>
</template>

<script>
// @ is an alias to /src
export default {
  name: "Dashboard",
  components: {},
  props: ["links", "routepath"],
};
</script>
