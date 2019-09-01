#!/usr/bin/env bash
 
COLOR_WHITE=$(tput setaf 7);
COLOR_MAGENTA=$(tput setaf 5);
FONT_BOLD=$(tput bold);
FONT_NORMAL=$(tput sgr0);

echo
echo -e "$COLOR_WHITE $FONT_BOLD Docker Information $FONT_NORMAL";
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD Docker Containers $COLOR_WHITE $FONT_NORMAL";
echo
docker ps -a
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD Docker Images $COLOR_WHITE $FONT_NORMAL";
echo
docker images
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD Docker Machine information $COLOR_WHITE $FONT_NORMAL";
echo
echo -e "    docker inspect <CONTAINER_ID>";
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD Docker Logs $COLOR_WHITE $FONT_NORMAL";
echo
echo -e "    docker logs <CONTAINER_ID>";
