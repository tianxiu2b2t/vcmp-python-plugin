FROM python:3.13-slim

WORKDIR /app
# mkdir
RUN mkdir -p /app/plugins
# RUN cp /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugin/vcmp-python-plugin-cpy313-rel64.so
ADD requirements.txt /app/requirements.txt
ADD /libraries/vcmp-python-plugin-cpy313-rel64.so /app/plugins/vcmp-python-plugin-cpy313-rel64.so

# update 
RUN apt-get update && apt-get install -y \
    gcc-12 g++-12 libstdc++6

RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-12 100 \
    && update-alternatives --install /usr/bin/g++ g++ /usr/bin/g++-12 100
    
# verify
RUN strings /usr/lib/x86_64-linux-gnu/libstdc++.so.6 | grep GLIBCXX

# clean
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

RUN pip install -r requirements.txt

