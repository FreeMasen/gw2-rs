#! /bin/bash

make_request() {
    curl "https://api.guildwars2.com/v2/items?page=$1&page-size=50" -H "Authorization: Bearer $(cat ~/.config/gw2/api-key)" -s > "out/$1.json" &&\
    cat ./out.json | rg "type\": \"Tool"
}

for i in {0..1269}
do
    echo "Request $i"
    make_request $i
    sleep 0.3
done