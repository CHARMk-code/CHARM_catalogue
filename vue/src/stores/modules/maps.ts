import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Company_Map {
  id: number;
  name: string;
  image: string;
  ref: number;
}

interface State {
  maps: Map<number, Company_Map>;
  load_wait: number;
}

export const useMapsStore = defineStore("maps", {
  state: (): State => ({
    maps: new Map(),
    load_wait: 0,
  }),
  actions: {
    setAllMaps(company_maps: Company_Map[]) {
      this.maps = new Map(company_maps.map((m) => [m.id, m]));
    },
    removeMapById(id: number) {
      this.maps.delete(id);
    },
    removeAllMaps() {
      this.maps = new Map();
    },
    getMaps() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/map")
            .then((resp: any) => {
              this.removeAllMaps();
              const maps = resp.data;
              if (maps.length > 0) {
                this.setAllMaps(maps);
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
    updateMap(map: Company_Map) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/map", map)
          .then((resp: any) => {
            this.maps.set(map.id, map);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    removeMap(map: Company_Map) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/map/" + map.id)
          .then((resp: any) => {
            this.removeMapById(map.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    getMapFromId: (state) => (id: number) => {
      return state.maps.get(id);
    },
  },
});
