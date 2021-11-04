nix-shell -p wrk --command "wrk -t 8 -c 10 http://ryan.dse.in.tum.de:8081" 
nix-shell -p wrk --command "wrk -t 8 -c 100 http://ryan.dse.in.tum.de:8081"
nix-shell -p wrk --command "wrk -t 8 -c 1000 http://ryan.dse.in.tum.de:8081"
nix-shell -p wrk --command "wrk -t 8 -c 10000 http://ryan.dse.in.tum.de:8081"
