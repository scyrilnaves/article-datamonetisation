#!/bin/bash
my_dir="$(dirname "$0")"

cat <<EOF
apiVersion: v1
kind: List

items:

EOF

cat <<EOF
####################################### BENCHMARK MACHINE #########################

- apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: benchmark
    namespace: substrate-net
  spec:
    replicas: 1
    selector:
        matchLabels:
          name: benchmark-deployment
    template:
      metadata:
        labels:
          name: benchmark-deployment
          serviceSelector: benchmark-deployment
      spec:
        hostAliases:
        - ip: "185.52.32.4"
          hostnames:
          - "substrate-ws.unice.cust.tasfrance.com"
          - "vehicleapi.unice.cust.tasfrance.com"
          - "radarapi.unice.cust.tasfrance.com"
        containers:
        - name: auraconta
          image: docker:18.05-dind
          resources:
            limits:
              cpu: "10"
              memory: "20Gi"
            requests:
              cpu: "10"
              memory: "20Gi"
          env:
          - name: DOCKER_HOST
            value: tcp://localhost:2375
          securityContext:
            privileged: true
          volumeMounts:
            - name: dind-storage
              mountPath: /var/lib/docker
        volumes:
          - name: dind-storage
            emptyDir: {}
        
EOF
