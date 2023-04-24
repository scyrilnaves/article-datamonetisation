#!/bin/sh

cd "$(dirname "$0")"

echo "JAVA BUILD STARTED"
mvn clean package -DskipTests -Dhttps.protocols=TLSv1.2
