const decoder = new TextDecoder("utf-8");
const M3U8_PLAYLISTS_KEY = "items";
let socket = new WebSocket("ws://127.0.0.1:9999");

const initSocket = () => {
    socket.onmessage = (event) => {
        const data = JSON.parse(event.data);
    };
    socket.onclose = () => {
        console.log("Connessione chiusa dal server");
    };
    socket.onerror = (error) => {
        console.error("Errore WebSocket:", error);
    };
}

const getStorage = async (key, defaultValue) => {
    return new Promise((resolve) => {
        chrome.storage.local.get({ [key]: defaultValue }, (data) => {
            resolve(data[key]);
        });
    });
};

const setStorage = async (key, value) => {
    return new Promise((resolve) => {
        chrome.storage.local.set({ [key]: value }, () => {
            resolve();
        });
    });
};

const removeStorageItem = async (key) => {
    return new Promise((resolve) => {
        chrome.storage.local.remove(key, () => resolve());
    });
};

const clearStorage = async () => {
    return new Promise((resolve) => {
        chrome.storage.local.clear(() => resolve());
    });
};

const updateBadge = async () => {
    const items = await getStorage("items", {});
    const tabs = await chrome.tabs.query({ active: true, currentWindow: true });

    if(tabs[0]) {
        console.log(tabs)
        const count = Object.values(items).filter(e => e.tab_id === tabs[0].id).length;
        await chrome.action.setBadgeText({ text: count > 0 ? count.toString() : "" });
    } else {
        await chrome.action.setBadgeText({ text: "" });
    }
}

chrome.webRequest.onCompleted.addListener(
    async (request) => {
        if (
            request.statusCode === 200 &&
            request.method === "GET" &&
            request.type === "xmlhttprequest"
        ) {
            const url = request.url;
            const items = await getStorage(M3U8_PLAYLISTS_KEY, {});
            
            const keys = Object.keys(items);

            if (!keys.some(k => items[k].url === url)) {
                try {
                    const response = await fetch(url);
                    const data = await response.arrayBuffer();
                    const buffer = data.slice(0, 7);
                    const prefix = decoder.decode(buffer);

                    if (prefix === "#EXTM3U") {
                        const file = decoder.decode(data);

                        if (file.includes("#EXT-X-STREAM-INF:")) {
                            chrome.tabs.query({ active: true, currentWindow: true }, async function (tabs) {
                                const uid = crypto.randomUUID();
                                const streamUrl = url.substring(0, url.lastIndexOf("/"));
                                const pageTitle = tabs[0] ? tabs[0].title : 'No title available';
                                const tabId = tabs[0] ? tabs[0].id : null;

                                items[uid] = {
                                    uid: uid,
                                    url: url,
                                    page_url: request.url,
                                    stream_url: streamUrl,
                                    title: pageTitle,
                                    tab_id: tabId
                                };

                                await setStorage(M3U8_PLAYLISTS_KEY, items);
                                
                                console.log(items);
                                console.log("Link saved");
                            });

                        }
                    }
                } catch (e) {
                    console.error(e);
                }
            }
        }
    },
    { urls: ["<all_urls>"], types: ["xmlhttprequest"] }
);

chrome.runtime.onMessage.addListener(async (message, sender, sendResponse) => {
    if (message.type === "parsePlaylist") {
        const items = await getStorage("items", {});
        const item = items[message.uid];

        if(item) {
            socket.send(item.url);
            console.log("Message sended");
            sendResponse({ status: "Badge aggiornato!" });
        }
    }
});

setInterval(async () => {
    if (socket.readyState === WebSocket.CLOSED) {
        try {
            socket = new WebSocket("ws://127.0.0.1:9999");
            socket.onopen = async () => {
                console.log("Connection recreated");
                await clearStorage();
                await removeStorageItem(M3U8_PLAYLISTS_KEY)
                initSocket();
            }
        } catch (e) {
            console.log(e);
        }
    }
}, 3000);

setInterval(async () => {
    await updateBadge();
}, 500);
