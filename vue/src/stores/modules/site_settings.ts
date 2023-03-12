import { defineStore } from "pinia";
import { unref } from "vue";
import { setCssVar } from "quasar";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Card {
  name: string;
  text: string;
  active: boolean;
}

interface State {
  settings: {
    company_view: {
      cards: Card[];
    };
    navigation: {
      next: string | undefined;
      prev: string | undefined;
    };
    theme: {
      logo: string;
      primary: string;
    };
  };
  load_wait: number;
}

export const useSite_settingsStore = defineStore("site_settings", {
  state: (): State => ({
    settings: {
      company_view: {
        cards: [
          { text: "Logo", name: "logo", active: true },
          { text: "Name", name: "name", active: true },
          { text: "Description", name: "desc", active: true },
          { text: "Did you know", name: "didyouknow", active: true },
          { text: "Divisions", name: "tag_divisions", active: true },
          { text: "Business Areas", name: "tag_business_areas", active: true },
          { text: "Offering", name: "tag_offering", active: true },
          { text: "Looking for", name: "tag_looking_for", active: true },
          { text: "Website", name: "website", active: true },
          { text: "Map", name: "map", active: true },
          { text: "Summer job", name: "summerjob", active: true },
          { text: "Notes", name: "notes", active: true },
          { text: "CHARMtalks", name: "CHARMtalks", active: true },
          { text: "Language", name: "language", active: true },
          { text: "Fair Areas", name: "fair_area", active: true },
        ],
      },
      navigation: {
        next: undefined,
        prev: undefined,
      },
      theme: {
        logo: "prepage0.png",
        primary: "#d60000",
      },
    },
    load_wait: 0,
  }),
  actions: {
    saveSettings() {
      console.log("test");
      return new Promise<void>((resolve, reject) => {
        this.axios
          .put("/settings/site", { name: "settings", blob: this.settings })
          .then(() => {
            resolve();
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    fetchSettings(force = false) {
      return new Promise<void>((res, rej) => {
        if (force || this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/settings/site")
            .then((resp: any) => {
              if (resp.data.name === "settings") {
                this.settings = JSON.parse(resp.blob);
                setCssVar("primary", this.settings.theme.primary);
                res(resp);
              }
              rej("Didn't receive settings blob");
            })
            .catch((err: any) => {
              rej(err);
            });
        }
      });
    },
    setCompanyCards(active_cards: Card[]) {
      return new Promise<void>((resolve) => {
        const all_cards = this.settings.company_view.cards;
        all_cards.forEach((card) => {
          if (
            active_cards.some((active_card) => active_card.name === card.name)
          ) {
            card.active = true;
          } else {
            card.active = false;
          }
        });

        resolve();
      });
    },
    consumeNext() {
      const temp = unref(this.settings.navigation.next);
      this.settings.navigation.next = undefined;
      return temp;
    },
    consumePrev() {
      const temp = unref(this.settings.navigation.prev);
      this.settings.navigation.prev = undefined;
      return temp;
    },
  },
});
