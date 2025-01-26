FROM python:3

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    libpq-dev gcc python3-dev musl-dev iputils-ping \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY ./m3u8_flash .
CMD ["python", "manage.py", "runserver", "0.0.0.0:8000"]