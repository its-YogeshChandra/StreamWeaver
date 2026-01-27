#!/bin/bash

#variable for the renewal script
echo "running renewal script"

#infinite while loop in the file
while true; do
  #the main command
  certbot renew --quiet
  sleep 12h
done
