import type { TextJustify } from "ol/style/Text";
import { defineStore } from "pinia";

const NUMBER_OF_MS_BEFORE_RELOAD = 60000; // Don't reload more often then ones an hour.

export interface FairMap {
  id: number;
  name: string;
  background: string;
  styling: FairMapStyle;
  mapGeometry: MapGeometry[];
}

export interface FairMapStyle {
  circleSize?: number;
  maxZoom?: number;
  mapSize: [number, number]
}

export interface MapGeometry {
  id: number;
  position: [number, number];
  type: "company" | "other";
  refId: number;
  styling: GeometryStyle;
}

type RGBA = [number, number, number, number];

export interface GeometryStyle {
  text?: {
    font?: string;
    textAlign?: CanvasTextAlign;
    justify?: TextJustify;
    color?: RGBA;
  };
  backgroundColor?: RGBA;
}

interface State {
  fairMaps: FairMap[];
  last_load: number;
}

const example: FairMap = {
  id: 1,
  name: "FairMap",
  background: "SU1.png",
  styling: {
    mapSize: [1526, 678],
  },
  mapGeometry: [
    { id: 1, position: [0, 0], type: "company", refId: 87, styling: {backgroundColor: [255,0,0,1]} },
  ],

};

export const useFairMapsStore = defineStore("fairMaps", {
  state: (): State => ({
    fairMaps: [example],
    last_load: 0,
  }),
  actions: {
    fetchMaps(force = false) {
      return new Promise<void>((res, rej) => {
        if (force || this.last_load + NUMBER_OF_MS_BEFORE_RELOAD < Date.now()) {
          this.last_load = Date.now();

          this.axios
            .get("/v2/map/")
            .then((resp: any) => {
              this.fairMaps = resp.data;
              res();
            })
            .catch((err: any) => {
              rej(err);
            });
        }
      });
    },
  },
});
