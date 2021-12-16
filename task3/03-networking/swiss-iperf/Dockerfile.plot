FROM python

RUN python -m pip install matplotlib

WORKDIR /usr/src/app

CMD ["python", "plot.py"]
