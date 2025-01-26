from django.apps import AppConfig

from core.websocket import WebSocketClient

class CoreConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'core'

    def ready(self):
        print("START SOCKET WS")
        socket = WebSocketClient(uri="ws://engine:9999")

        #socket.send_message("https://vixcloud.co/playlist/194073?token=a745b3577ef3664c5ab38982917f65a1&expires=1743078396&h=1")