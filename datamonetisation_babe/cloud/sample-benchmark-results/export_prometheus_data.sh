#!/bin/bash


PROMETHEUS_URL=https://prometheus.unice.cust.tasfrance.com
PROMETHEUS_DB=metrics
DATA_PATH=datas

# rm -rf $DATA_PATH
mkdir -p $DATA_PATH

NOW=$(date +%s%N | cut -b1-13)
START_TS=$1 #1618796106358
END_TS=$2 #1618805531278 #$NOW
GROUP_BY_TIME="1s" #in seconds
TEST_NB=$3

####
query="avg(rate(substrate_sub_txpool_validations_finished[20s]))"
name="avg_finished_rate"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query="avg(substrate_sub_txpool_validations_finished)"
name="avg_tot_finished"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query="avg(rate(substrate_sub_txpool_validations_invalid[20s]))"
name="avg_invalid_rate"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query="histogram_quantile(0.90, sum(rate(substrate_proposer_block_constructed_bucket[5m])) by (le))"
name="block_construct_time"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query="avg(substrate_ready_transactions_number)"
name="avg_queue_size"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query="avg(substrate_number_leaves)"
name="avg_forks"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query='avg(rate(substrate_block_height{status="best"}[20s]))*60'
name="avg_block_per_min"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query='avg(rate(substrate_block_height{status="finalized"}[20s]))*60'
name="avg_finalized_block_per_min"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####
query='avg(rate(substrate_rpc_calls_total[5m]))'
name="avg_rpc_calls_rate"
echo "time,mean" > $DATA_PATH/${TEST_NB}_${name}.csv
curl -s --insecure -H "Content-Type: application/x-www-form-urlencoded" -XPOST \
    "$PROMETHEUS_URL/api/v1/query_range" \
    --data-urlencode "start=${START_TS::10}" \
    --data-urlencode "end=${END_TS::10}" \
    --data-urlencode "step=$GROUP_BY_TIME" \
    --data-urlencode  "query=$query" \
    | jq -r '.data.result[0].values' | jq -r '.[] | [.[]] | @csv' >> $DATA_PATH/${TEST_NB}_${name}.csv
####