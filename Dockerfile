FROM python:3.13-slim

WORKDIR /app
COPY pyproject.toml pdm.lock /app/
COPY libraries /app/plugins

RUN pip install pdm --no-cache-dir && \
    pdm install && \
    pdm add vcmp-python-plugin

