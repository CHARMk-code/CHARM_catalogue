<template>
  <div ref="mapRoot" class="map" style="position: absolute" />
</template>

<script setup lang="ts">
import { Vector as VectorLayer } from "ol/layer.js";
import Map from "ol/Map.js";
import View from "ol/View.js";
import Feature from "ol/Feature.js";
import { Circle, Geometry } from "ol/geom.js";
import { Vector as VectorSource } from "ol/source.js";
import { onMounted, ref } from "vue";
import "ol/ol.css";
import Select from "ol/interaction/Select.js";
import { click } from "ol/events/condition.js";
import axios from "@/plugins/axios";
import ImageLayer from "ol/layer/Image";
import Static from "ol/source/ImageStatic";
import Projection from "ol/proj/Projection";
import { Style, Fill, Text } from "ol/style.js";
import type {
  FairMap,
  FairMapStyle,
  GeometryStyle,
  MapGeometry,
} from "../../stores/modules/fairMaps";
import { useFairMapsStore } from "../../stores/modules/fairMaps";
import { useCompaniesStore } from "@/stores/modules/companies";

const props = defineProps<{
  fairMapId: number;
}>();

const emit = defineEmits(["markerClicked"]);

const mapRoot = ref<string | HTMLElement | undefined>(undefined);
const companiesStore = useCompaniesStore();
const fairMapsStore = useFairMapsStore();

function setupMap(fairmap: FairMap): Map {
  const viewExtent = extentFromSize(fairmap.styling.mapSize.map((n) => n * 2));
  const viewProjection = new Projection({
    code: "FairMap",
    units: "pixels",
    extent: viewExtent,
  });

  const backgroundLayer = createBackgroundLayer(fairmap);

  const companyMarkers = fairmap.mapGeometry.filter(
    (geom: MapGeometry) => geom.type == "company"
  );

  const companyLayer = createCompanyLayer(companyMarkers, fairmap.styling);

  const map = new Map({
    layers: [backgroundLayer, companyLayer],
    view: new View({
      center: [0, 0],
      extent: viewExtent,
      projection: viewProjection,
      zoom: 1.5, // TODO: Define scale factor depending on background sizing
      maxZoom: fairmap.styling.maxZoom || 4,
    }),
  });

  const clickSelect = new Select({
    condition: click,
    style: function (feature, resolution) {
      const styling: GeometryStyle = feature.get("geomStyle");
      const mapStyling: FairMapStyle = feature.get("mapStyle");
      
      let backgroundColorClicked;
      if (styling.backgroundColorClicked) {
        backgroundColorClicked = styling.backgroundColorClicked;
      } else if (styling.backgroundColor) {
        backgroundColorClicked = [
          Math.max(styling.backgroundColor[0] - 80, 0),
          Math.max(styling.backgroundColor[1] - 80, 0),
          Math.max(styling.backgroundColor[2] - 80, 0),
          styling.backgroundColor[3],
        ];
      } else {
        backgroundColorClicked = [15, 15, 15, 1];
      }

      return new Style({
        text: new Text({
          font: styling.text?.font || "bold 22px sans-serif",
          textAlign: styling.text?.textAlign || "center",
          justify: styling.text?.justify || "center",
          text: feature.get("text"),

          scale: (mapStyling.circleSize || 24) / 24 / resolution,
          fill: new Fill({
            color: styling.text?.color || [0, 0, 0, 1],
          }),
        }),
        fill: new Fill({
          color: backgroundColorClicked,
        }),
      });
    },
  });

  map.addInteraction(clickSelect);
  clickSelect.on("select", (e) => {
    emit("markerClicked", e.selected[0]);
  });

  return map;
}

function createBackgroundLayer(fairmap: FairMap): ImageLayer<Static> {
  const extent = extentFromSize(fairmap.styling.mapSize);
  const projection = new Projection({
    code: "FairMap",
    units: "pixels",
    extent: extent,
  });

  return new ImageLayer({
    source: new Static({
      url: axios.defaults.baseURL + "/v2/image/" + fairmap.background,
      projection: projection,
      imageExtent: extent,
    }),
  });
}

function createCompanyLayer(
  companyMarkers: MapGeometry[],
  mapStyling: FairMapStyle
): VectorLayer<VectorSource<Geometry>> {
  const features = companyMarkers.map((geom) => {
    const styling = geom.styling;

    const company = companiesStore.companies.get(geom.refId);

    const text = company?.booth_number.toString();

    const feature = new Feature({
      geometry: new Circle(geom.position, mapStyling.circleSize || 24, "XY"), // TODO: Define scale factor depending on background sizing
    });

    feature.setStyle((feature, resolution): Style => {
      return new Style({
        text: new Text({
          font: styling.text?.font || "bold 22px sans-serif",
          textAlign: styling.text?.textAlign || "center",
          justify: styling.text?.justify || "center",
          text: text,

          scale: (mapStyling.circleSize || 24) / 24 / resolution,
          fill: new Fill({
            color: styling.text?.color || [0, 0, 0, 1],
          }),
        }),
        fill: new Fill({ color: styling.backgroundColor || "lightgray" }),
      });
    });

    feature.set("refId", geom.refId);
    feature.set("geomStyle", styling);
    feature.set("mapStyle", mapStyling);
    feature.set("text", text);
    return feature;
  });

  return new VectorLayer({
    source: new VectorSource({
      features,
    }),
    updateWhileAnimating: true,
    updateWhileInteracting: true,
  });
}

function extentFromSize(size: number[]): [number, number, number, number] {
  return [-size[0] / 2, -size[1] / 2, size[0] / 2, size[1] / 2];
}

const map = setupMap(fairMapsStore.fairMaps[props.fairMapId]);

onMounted(() => {
  map.setTarget(mapRoot?.value);
});
</script>

<style scoped lang="scss">
#mapLayoutContainer {
  height: calc(100vh - 90px);
}

.map {
  width: 100%;
  height: 100%;
}

#map {
  background: black;
}
</style>
