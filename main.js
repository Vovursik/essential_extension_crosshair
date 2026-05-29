import App from './src/app.js';

const app = new App(40, './src/public/crosshair_page.html');
app.init().catch(console.error);