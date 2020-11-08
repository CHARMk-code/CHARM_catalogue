FROM python:3

WORKDIR /anyass

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

ENTRYPOINT ["python init_db.py"]

CMD [ "flask", "run", "--host=0.0.0.0"]
