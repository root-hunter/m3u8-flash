from django.apps import AppConfig

from core.websocket import WebSocketClient

class CoreConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'core'

    def ready(self):
        socket = WebSocketClient(uri="ws://localhost:9999")

        socket.send_message("https://vixcloud.co/playlist/279808?token=79d9ca5042f3905e61b5a46b6656651f&expires=1742863198&h=1")