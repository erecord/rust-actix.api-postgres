#!/bin/bash
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

psql -h localhost -p 5432 -U actix <  $SCRIPTPATH/database.sql