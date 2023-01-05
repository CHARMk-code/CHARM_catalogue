import { defineStore } from "pinia"
import dayjs from "dayjs"

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface Company {
  id: number,
  last_updated: Date,
  active: boolean,
  charmtalk: boolean,
  name: string,
  description: string,
  unique_selling_point: string,
  summer_job_description: string,
  summer_job_link: string,
  summer_job_deadline: string, //Should maybe be date
  contacts: string,
  contact_email: string,
  employees_world: number,
  employees_sweden: number,
  website: string,
  talk_to_us_about: string,
  logo: string,
  map_image: number,
  booth_number: number,
  tags: Set<number>,
}


interface State {
  companies: Map<number, Company>,
  load_wait: number
}

export const useCompaniesStore = defineStore('companies', {
  state: (): State => ({
    companies: new Map(),
    load_wait: 0,
  }),
  actions: {
    setAllCompanies(companies: Company[]) {
      this.companies = new Map(companies.map(c => [c.id, c]))
    },
    removeCompanyById(id: number) {
      this.companies.delete(id)
    },
    removeAllCompanies() {
      this.companies.clear();
    },
    fetchCompanies() {
      return new Promise<void>((resolve, reject) => {
        if (this.load_wait < Date.now()) {
          this.load_wait = Date.now() + NUMBER_OF_MS_BEFORE_RELOAD;
          this.axios
            .get("/company")
            .then((resp: any) => { //TODO Remove this any and replace with actual type
              this.removeAllCompanies()
              const companies: Company[] = resp.data;

              // Work around to get summer job deadline in correct format
              companies.forEach(comp => {
                comp.summer_job_deadline =
                  dayjs(comp.summer_job_deadline, "DD MMM YYYY HH:mm:ss", false).format("YYYY-MM-DD");
                comp.tags = new Set(comp.tags)
              });

              if (companies.length > 0) {
                this.setAllCompanies(companies)
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
    updateCompany(company: Company) {
      return new Promise((resolve, reject) => {
        this.axios
          .put("/company", company)
          .then((resp: any) => {
            this.companies.set(company.id, company)
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
    removeCompany(company: Company) {
      return new Promise((resolve, reject) => {
        this.axios
          .delete("/company/" + company.id)
          .then((resp: any) => {
            this.removeCompanyById(company.id);
            resolve(resp);
          })
          .catch((err: any) => {
            reject(err);
          });
      });
    },
  },
  getters: {
    companyByName: (state) => (name: string) => {
      const matching = Array.from(state.companies.values()).filter((c) => c.name == name);
      if (matching.length > 0) {
        return matching[0]
      }
    },
  },
});
