#!/usr/bin/env bash
 
COLOR_WHITE=$(tput setaf 7);
COLOR_MAGENTA=$(tput setaf 5);
FONT_BOLD=$(tput bold);
FONT_NORMAL=$(tput sgr0);

echo
echo -e "$COLOR_WHITE $FONT_BOLD Information $FONT_NORMAL";
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD View System Info, Hostname, Timezone $COLOR_WHITE $FONT_NORMAL";
echo
hostnamectl set-hostname scon
timedatectl set-timezone 'Australia/Sydney'
uname -a
echo
echo -e "  $COLOR_MAGENTA $FONT_BOLD View TCP Ports with listening daemons $COLOR_WHITE $FONT_NORMAL";
echo
netstat -ltn
echo
# https://help.ubuntu.com/community/IptablesHowTo
echo -e "  $COLOR_MAGENTA $FONT_BOLD View IP Tables and Firewall status $COLOR_WHITE $FONT_NORMAL";
echo
iptables -L -n
