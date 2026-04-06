#!/bin/bash

# Description
## Creates an entry in `events` folder with the date of the event

HOME=$(dirname $0);

if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    echo "Usage: log.sh [options]"
    echo "options:"
    echo "  -h, --help: Show this help message and exit"
    echo "  -d, --date: Set the date in YYYY-MM-DD format, defaults to today, ie: 2024-06-01"
    exit 0
fi

DATE=$(date +%Y-%m-%d);

if [[ "$1" == "-d" || "$1" == "--date" ]]; then
  # Check if the date is in the correct format
  if [[ ! "$2" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    echo "Error: Date must be in YYYY-MM-DD format"
    exit 1
  fi
  DATE=$2;
  echo $2 >> $HOME/events/entries.txt
else
  echo $DATE >> $HOME/events/entries.txt
fi
