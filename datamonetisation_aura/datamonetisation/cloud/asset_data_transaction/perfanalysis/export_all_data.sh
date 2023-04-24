#!/bin/bash

DATA_PATH=datas
NB_MAX_TESTS_TO_DETECT=01 #use two digit format

echo "Remove previous results"
rm -rf ./datas_csv/*
rm -rf ./datas_csv_paper_ready/*
rm -rf $DATA_PATH

GRAFANA_URL="http://grafana.unice.cust.tasfrance.com/api/annotations"
GRAFANA_DASHBOARD_ID="2"
function get_annotations {
    curl -s -H "Content-Type: application/json" \
        -X GET \
        -u admin:admin1234 \
        "$GRAFANA_URL?dashboardId=$GRAFANA_DASHBOARD_ID&type=annotation&limit=1000&from=$1&to=$2&tags=$3" #&tags=send_tx
    echo ""
}

echo "Get metadata timestamps"
#list of start-stop timestamps, generated using the annotation in grafana
#select all annotation start/end sending tx
start_id_list=($(get_annotations $1 $2 "send_tx" | jq -r -s '.[] | sort_by(.id)' | jq -r '.[] | (.id)'))
start_text_list=($(get_annotations $1 $2 "send_tx" | jq -r -s '.[] | sort_by(.id)' | jq -r '.[] | (.text)'))
start_time_list=($(get_annotations $1 $2 "send_tx" | jq -r -s '.[] | sort_by(.id)' | jq -r '.[] | (.time)'))
end_time_list=($(get_annotations $1 $2 "end_send_tx" | jq -r -s '.[] | sort_by(.id)' | jq -r '.[] | (.time)'))
length=$(get_annotations $1 $2 "end_send_tx" | jq -r length)

echo "Number of tests detected: $length"
for ((i = 0; i <= $(($length - 1)); i++)); do
    # echo "${start_id_list[i]} ${start_time_list[i]} ${end_time_list[i]}"
    #parse to get key=value
    tps=$(echo "${start_text_list[i]}" | grep -o 'tps=[^,]*')
    nodes=$(echo "${start_text_list[i]}" | grep -o 'nodes=[^,]*')
    test=$(echo "${start_text_list[i]}" | grep -o 'test=[^,]*')
    total=$(echo "${start_text_list[i]}" | grep -o 'total=[^,]*')
    #get only value
    tps=$(printf "%s\n" "${tps##*=}")
    nodes=$(printf "%s\n" "${nodes##*=}")
    test=$(printf "%02d\n" "${test##*=}")
    total=$(printf "%s\n" "${total##*=}")

    # echo "${tps}tps|${nodes}_nodes|$test"

    #we have a test
    #export data
    ./export_prometheus_data.sh ${start_time_list[i]} ${end_time_list[i]} $test

    #if 10 tests we can build final data
    if [ "$test" == "$NB_MAX_TESTS_TO_DETECT" ]; then
        #we have a test end
        end=${end_time_list[i]}
        echo "##########################################################"
        echo "We have $NB_MAX_TESTS_TO_DETECT tests for the config: ${tps}tps|${nodes}_nodes"
        #We can now build data using .py script
        ./build_data.py "${tps}tps|${nodes}_nodes" #name the test
        ./build_data_merged.py "${nodes}_nodes"
        ./final.py
        # exit
        #remove tmp data after build
        rm -rf $DATA_PATH
    fi

done

# echo "Start compiling all results"

# echo "5_nodes"
# ./export_prometheus_data.sh 1627477423726 1627480602869
# ./build_data.py "1000tps|5_nodes" #name the test

# echo "12_nodes"

# echo "18_nodes"

# echo "24_nodes"

echo "End compiling all results"
