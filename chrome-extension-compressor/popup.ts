let threshold = document.getElementById("threshold");
let knee = document.getElementById("knee");
let ratio = document.getElementById("ratio");
let attack = document.getElementById("attack");
let release = document.getElementById("release");
let reduction = document.getElementById("reduction");
let enableCompressor = document.getElementById("enable-compressor");
let saveButton = document.getElementById("save-settings");

const settingsSliders = [threshold, knee, ratio, attack, release];

const communicate = (obj: object, fn: string) => (
    type: string,
    value: string | number
): void => {
    if (fn === "postMessage") {
        obj[fn]({ type, value });
    } else if (fn === "set") {
        obj[fn]({ [type]: Number(value) });
    }
};

const getStoredData = async (key) =>
    new Promise((resolve, reject) => {
        chrome.storage.sync.get(key, (data) => {
            const value = data[key];
            resolve(value);
        });
    });

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

saveButton.addEventListener("click", (event) => {
    settingsSliders.forEach((slider) => {
        chrome.storage.sync.set({ [slider.id]: Number(slider.value) });
    });
});

(async function () {
    let [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    let port = chrome.tabs.connect(tab.id, { name: "compressor" });
    console.info(`Open port %c${port.name}`, "color: #3aa757");

    window.addEventListener("unload", () => {
        console.info(`Closing port %c${port.name}`, "color: #3aa757");
        port.disconnect();
    });

    port.onMessage.addListener((msg) => {
        if (msg.type === "reduction") {
            reduction.value = msg.value;
        }
    });

    const promises = settingsSliders.map((slider) => getStoredData(slider.id));
    const values = await Promise.all(promises);
    for (let i = 0; i < values.length; i++) {
        settingsSliders[i].value = values[i];
    }

    const communicator = communicate(port, "postMessage");
    // const communicator = communicate(chrome.storage.sync, "set");

    threshold.addEventListener("change", (event) => {
        communicator("threshold", event.target.value);
    });
    knee.addEventListener("change", (event) => {
        communicator("knee", event.target.value);
    });
    ratio.addEventListener("change", (event) => {
        communicator("ratio", event.target.value);
    });
    attack.addEventListener("change", (event) => {
        communicator("attack", event.target.value);
    });
    release.addEventListener("change", (event) => {
        communicator("release", event.target.value);
    });
})();
