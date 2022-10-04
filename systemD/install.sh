#!/bin/bash

## Global variables
SYSTEMDDIR="/etc/systemd/system"
WD="$(pwd)" # Working directory
APP="" # App directory
USER="$(whoami)" # User to run

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

echo -n -e "⌨️${GREEN}  Provide the user that will run the app${RESET} [default:${USER}]: ";
read USERI;
if ! [[ -z $USERI ]]; then
  id $USERI 2>&1 > /dev/null;
  if [ $? == "0" ]; then
    USER=$USERI;
  else
    die "User ${USERI} not found";
  fi
fi

echo -n -e "⌨️${GREEN}  Provide the working directory${RESET} [default:${WD}]: ";
read WDI;
if ! [[ -z $WDI ]]; then
  # Validate if working dir exists does not exist
  if ! [[ -d $WDI ]]; then
    die "Directory ${WD} does not exist."
  else
    WD=$WDI;
  fi
fi

echo -n -e "⌨️${GREEN}  Provide the app directory: ${RESET}";
read APP;
# Validate if file does not exist
if ! [[ -s $APP ]]; then
  die "File ${APP} does not exist."
fi

echo -e "⚙️ ${YELLOW} Generating service for Dziro ... ${RESET}";

# Use sed with pipe delimiter to avoid collisions with path slash
outFile=$(cat ./systemD/dziro.service | 
  sed "s|\$WD|$WD|g" | # replace working direcotry
  sed "s|\$USER|$USER|g" | # replace user
  sed "s|\$APP|$APP|g"); # replace application to execute

echo -e "${BLUE}${outFile}${RESET}";

echo -n -e "${GREEN}⌨️  Should try to installit under ${SYSTEMDDIR}${RESET} [default:y]: ";
read INSTALLI;

if [ -z $INSTALLI ] || [ $INSTALLI == "y" ] ; then
  echo -e "⚙️ ${YELLOW} Installing service for Dziro ... ${RESET}";
  echo $outFile > "${SYSTEMDDIR}/dziro.service";
fi

echo -e "⚙️ ${YELLOW} Remeber to enable the service to start after reboot using ${GREEN} systemctl enable dziro.service ${RESET}"