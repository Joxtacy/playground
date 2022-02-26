console.log("this is child");

const notifyMe = async () => {
    // Let's check if the browser supports notifications
    if (!("Notification" in window)) {
        alert("This browser does not support desktop notification");
        throw Error("Nope. No notifications.");
    }

    // Let's check whether notification permissions have already been granted
    else if (Notification.permission === "granted") {
        // If it's okay let's create a notification
        console.log("");
        return new Notification("Hi there!");
    }

    // Otherwise, we need to ask the user for permission
    else if (Notification.permission !== "denied") {
        Notification.requestPermission().then(function (permission) {
            // If the user accepts, let's create a notification
            if (permission === "granted") {
                return new Notification("Hi there!");
            }
        });
    }

    // At last, if the user has denied notifications, and you
    // want to be respectful there is no need to bother them any more.
};

setTimeout(() => {
    notifyMe().then((notification) => {
        notification.onclick = () => {
            console.log("clicked the notidfication derp", {
                window,
                parent: window.parent,
                top: window.top,
            });

            // window.parent && window.parent.focus();
            // window.top && window.top.focus && window.top.focus();
            console.log(
                "chrome.runtime.MessageSender",
                chrome.runtime.MessageSender
            );
            chrome.runtime.sendMessage(
                "bjjekalmelaaafdpinmbgenobhacpdjn",
                {
                    derp: "herp",
                },
                (response) =>
                    console.log("this is the message response derp", response)
            );
        };
    });
}, 5000);
