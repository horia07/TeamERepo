echo build plot container
docker build -t teame/matplotlib -f Dockerfile.plot .

echo starting benchmark

if [[ $1 == "--local" ]]; then
    python3 scripts/bench_iperf.py --time 10 --host ::1
else
    python3 scripts/bench_iperf.py --time 15 --host fe80::e63d:1aff:fe72:f0 --interface swissknife1
fi

echo generating plots
docker run --rm -i -v $PWD/result:/data -v $PWD/scripts:/usr/src/app teame/matplotlib < result/data_latest.txt
