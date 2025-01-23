(function() {
    // Intercetta tutte le richieste di rete con fetch()
    const originalFetch = window.fetch;
    window.fetch = async function(...args) {
        const response = await originalFetch(...args);
        const clonedResponse = response.clone();
        
        clonedResponse.text().then(body => {
            console.log("Risposta intercettata via fetch:", body);
            chrome.runtime.sendMessage({ url: args[0], response: body });
        });

        return response;
    };

    // Intercetta XMLHttpRequest
    const originalXHROpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function(method, url) {
        this.addEventListener('load', function() {
            console.log("Risposta intercettata via XHR:", this.responseText);
            chrome.runtime.sendMessage({ url: url, response: this.responseText });
        });
        originalXHROpen.apply(this, arguments);
    };
})();
