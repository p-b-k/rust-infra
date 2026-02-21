#!/bin/bash
########################################################################################################################
# Start the appropriate server
########################################################################################################################

export PATH=$PATH:/cplane
export SLEEP_SECS=${SLEEP_SECS:-15}

echo "Starting Database ..."
service mariadb start || exit -1
echo "... Database started"

echo "Sleeping for $SLEEP_SECS seconds ..."
sleep $SLEEP_SECS

echo "Starting Control Plane Service ..."
cp-svr --port 8000 --db-name sample
