console.log("Audio effects content script loaded.");

let video = document.getElementsByTagName("video")[0];
let audioContext = new AudioContext();
let videoSource = audioContext.createMediaElementSource(video);
let compressor = audioContext.createDynamicsCompressor();

chrome.storage.sync.get("compressor-enabled", (data) => {
    const enabled = data["compressor-enabled"];

    if (enabled === undefined || enabled === false) {
        videoSource.connect(audioContext.destination);
    } else if (enabled === true) {
        videoSource.connect(compressor);
        compressor.connect(audioContext.destination);
    }
});

const messageHandler = (msg) => {
    switch (msg.type) {
        case "threshold": {
            compressor.threshold.setValueAtTime(
                msg.value,
                audioContext.currentTime
            );
            break;
        }
        case "knee": {
            compressor.knee.setValueAtTime(msg.value, audioContext.currentTime);
            break;
        }
        case "ratio": {
            compressor.ratio.setValueAtTime(
                msg.value,
                audioContext.currentTime
            );
            break;
        }
        case "attack": {
            compressor.attack.setValueAtTime(
                msg.value,
                audioContext.currentTime
            );
            break;
        }
        case "release": {
            compressor.release.setValueAtTime(
                msg.value,
                audioContext.currentTime
            );
            break;
        }
        default: {
            console.warn("What even is this message?", msg);
        }
    }
};

const initializeStorage = (changes, namespace) => {
    for (let key in changes) {
        let storageChange = changes[key];

        switch (key) {
            case "compressor-enabled": {
                if (storageChange.newValue == true) {
                    videoSource.disconnect(audioContext.destination);
                    videoSource.connect(compressor);
                    compressor.connect(audioContext.destination);
                } else {
                    videoSource.disconnect(compressor);
                    compressor.disconnect(audioContext.destination);
                    videoSource.connect(audioContext.destination);
                }
                break;
            }
            case "threshold":
            case "knee":
            case "ratio":
            case "attack":
            case "release": {
                compressor[key].value = storageChange.newValue;
            }
        }
    }
};

const initializeMessaging = (port) => {
    let connected = true;
    console.info(`Port %c${port.name}`, "color: #3aa757", "connected!");
    port.onDisconnect.addListener(() => {
        console.info(`Port %c${port.name}`, "color: #3aa757", "disconnected.");
        connected = false;
    });

    if (port.name === "compressor") {
        port.onMessage.addListener(messageHandler);

        function sendReduction() {
            if (connected) {
                port.postMessage({
                    type: "reduction",
                    value: compressor.reduction,
                });
                requestAnimationFrame(sendReduction);
            }
        }
        sendReduction();
    }
};

chrome.storage.onChanged.addListener(initializeStorage);
chrome.runtime.onConnect.addListener(initializeMessaging);
//add onDisconnect
