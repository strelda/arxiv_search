#! /bin/bash

search_query="abs:state_driving OR abs:qubit OR abs:quantum_computing OR abs:quantum_computer OR abs:state_manifold"
since_date=$(date -d "2 days ago" +%Y-%m-%d)

./release/arxiv_search "$search_query" "$since_date" > localhost/main.html && 
http-server localhost/
