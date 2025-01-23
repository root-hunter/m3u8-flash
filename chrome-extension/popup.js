// Quando il popup si apre, carica gli URL M3U8 memorizzati
chrome.storage.local.get("m3u8Urls", function (data) {
    const m3u8Urls = data.m3u8Urls || []; // Se non ci sono URL, usa un array vuoto
    const m3u8List = document.getElementById("m3u8-list");
  
    // Pulisci la lista precedente
    m3u8List.innerHTML = "";
  
    // Se ci sono URL, aggiungili alla lista
    if (m3u8Urls.length > 0) {
      m3u8Urls.forEach(url => {
        const listItem = document.createElement("li");
        listItem.textContent = url;
        m3u8List.appendChild(listItem);
      });
    } else {
      // Se non ci sono URL M3U8, mostra un messaggio
      const noUrlsMessage = document.createElement("li");
      noUrlsMessage.textContent = "Nessun file M3U8 trovato.";
      m3u8List.appendChild(noUrlsMessage);
    }
  });
  
  // Copia l'URL M3U8 selezionato negli appunti (selezione singola)
  document.getElementById("m3u8-list").addEventListener("click", function (event) {
    if (event.target.tagName === "LI") {
      const selectedUrl = event.target.textContent;
      navigator.clipboard.writeText(selectedUrl).then(function () {
        alert("URL M3U8 copiato negli appunti!");
      });
    }
  });
  