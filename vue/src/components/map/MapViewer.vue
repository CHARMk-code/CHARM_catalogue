<template>
  <div ref="mapRoot" class="map"></div>
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
import axios from "@/plugins/axios";
import ImageLayer from "ol/layer/Image";
import Static from "ol/source/ImageStatic";
import Projection from "ol/proj/Projection";
import { Style, Fill, Text } from "ol/style.js";
import type { FairMap, MapGeometry } from "../../stores/modules/fairMaps";
import { useFairMapsStore } from "../../stores/modules/fairMaps";

const mapRoot = ref<string | HTMLElement | undefined>(undefined);

// MapLayer
const extent = [-500, -500, 500, 500]; // TODO: Define sane defaults here
const projection = new Projection({
  code: "SB-image",
  units: "pixels",
  extent: extent,
});

const fairMapsStore = useFairMapsStore();

function setupMap(fairmap: FairMap): Map {
  const backgroundLayer = createBackgroundLayer(fairmap);
  const companyMarkers = fairmap.mapGeometry.filter(
    (geom: MapGeometry) => geom.type == "company"
  );
  const companyLayer = createCompanyLayer(companyMarkers);

  return new Map({
    layers: [backgroundLayer, companyLayer],
    view: new View({
      center: [0, 0],
      extent: extent,
      projection: projection,
      zoom: 2, // TODO: Define scale factor depending on background sizing
    }),
  });
}

function createBackgroundLayer(fairmap: FairMap): ImageLayer<Static> {
  return new ImageLayer({
    source: new Static({
      url: axios.defaults.baseURL + "/v2/image/" + fairmap.background,
      projection: projection,
      imageExtent: extent,
    }),
  });
}

function createCompanyLayer(
  companyMarkers: MapGeometry[]
): VectorLayer<VectorSource<Geometry>> {
  const features = companyMarkers.map((geom) => {
    const styling = geom.styling;
    const feature = new Feature({
      geometry: new Circle(geom.position, 24, "XY"), // TODO: Define scale factor depending on background sizing
    });

    feature.setStyle((feature, resolution): Style => {

      return new Style({
        text: new Text({
          font: styling.text?.font || "bold 10px sans-serif",
          textAlign: styling.text?.textAlign || "center",
          justify: styling.text?.justify || "center",
          text: "100",
          scale: 2 / resolution,
          fill: new Fill({
            color: styling.text?.color || [0, 0, 0, 1],
          }),
        }),
        fill: new Fill({ color: styling.backgroundColor || "lightgray"}),
      });
    });
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

const map = setupMap(fairMapsStore.fairMaps[0]);

onMounted(() => {
  map.setTarget(mapRoot?.value);
});
</script>

<style>
.map {
  width: 500px;
  height: 400px;
}

#map {
  background: black;
}

.svg-layer path:hover {
  opacity: 0.4;
}
</style>
