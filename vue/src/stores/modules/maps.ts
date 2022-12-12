import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

interface Company_Map {
  id: number,
  name: string,
  image: string,
  ref: number,
}

interface State {
  maps: Company_Map[],
  load_wait: number,

}


export const useMapsStore = defineStore('maps', {
  state: (): State => ({
    maps: [],
    load_wait: 0,
  }),
  actions: {
    setMaps(maps: Company_Map[]) {
      this.maps = maps;
    },
    removeMap(id: number) {
      this.maps = this.maps.filter((p) => p.id != id);
    },

    removeAllMaps() {
      this.maps = [];
    },
    getMaps() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          console.log(this.axios)
          this.axios
            .get("/map")
            .then((resp: any) => {
              this.removeAllMaps();
              const maps = resp.data;
              if (maps.length > 0) {
                this.setMaps(maps)
              }
              resolve(resp);
            })
            .catch((err: any) => {
              reject(err);
            });
        } else {
          resolve();
        }
      });
    },
    modifyMap(map: Company_Map) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/map", map)
          .then((resp: any) => {
            if (!this.maps.some((p) => (p.id = map.id))) {
              this.maps.push(map);
            } else {
              this.maps[this.maps.findIndex((p) => p.id == map.id)];
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deleteMap(map: Company_Map) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/map/" + map.id)
          .then((resp: any) => {
            this.removeMap(map.id)
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    get: (state) => {
      return state.maps;
    },
  },
});
