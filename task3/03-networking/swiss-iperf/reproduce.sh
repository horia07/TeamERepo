echo build swiss-iperf binaries
bash ./build.sh

echo build plot container
docker build -t teame/matplotlib -f Dockerfile.plot .

echo starting benchmark
python3 scripts/bench_iperf.py --time 15 --host fe80::e63d:1aff:fe72:f0 --interface swissknife1
# python3 scripts/bench_iperf.py --time 1 --host ::1

echo generating plots
docker run --rm -i -v $PWD/result:/data -v $PWD/scripts:/usr/src/app teame/matplotlib < result/data_latest.txt
