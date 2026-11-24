// Единственный JavaScript на сайте: браузер пока не умеет запускать WebAssembly без загрузчика.
// Всё поведение страницы живёт в Rust (src/app).
import init from './pkg/vasilisa_site.js';

init().catch((error) => console.error('WASM не загрузился, страница работает без интерактива:', error));
