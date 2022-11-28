#!/bin/bash
#
# Use this script to generate a new release version of the project
# 
# It updtes the versio specified inside version.txt file
# It creates a new git tag
# It asks to push the new version to git repository
#


## Global colors
BLUE="\e[34m"
GREEN="\e[32m"
RED="\e[31m"
YELLOW="\e[33m"
RESET="\e[0m"

# GLOBAL VARIABLES
VERSION_TAG="" # new version tag

function help {
  echo -e "-h show help
-a add tag for the version to generate
  "
}

function validateTag {
  # Check if version matches pattern
  if ! [[ "${1}" =~  ^v[0-9]\.[0-9]\.[0-9]$ ]]; then
    echo -e "${YELLOW}Version parameter should follow the following pattern vx.x.x${RESET}";
    echo -e "${RED}Invalid tag ${1}${RESET}";
    exit 1;
  fi
}

function generateVersion {
  # Write version to main file
  echo -n -e "${BLUE}Generating ${VERSION_TAG} version${RESET} ... ";
  echo -n "${VERSION_TAG}" > "version.txt";
  # Add version file to commit
  git add version.txt;
  git commit -m "Version ${VERSION_TAG}"  1> /dev/null 2>&1;
  # Create a tag for git
  git tag -a "${VERSION_TAG}" -m "Version ${VERSION_TAG}"
  echo -e "[${GREEN}ok${RESET}]";
  
  echo -n -e "${BLUE}Should try to upload it to git?${RESET} (y/n): ";
  read -n 1 TRY_PUSH;

  if [ "${TRY_PUSH}" == "y" ]; then
    echo -n -e "\n${BLUE}Uploading tags${RESET} ... ";
    git push --tags 1> /dev/null 2>&1;
    echo -e "[${GREEN}ok${RESET}]";
  fi
}

while getopts ":hpa:" option; do
  case $option in 
    a)
      validateTag "${OPTARG}";
      VERSION_TAG="${OPTARG}";
      generateVersion;
      ;;
    h)
      help;
      ;;
    \?) # Invalid option
      echo -e "${RED}Invalid option${RESET}";
      help;
      exit ;;
  esac
done