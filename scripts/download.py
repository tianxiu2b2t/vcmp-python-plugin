import requests

URL = "https://v04.adtec.ovh/server/VCMP04_server_v{id}_win64.zip"

def download_server(version_id: int):
    url = URL.format(id=version_id)
    print(f"正在下载 Server v{version_id}...")
    
    try:
        # 使用 stream=True 启用流式下载
        with requests.get(url, stream=True) as r:
            if r.status_code != 200:
                print(f"  Server v{version_id} 下载失败 (HTTP {r.status_code})")
                return
            
            # 保存文件
            filename = f"server_v{version_id}.zip"
            with open(filename, 'wb') as f:
                for chunk in r.iter_content(chunk_size=1024):
                    if chunk:  # 过滤掉空的chunk
                        f.write(chunk)
            
            print(f"  Server v{version_id} 下载完成")
    except Exception as e:
        print(f"  Server v{version_id} 下载出错: {str(e)}")

def main():
    print("开始顺序下载服务器文件...")
    # 顺序下载版本1到39
    for version_id in range(1, 64):
        download_server(version_id)
    
    print("\n所有文件下载完毕！")

if __name__ == "__main__":
    main()