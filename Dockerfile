FROM python:3.13-slim

WORKDIR /app
# mkdir
RUN mkdir -p /app/plugins
# RUN cp /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so
ADD requirements.txt /app/requirements.txt
ADD /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugins/vcmp-python-plugin-cpy313-rel64.so

RUN apt-get update && apt-get install -y libc6
# verify
RUN strings /usr/lib/x86_64-linux-gnu/libstdc++.so.6 | grep GLIBCXX
RUN strings /usr/lib/x86_64-linux-gnu/libgcc_s.so.1 | grep GLIBC

# clean
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

RUN pip install -r requirements.txt

