document.addEventListener("DOMContentLoaded", async function () {
  const urlList = document.getElementById("urlList");
  const clearBtn = document.getElementById("clearStorage");

  // Recupera gli URL dallo storage
  const getItems = async () => {
    return new Promise((resolve) => {
      chrome.storage.local.get({ items: {} }, (data) => {
        const items = Object.keys(data.items).map(k => data.items[k]);
        resolve(items);
      });
    });
  };

  const getItemsObject = async () => {
    return new Promise((resolve) => {
      chrome.storage.local.get({ items: {} }, (data) => {
        resolve(data.items);
      });
    });
  };

  // Rimuove un URL dalla lista
  const removeUrl = async (uid) => {
    let items = await getItemsObject();
    items[uid] = undefined;

    chrome.storage.local.set({ items: items }, async () => await loadUrls());
  };

  // Mostra gli URL nella UI
  const loadUrls = async () => {
    const urls = await getItems();
    const urlList = document.getElementById("urlList");
    urlList.innerHTML = "";

    urls.forEach(item => {
        // Creazione dell'elemento della lista con Bootstrap
        const li = document.createElement("li");
        li.className = "row card";

        const liHeader = document.createElement("li");
        liHeader.className = "card-header list-group-item d-flex justify-content-between align-items-center mb-1";

        // Testo dell'URL
        const urlText = document.createElement("span");
        urlText.innerHTML = `<b>${item.title}</b>`;
        urlText.className = "text-truncate"; // Per evitare overflow su URL lunghi

        // Pulsante di azione (esempio: Avvia richiesta)
        const actionBtn = document.createElement("button");
        actionBtn.className = "btn btn-success btn-sm me-2"; // Piccolo pulsante con margine
        actionBtn.innerHTML = '<i class="bi bi-cloud-download-fill"></i>';
        actionBtn.onclick = () => startRequest(item.url);

        // Creazione del contenitore per i dettagli
        const detailsContainer = document.createElement("div");
        detailsContainer.className = "card-body mt-2";
        detailsContainer.style.display = "none"; // Dettagli nascosti di default

        const streamUrlText = document.createElement("div");
        streamUrlText.className = "row"
        streamUrlText.innerHTML = `
          <strong class="col">Stream URL:</strong><em class="col">${item.stream_url}</em>
        `;

        const fullUrlText = document.createElement("div");
        fullUrlText.className = "row"
        fullUrlText.innerHTML = `
          <strong class="col">Playlist URL:</strong><em class="col">${item.url}</em>
        `;
        
        detailsContainer.appendChild(streamUrlText);
        detailsContainer.appendChild(fullUrlText);

        // Pulsante per mostrare/nascondere i dettagli
        const detailBtn = document.createElement("button");
        detailBtn.className = "btn btn-light btn-sm me-2";
        detailBtn.innerHTML = `<i class="bi bi-info-circle"></i>`;
        detailBtn.onclick = () => {
            // Toggle visibility dei dettagli
            const isVisible = detailsContainer.style.display === "block";
            detailsContainer.style.display = isVisible ? "none" : "block";
        };

        // Pulsante di eliminazione
        const deleteBtn = document.createElement("button");
        deleteBtn.className = "btn btn-danger btn-sm me-2";
        deleteBtn.innerHTML = `<i class="bi bi-trash3-fill"></i>`; // Icona Bootstrap
        deleteBtn.onclick = () => removeUrl(item.uid);

        const showBtn = document.createElement("a");
        showBtn.className = "btn btn-secondary btn-sm me-2";
        showBtn.innerHTML = `<i class="bi bi-code-slash"></i>`; // Icona Bootstrap
        showBtn.href = item.url;

        // Gruppo di pulsanti
        const btnGroup = document.createElement("div");
        btnGroup.className = "d-flex";
        btnGroup.appendChild(actionBtn);
        btnGroup.appendChild(showBtn);
        btnGroup.appendChild(deleteBtn);

        // Aggiungi tutto
        liHeader.appendChild(detailBtn); 
        liHeader.appendChild(urlText);   
        liHeader.appendChild(btnGroup);  

        li.appendChild(liHeader); // Aggiungi i dettagli sotto
        li.appendChild(detailsContainer); // Aggiungi i dettagli sotto

        // Aggiungi l'elemento alla lista
        urlList.appendChild(li);
    });
};

  // Esempio di funzione per eseguire una richiesta
  const startRequest = (url) => {
    console.log("Avvio richiesta per:", url);
    fetch(url).then(response => console.log("Risultato:", response));
  };

  // Svuota tutta la lista
  clearBtn.addEventListener("click", () => {
    chrome.storage.local.set({ items: {} }, async () => await loadUrls());
  });

  // Carica gli URL all'avvio
  loadUrls();
});
