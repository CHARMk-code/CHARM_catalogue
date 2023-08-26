<template>
  <div ref="mapRoot" class="map"></div>
</template>

<script setup lang="ts">
import { Vector as VectorLayer } from "ol/layer.js";
import Map from "ol/Map.js";
import View from "ol/View.js";
import Feature from "ol/Feature.js";
import { Circle } from "ol/geom.js";
import { Vector as VectorSource } from "ol/source.js";
import { onMounted, ref } from "vue";
import "ol/ol.css";
import axios from "@/plugins/axios";
import ImageLayer from "ol/layer/Image";
import Static from "ol/source/ImageStatic";
import Projection from "ol/proj/Projection";
import { Style, Fill, Text } from "ol/style.js";

const mapRoot = ref<string | HTMLElement | undefined>(undefined);

// Styling
const companyStyle = new Style({
    text: new Text({
      font: 'bold 9px sans-serif',
      textAlign: "center",
      justify: "center",
      text: "100",
      fill: new Fill({
        color: [0, 0, 0, 1],
      }),
    }),
    fill: new Fill({ color: 'red'}),
})


// Company Layer
function createCompanyPoint(company, position, color) {
  const company_feature = new Feature({
    geometry: position,
  });

  return company_feature;
}

const company_points = [
];
company_points.push(createCompanyPoint(undefined, new Circle([0, -100], 16, 'XY'), undefined));

const vecLayer = new VectorLayer({
  source: new VectorSource({
    features: company_points,
  }),
  style: function(feature, resolution) {
    console.log("feature", feature)
    companyStyle.getText().setScale(2 / resolution )
    return companyStyle;
  },
  updateWhileAnimating: true,
  updateWhileInteracting: true,
});



// MapLayer
const extent = [-500, -500, 500, 500];
const projection = new Projection({
  code: "SB-image",
  units: "pixels",
  extent: extent,
});

const imgLayer = new ImageLayer({
  source: new Static({
    url: axios.defaults.baseURL + "/v2/image/SB.png",
    projection: projection,
    imageExtent: extent,
  }),
});



// Map engine
const map = new Map({
  layers: [imgLayer, vecLayer],
  view: new View({
    center: [0, 0],
    extent: extent,
    projection: projection,
    zoom: 2,
  }),
});

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
