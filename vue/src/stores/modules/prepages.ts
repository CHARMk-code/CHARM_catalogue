import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

interface Prepage {
  id: number,
  name: string,
  image: string,
  order: number,
  active: boolean,
}

interface State {
  prepages: Prepage[],
  active_prepages: Prepage[],
  load_wait: number,
}


export const usePrepagesStore = defineStore('prepages', {
  state: (): State => ({
    load_wait: 0,
    prepages: [],
    active_prepages: [],
  }),
  actions: {
    getPrepages() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/prepage")
            .then((resp: any) => {
              this.prepages = []
              const prepages: Prepage[] = resp.data;
              if (prepages.length > 0) {
                this.prepages = prepages
                let filtered_prepages = prepages.filter(
                  (page) => page.active && page.image != null
                );
                filtered_prepages.sort((a, b) => {
                  if (a.order > b.order) {
                    return 1;
                  } else if (a.order < b.order) {
                    return -1;
                  } else {
                    return 0;
                  }
                });
                this.active_prepages = filtered_prepages
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
    modifyPrepage(prepage: Prepage) {
      return new Promise((resolve, reject) => {
        console.log("prepage", prepage)
        this.axios
          .put("/prepage", prepage)
          .then((resp: any) => {
            if (!this.prepages.some((p) => (p.id = prepage.id))) {
              this.prepages.push(prepage);
            } else {
              this.prepages[this.prepages.findIndex((p) => p.id == prepage.id)];
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    deletePrepage(prepage: Prepage) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/prepage/" + prepage.id)
          .then((resp: any) => {
            this.prepages = this.prepages.filter((p) => p.id != prepage.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
