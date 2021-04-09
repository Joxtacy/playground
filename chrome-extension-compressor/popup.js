// Initialize button with user's preferred color
// let changeColor = document.getElementById("changeColor");

let threshold = document.getElementById("threshold");
let knee = document.getElementById("knee");
let ratio = document.getElementById("ratio");
let attack = document.getElementById("attack");
let release = document.getElementById("release");
let reduction = document.getElementById("reduction");
let enableCompressor = document.getElementById("enable-compressor");

chrome.storage.sync.get("compressor-enabled", (data) => {
    const enabled = data["compressor-enabled"];

    if (enabled === undefined || enabled === false) {
        enableCompressor.setAttribute("data-active", "false");
        enableCompressor.textContent = "Enable";
    } else if (enabled === true) {
        enableCompressor.setAttribute("data-active", "true");
        enableCompressor.textContent = "Disable";
    }
});

enableCompressor.addEventListener("click", (event) => {
    const active = enableCompressor.getAttribute("data-active");

    if (active == "false") {
        chrome.storage.sync.set({ "compressor-enabled": true });
        enableCompressor.setAttribute("data-active", "true");
        enableCompressor.textContent = "Disable";
    } else if (active == "true") {
        chrome.storage.sync.set({ "compressor-enabled": false });
        enableCompressor.setAttribute("data-active", "false");
        enableCompressor.textContent = "Enable";
    }
});

(async function () {
    let [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    let port = chrome.tabs.connect(tab.id, { name: "compressor" });
    console.log("set up connection", port);
    port.onMessage.addListener((msg) => {
        console.log("popup onMessage", msg);
        if (msg.type === "reduction") {
            reduction.value = msg.value;
        }
    });

    threshold.addEventListener("change", (event) => {
        port.postMessage({
            type: "threshold",
            value: event.target.value,
        });
    });
    knee.addEventListener("change", (event) => {
        port.postMessage({
            type: "knee",
            value: event.target.value,
        });
    });
    ratio.addEventListener("change", (event) => {
        port.postMessage({
            type: "ratio",
            value: event.target.value,
        });
    });
    attack.addEventListener("change", (event) => {
        port.postMessage({
            type: "attack",
            value: event.target.value,
        });
    });
    release.addEventListener("change", (event) => {
        port.postMessage({
            type: "release",
            value: event.target.value,
        });
    });
})();

// Maybe use the storage API for communication?
/*
threshold.addEventListener("change", (event) => {
    console.log("threshold change", event.target.value);
    chrome.storage.sync.set({ threshold: Number(event.target.value) });
});
*/

/*
knee.addEventListener("change", (event) => {
    console.log("knee change", event.target.value);
    chrome.storage.sync.set({ knee: Number(event.target.value) });
});
*/
