FROM python 
RUN pip install matplotlib

RUN mkdir /data
RUN touch /data/bench.out
RUN touch /data/plot.png

COPY plot.py /src/plot.py


WORKDIR /data

CMD python /src/plot.py


