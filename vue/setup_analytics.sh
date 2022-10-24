#!/bin/bash

if [[ -n "${ANALYTICS_DOMAIN}" ]]; then # Is ANALYTICS_DOMAIN set?
  echo "Analytics domain set to: ${ANALYTICS_DOMAIN}"
  sed -i -E "s/<script defer data-domain=.*?plausible.js\"><\/script>//" /dist/index.html;
  sed -i "s#<head>#<head><script defer data-domain=\"$ANALYTICS_DOMAIN\" src=\"https://analytics.glimfjord.com/js/plausible.js\"></script>#" /dist/index.html
fi
