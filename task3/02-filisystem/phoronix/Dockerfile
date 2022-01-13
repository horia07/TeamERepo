FROM phoronix/pts


WORKDIR /usr/src/app


RUN apt-get install sudo 
RUN sudo apt-get  update && apt-get install -y make libz-dev gcc g++ libtool perl wget python2 libaio-dev php-cli php-xml python3 bash unzip zlib1g python librdmacm1 libibverbs1 libc6 libaio1


COPY ./phoronix-test-suite.xml /etc/

COPY ./run.sh /usr/share/phoronix-test-suitpts-core/hooks/startup

COPY ./run.sh .
COPY ./rename.sh .

RUN ["chmod", "+x", "run.sh"]
RUN ["chmod", "+x", "rename.sh"]

CMD ["bash", "run.sh"]
