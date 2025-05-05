FROM python:3.13-silm

WORKDIR /app
# mkdir
RUN mkdir -p /app/plugin
ADD /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so

RUN pip install -r requirements.txt

