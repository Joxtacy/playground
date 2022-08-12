function handleContextMenuClick(info, tab) {
    if (info.selectionText && tab.id) {
        const tel = `tel:${info.selectionText}`;
        chrome.tabs.sendMessage(tab.id, tel);
    }
}

chrome.contextMenus.create({
    "id": "click-to-dial",
    "title": "Call",
    "contexts": ["selection"]
});

chrome.contextMenus.onClicked.addListener(
    handleContextMenuClick
);
