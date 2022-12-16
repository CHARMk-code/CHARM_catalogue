import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Card {
  id: number,
  name: string,
  text: string,
  active: boolean,
}


interface State {
  settings: {
    company_view: {
      cards: Card[]
    }
  },
  load_wait: number,
}


export const useSite_settingsStore = defineStore('site_settings', {
  state: (): State => ({
    settings: {
      company_view: {
        cards: [],
      },
    },
    load_wait: 0,
  }),
  actions: {
    getCompanyCards(force = false) {
      return new Promise<void>((resolve, reject) => {
        if (force || this.load_wait < Date.now()) {
          this.load_wait =
            Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/settings/company_view")
            .then((resp: any) => {
              this.settings.company_view.cards = resp.data
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
    setCompanyCards(cards: Card[]) {
      return new Promise<void>((resolve) => {
        const all_cards = this.settings.company_view.cards;

        const new_cards = all_cards.map((ac) => ({
          ...ac,
          active: cards.some((c) => ac.name === c.name) ? true : false,
        }));

        this.settings.company_view.cards = new_cards;
        resolve();
      });
    },
    saveCompanyCards() {
      return new Promise<void>((resolve, reject) => {
        const cards = this.settings.company_view.cards;
        this.axios
          .put("/settings/company_view", cards)
          .then(() => {
            resolve();
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    resetCompanyCards() {
      return new Promise((resolve, reject) => {
        this.axios
          .get("/settings/company_view/reset")
          .then(() => {
            resolve(this.getCompanyCards(true));
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
});
