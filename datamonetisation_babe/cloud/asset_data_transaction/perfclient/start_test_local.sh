#!/bin/bash

#RUN example:
#
#./big_test.sh <RANCHER TOKEN> > logs/test-"`date +"%Y-%m-%d-%T"`".log
#

GRAFANA_URL="http://grafana.unice.cust.tasfrance.com/api/annotations"
GRAFANA_DASHBOARD_ID="3"

JS_THREADS=28

#key=tps
#we want an avg of 1 min test, ie 60 * tps
# arr_tests=(100 500 800 1000 1100 1300 1500)
# arr_tests_nodes=(4 6 12 18 24)
arr_tests_nodes=(5 7 9 10 12 15 20 25) #always odd
# arr_tests_tps=(1000 1100 1200 1300 1400 1500 1600)
arr_tests_tps=(200 400 600 800 1000 1200 1400 1600 2000 2500)
# should be minimum 30!!!!!
# should match and above js_threads
tot_admins=30

prod=false
total_tx=30000
iteration=1

bootnode=0

function send_annotation {
    curl -s -H "Content-Type: application/json" \
        -X POST \
        -u admin:admin1234 \
        -d "{\"tags\":[\"tests\", \"$5\"], \"dashboardId\":$GRAFANA_DASHBOARD_ID, \"text\":\"nodes=$4,tps=$1,total=$2,test=$3,admin=$tot_admins,threads=$JS_THREADS\"}" \
        $GRAFANA_URL
    echo ""
}

for nb_nodes in "${arr_tests_nodes[@]}"; do

    for tps in "${arr_tests_tps[@]}"; do

        #100000 #$(($tps * 60))
        if $prod; then
            send_annotation "${tps}" "$total_tx" "${i}" "$(($nb_nodes - $bootnode))" "init_network"

            ####################### Main Cloud Deploy #################################""""""

            cd ../cloud-deployments
            ./delete-substrate-net.sh $1
            sleep 10
            ./genNodeYaml.sh $nb_nodes >substrate-kube.yaml
            ./deploy-substrate.sh $1
            sleep 180

            send_annotation "${tps}" "$total_tx" "${i}" "$(($nb_nodes - $bootnode))" "end_init_network"
        fi
        ###################### End Cloud Deploy ####################################################
        echo "Script executed from: ${PWD}"

        if $prod; then
            send_annotation "${tps}" "$total_tx" "${i}" "$(($nb_nodes - $bootnode))" "init_tx"
        fi
        echo "generate identities admins=$tot_admins"
        if $prod; then
            ./cmd_remote.sh "./genAccounts.js $tot_admins" | sed -r "s/\x1B\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g"
        else
            # 1) Remember to install dependencies
            # 2) cd /Documents/code/substrate/datamarket-new/datamarket/cloud/data-transaction-js
            # 3) npm install
            ./genAccounts.js $tot_admins
        fi
        #sed to remove colors

        echo "send init identities"
        if $prod; then
            ./cmd_remote.sh "./init_v2.js" | sed -r "s/\x1B\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g"

            send_annotation "${tps}" "$total_tx" "${i}" "$(($nb_nodes - $bootnode))" "end_init_tx"
        else
            # 1) Remember to install dependencies
            # 2) cd /Documents/code/substrate/datamarket-new/datamarket/cloud/data-transaction-js
            # 3) npm install
            ./init_v2.js
        fi

        sleep 20 #wait finalisation
        echo "itr"
        echo $iteration

        for i in {1..${iteration}}; do #repeat 5 times the test

            echo "################### TEST tps=$tps n°$iteration #######################"

            if $prod; then

                send_annotation "${tps}" "$total_tx" "${iteration}" "$(($nb_nodes - $bootnode))" "send_tx"

            fi

            echo "send tx"
            if $prod; then
                ./cmd_remote.sh "./send_v2.js $total_tx $tps $JS_THREADS" | sed -r "s/\x1B\[([0-9]{1,3}(;[0-9]{1,2})?)?[mGK]//g"
            else
                # 1) Remember to install dependencies
                # 2) cd /Documents/code/substrate/datamarket-new/datamarket/cloud/data-transaction-js
                # 3) npm install
                ./send_v2.js $total_tx $tps $JS_THREADS
            fi

            sleep 30

            if $prod; then

                send_annotation "${tps}" "$total_tx" "${iteration}" "$(($nb_nodes - $bootnode))" "end_send_tx"

            fi

            echo "################### END TEST n°$iteration #######################"

            sleep 180
        done

    done

done
