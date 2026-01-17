const CACHE_NAME = "qiraa-v1";
const CORE_ASSETS = [
  "/",
  "/index.html",
  "/manifest.json",
  "/qiraa.png",
  "/qiraat.png",
  // add other assets like JS/CSS here if needed
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(CORE_ASSETS))
  );
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then(
      (cached) => cached || fetch(event.request)
    )
  );
});
