FROM python

RUN python -m pip install matplotlib

WORKDIR /usr/src/app

COPY ./plot.py .

CMD ["python", "plot.py"]
