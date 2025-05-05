FROM python:3.13-slim

WORKDIR /app
# mkdir
RUN mkdir -p /app/plugin
# RUN cp /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so
ADD requirements.txt /app/requirements.txt
ADD /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so

# update 
RUN apt update && apt install -y \
    gcc-12 g++-12 libstdc++6 \
    gcc-13 g++-13

RUN pip install -r requirements.txt

