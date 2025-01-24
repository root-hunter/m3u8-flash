const decoder = new TextDecoder("utf-8");
let socket = new WebSocket("ws://127.0.0.1:9999");

const initSocket = () => {
    socket.onopen = () => {
        console.log("Connesso al server WebSocket");
    };
    socket.onmessage = (event) => {
        const data = JSON.parse(event.data);

        chrome.storage.local.set({ [`playlist_${Date.now()}`]: data }, function() {
            console.log('Dati salvati correttamente!');
        });

        console.log(data);
    };
    socket.onclose = () => {
        console.log("Connessione chiusa dal server");
    };
    socket.onerror = (error) => {
        console.error("Errore WebSocket:", error);
    };
}

initSocket();

chrome.webRequest.onCompleted.addListener(
    function (request) {
        if (
            request.statusCode === 200
            && request.method === "GET"
            && request.type === "xmlhttprequest"
            && request.initiator === "https://vixcloud.co"
        ) {
            const url = request.url;
            fetch(url)
                .then(async (response) => {
                    const data = await response.arrayBuffer();
                    const buffer = data.slice(0, 7);
                    const prefix = decoder.decode(buffer);

                    if (prefix === "#EXTM3U") {
                        const file = decoder.decode(data);

                        if (file.includes("#EXT-X-STREAM-INF:")) {
                            console.log(request.url);
                            socket.send(request.url);
                        }
                    }
                }).catch((e) => {
                    console.log(e)
                })
        }
    },
    { urls: ["<all_urls>"], types: ["xmlhttprequest"] },  // Monitora tutte le richieste
)


setInterval(() => {
    if (socket.readyState === WebSocket.CLOSED) {
        try {
            socket = new WebSocket("ws://127.0.0.1:9999");
            if (socket.readyState === WebSocket.CONNECTING || socket.readyState === WebSocket.OPEN) {
                console.log("Connection recreated");
                initSocket();
            }
        } catch (e) {
            //console.log(e);
        }
    }
}, 3000)