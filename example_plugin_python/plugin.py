# 本模板由天机Ceyase编写，基于MIT许可证开源
import hiperlink_pb2
import hiperlink_pb2_grpc
import health_pb2_grpc
import grpc
import concurrent.futures as futures
import sys
import time
import random
import socket

class HiperLinkServicer(hiperlink_pb2_grpc.PluginServicer):

    # 加载插件
    def OnLoad(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # 停用插件
    def OnUnload(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # 安装插件
    def OnInstall(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # 卸载插件
    def OnUninstall(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # HL 启动
    def OnStart(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # HL 停止
    def OnStop(self, request, context):
        return hiperlink_pb2.EventsResponse()

    # 前后端交互
    def Interaction(self, request, context):
        return hiperlink_pb2.InteractionResponse()

def get_free_port():
    while True:
        port = random.randint(32768, 61000)
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        if not (sock.connect_ex(('127.0.0.1', port)) == 0):
            return port 

if __name__ == '__main__':
    health = health_pb2_grpc.HealthServicer()
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    hiperlink_pb2_grpc.add_PluginServicer_to_server(HiperLinkServicer(), server)
    health_pb2_grpc.add_HealthServicer_to_server(health, server)
    port = get_free_port()
    server.add_insecure_port(f'127.0.0.1:{port}')
    server.start()
    print(f"1|1|tcp|127.0.0.1:{port}|grpc")
    sys.stdout.flush()
    try:
        while True:
            time.sleep(60 * 60 * 24)
    except KeyboardInterrupt:
        server.stop(0)
