const staticClock = "clock-site-v1";
const assets = [
    "/",
    "/index.html",
    "/index.js",
    "/style.css",
    "/images/january.jpg",
    "/images/february.jpg",
    "/images/march.jpg",
    "/images/april.jpg",
    "/images/may.jpg",
    "/images/june.jpg",
    "/images/july.jpg",
    "/images/august.jpg",
    "/images/september.jpg",
    "/images/october.jpg",
    "/images/november.jpg",
    "/images/december.jpg",
    "/images/sakura.jpg",
];

self.addEventListener("install", (installEvent) => {
    installEvent.waitUntil(
        caches.open(staticClock).then((cache) => cache.addAll(assets))
    );
});

self.addEventListener("fetch", (fetchEvent) => {
    fetchEvent.respondWith(
        caches
            .match(fetchEvent.request)
            .then((res) => res || fetch(fetchEvent.request))
    );
});
