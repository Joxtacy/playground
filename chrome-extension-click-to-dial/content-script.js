chrome.runtime.onMessage.addListener((request, _sender, _sendResponse) => {
    window.open(request, "_top");
});

let inner = document.body.innerHTML;
//let replaced = inner.replaceAll(/(\+?\d{8,15})/g, '<a href="tel:$1">$1</a>');
// Kinda dumb regex. It replaces findings inside of a tags as well, which might not be optimal.
let replaced = inner.replaceAll(/(?<!<\/?[^>]*|&[^;]*)(\+?\d{8,15})/g, '<a href="tel:$1">$1</a>');
document.body.innerHTML = replaced;
