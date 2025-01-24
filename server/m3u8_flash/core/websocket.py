import asyncio
import websockets
import threading

class WebSocketClient:
    _instance = None  # Singleton instance

    def __new__(cls, *args, **kwargs):
        if not cls._instance:
            cls._instance = super(WebSocketClient, cls).__new__(cls)
            cls._instance.uri = kwargs.get("uri", "ws://localhost:9999")
            cls._instance.loop = asyncio.new_event_loop()
            threading.Thread(target=cls._instance._start_event_loop, daemon=True).start()
        return cls._instance

    def _start_event_loop(self):
        """Avvia l'event loop in un thread separato"""
        asyncio.set_event_loop(self.loop)
        self.loop.run_forever()

    async def _connect(self):
        """Crea la connessione WebSocket e avvia la ricezione dei messaggi"""
        self.websocket = await websockets.connect(self.uri)
        print(f"Connesso al WebSocket: {self.uri}")

        # Avvia il listener per ricevere messaggi
        asyncio.create_task(self._receive_messages())

    async def _receive_messages(self):
        """Riceve messaggi dal WebSocket in modo continuo"""
        try:
            async for message in self.websocket:
                print(f"Messaggio ricevuto: {message}")
                self.handle_message(message)
        except websockets.ConnectionClosed:
            print("Connessione chiusa, riconnessione in corso...")
            await self._connect()

    def handle_message(self, message):
        """Gestisce i messaggi ricevuti (puoi personalizzarlo come necessario)"""
        print(f"Elaborazione messaggio: {message}")

    def send_message(self, message):
        """Invia un messaggio al server WebSocket"""
        asyncio.run_coroutine_threadsafe(self._send_message(message), self.loop)

    async def _send_message(self, message):
        """Invia il messaggio in modo asincrono"""
        if not hasattr(self, 'websocket') or self.websocket.closed:
            await self._connect()
        await self.websocket.send(message)
        print(f"Messaggio inviato: {message}")

    def close(self):
        """Chiude la connessione WebSocket"""
        asyncio.run_coroutine_threadsafe(self._close_connection(), self.loop)

    async def _close_connection(self):
        if hasattr(self, 'websocket'):
            await self.websocket.close()
            print("Connessione WebSocket chiusa")
