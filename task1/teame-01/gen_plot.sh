echo "running benchmark"
./benchmark.sh | grep Requests/sec | awk '{print $2}' > bench.out

echo "building docker image"
docker build -t teame/plot -f plot.Dockerfile .

echo "creating plot.png file"
touch plot.png

echo "generate plots"
docker run --rm -it --volume ${PWD}/bench.out:/data/bench.out --volume ${PWD}/plot.png:/data/plot.png teame/plot
