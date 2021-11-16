FROM python:3

ENV FLASK_APP=/sec/secondary.py
ENV FLASK_RUN_HOST=0.0.0.0
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
EXPOSE 5000
# COPY . .
CMD ["flask", "run"]
