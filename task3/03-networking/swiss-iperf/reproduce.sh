echo build swiss-iperf binaries
bash ./build.sh

echo create virtual env
python3 -m venv venv/

echo activate virtual env
source venv/bin/activate

echo install requirements
pip install -r requirements.txt

echo install requirements
python3 scripts/bench_iperf.py --time 15 --host fe80::e63d:1aff:fe72:f0 --interface swissknife1

echo deactivate venv
deactivate

echo remove venv
rm -rf venv/
