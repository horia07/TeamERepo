sudo nix-env -iA nixos.maven
sudo nix-env -iA nixos.jdk
sudo nix-env -iA nixos.pythonFull

echo "------------------------------------------------"
echo "startin in-memory kv store (redis)"
echo "------------------------------------------------"

docker-compose up -d

git clone http://github.com/brianfrankcooper/YCSB.git
cd YCSB

echo "------------------------------------------------"
echo "binding redis to YCSB framework"
echo "------------------------------------------------"

mvn -pl site.ycsb:redis-binding -am clean package

echo "------------------------------------------------"
echo "loading the workload"
echo "------------------------------------------------"

sudo ./bin/ycsb load redis -s -P workloads/workloada -p "redis.host=172.28.1.4"

echo "------------------------------------------------"
echo "running the workload"
echo "------------------------------------------------"

sudo ./bin/ycsb run redis -s -P workloads/workloada -p "redis.host=172.28.1.4" > output1.txt
sudo ./bin/ycsb run redis -s -P workloads/workloada -p "redis.host=172.28.1.4" > output2.txt
sudo ./bin/ycsb run redis -s -P workloads/workloada -p "redis.host=172.28.1.4" > output3.txt
sudo ./bin/ycsb run redis -s -P workloads/workloada -p "redis.host=172.28.1.4" > output4.txt
sudo ./bin/ycsb run redis -s -P workloads/workloada -p "redis.host=172.28.1.4" > output5.txt

cd ..
mv ./YCSB/output1.txt ./plotting/results/
mv ./YCSB/output2.txt ./plotting/results/
mv ./YCSB/output3.txt ./plotting/results/
mv ./YCSB/output4.txt ./plotting/results/
mv ./YCSB/output5.txt ./plotting/results/



echo "------------------------------------------------"
echo "closing kv store container"
echo "------------------------------------------------"

docker-compose down
cd mariadb

echo "------------------------------------------------"
echo "building mariadb with rocksdb as storage engine"
echo "------------------------------------------------"

docker build -t "mariadb" .
echo "------------------------------------------------"
echo "opening mariadb"
echo "------------------------------------------------"

docker run --name mariadb -d -p 3306:3306 mariadb

git clone http://github.com/brianfrankcooper/YCSB.git
cd YCSB

echo "------------------------------------------------"
echo "binding mariadb to YCSB framework"
echo "------------------------------------------------"

sudo mvn -pl site.ycsb:rocksdb-binding -am clean package

echo "------------------------------------------------"
echo "loading the workload"
echo "------------------------------------------------"

sudo ./bin/ycsb load rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data

echo "------------------------------------------------"
echo "running the workload"
echo "------------------------------------------------"

sudo ./bin/ycsb run rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data > output6.txt
sudo ./bin/ycsb run rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data > output7.txt
sudo ./bin/ycsb run rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data > output8.txt
sudo ./bin/ycsb run rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data > output9.txt
sudo ./bin/ycsb run rocksdb -s -P workloads/workloada -p rocksdb.dir=/tmp/ycsb-rocksdb-data > output10.txt




cd ..
mv ./YCSB/output6.txt ../plotting/results/
mv ./YCSB/output7.txt ../plotting/results/
mv ./YCSB/output8.txt ../plotting/results/
mv ./YCSB/output9.txt ../plotting/results/
mv ./YCSB/output10.txt ../plotting/results/



echo "------------------------------------------------"
echo "closing mariadb"
echo "------------------------------------------------"


docker stop mariadb

cd ../plotting
bash gen_plot
