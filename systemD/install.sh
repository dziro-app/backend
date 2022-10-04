#!/bin/bash

## Global variables
WD="" # Working directory
APP="" # App directory

## Global colors
BLUE="\e[34m"
GREEN="\e[32m"
RED="\e[31m"
YELLOW="\e[33m"
RESET="\e[0m"


function die {
  echo -e "💣 ${RED} ${1} ${RESET}";
  exit 1;
}

echo -e "⌨️${GREEN}  Provide the working directory: ${RESET}";
read WD;

# Validate if working dir exists does not exist
if ! [[ -d $WD ]]; then
  die "Directory ${WD} does not exist."
fi


echo -e "⌨️${GREEN}  Provide the app directory: ${RESET}";
read APP;

# Validate if file does not exist
if ! [[ -s $APP ]]; then
  die "File ${APP} does not exist."
fi

echo -e "⚙️ ${YELLOW} Generating service for Dziro ... ${RESET}";

# Use sed with pipe delimiter to avoid collisions with path slash
cat ./systemD/dziro.service | 
  sed "s|\$WD|$WD|g" |
  sed "s|\$APP|$APP|g";