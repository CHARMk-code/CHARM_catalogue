import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Prepage {
  id?: number,
  name: string,
  image: string,
  active: boolean,
  mobile: boolean,
  side: "left" | "right",
  page: number,

}
export interface PrepageGroup {
  id: number,
  pages: Prepage[]
}
interface State {
  pageGroups: { [id: number]: PrepageGroup },
  inactivePrepages: Prepage[],
  load_wait: number,
}

function isValidPrepage(state: State, prepage: Prepage) {
  if (!prepage.active) {
    state.inactivePrepages.some(ip => ip.id == prepage.id)
    return true
  }
  if (state.pageGroups[prepage.page]) {
    if (prepage.side === "left" && !(state.pageGroups[prepage.page].pages[0].id === prepage.id)) {
      return true
    }
    if (prepage.side === "right" && !(state.pageGroups[prepage.page].pages[1].id === prepage.id)) {
      return true
    }
  }
  return false
}

export const usePrepagesStore = defineStore('prepages', {
  state: (): State => ({
    load_wait: 0,
    pageGroups: {},
    inactivePrepages: [],
  }),
  actions: {
    getPrepages(force?: boolean) {
      return new Promise<void>((resolve, reject) => {
        if (force || this.load_wait < Date.now()) {
          this.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/prepage")
            .then((resp: any) => {
              this.pageGroups = []
              const prepages: Prepage[] = resp.data;
              if (prepages.length > 0) {
                prepages.forEach(prepage => {
                  if (!prepage.active) {
                    this.inactivePrepages.push(prepage)
                  } else {
                    if (this.pageGroups[prepage.page] != undefined) {
                      if (prepage.side === "left") {
                        this.pageGroups[prepage.page].pages.unshift(prepage)
                      }
                      if (prepage.side === "right") {
                        this.pageGroups[prepage.page].pages.push(prepage);
                      }
                    } else {
                      this.pageGroups[prepage.page] = { id: prepage.page, pages: [prepage] }
                    }
                  }
                })

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
        if (prepage.id && !isValidPrepage(this, prepage)) {
          reject()
        }

        this.axios
          .put("/prepage", prepage)
          .then((resp: any) => {
            if (!prepage.id) {
              this.getPrepages(true); //Force reload of prepages (no ID is returned from backend upon creation)
            } else {
              if (prepage.side === "left") {
                this.pageGroups[prepage.page].pages[0]
              }
              if (prepage.side === "right") {
                this.pageGroups[prepage.page].pages[1]
              }
            }
            resolve(resp)
          })
          .catch((err: any) => {
            reject(err);
          });
      })
    },
    deletePrepage(prepage: Prepage) {
      return new Promise((resolve, reject) => {
        if (!prepage.id || !isValidPrepage(this, prepage)) {
          reject()
        }

        this.axios
          .delete("/prepage/" + prepage.id)
          .then((resp: any) => {
            const pagesInGroup = this.pageGroups[prepage.page].pages
            if (pagesInGroup.length === 1) {
              this.pageGroups[prepage.page].pages = []
            } else if (pagesInGroup.length === 2) {
              if (prepage.side === "left") {
                this.pageGroups[prepage.page].pages.splice(0, 1)
              }
              if (prepage.side === "right") {
                this.pageGroups[prepage.page].pages.splice(1, 1)
              }
            }
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
