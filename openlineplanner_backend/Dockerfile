# build stage
FROM python:slim
WORKDIR /app
COPY . .
RUN pip install -r requirements.txt
RUN mkdir log
EXPOSE 3024
CMD ["gunicorn", "-b", ":3024", "server:app"]
