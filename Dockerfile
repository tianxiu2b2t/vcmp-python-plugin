FROM python:3.13-slim

WORKDIR /app
# mkdir
RUN mkdir -p /app/plugins
# RUN cp /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so
ADD requirements.txt /app/requirements.txt
ADD /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugins/vcmp-python-plugin-cpy313-rel64.so
    
# verify
RUN strings /usr/lib/x86_64-linux-gnu/libstdc++.so.6 | grep GLIBCXX

# clean
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

RUN pip install -r requirements.txt

