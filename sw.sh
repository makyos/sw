#!/bin/bash

while true; do
	echo "$(tput cuu1)$(tput dl1)"
	sleep 0.1
	echo -n "$(date)"
done

