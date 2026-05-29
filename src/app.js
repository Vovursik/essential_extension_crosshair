import { app, BrowserWindow } from 'electron';
import { GlobalKeyboardListener } from 'keyspy';

import WindowCrosshair from './window_crosshair.js';

class App {
    constructor(crosshairSize, crosshairPage) {
        this.crosshairSize = crosshairSize;
        this.crosshairPage = crosshairPage;
        this.crosshairWindow = null;
        this.listener = null;
    }

    async init() {
        await app.whenReady();
        
        this.createWindow();
        this.setupEventListeners();
        this.setupKeyspy();
    }

    createWindow = () => this.crosshairWindow = new WindowCrosshair(this.crosshairSize, this.crosshairSize, this.crosshairPage);

    setupEventListeners() {
        app.on('window-all-closed', () => {
            this.cleanup();
            if (process.platform !== 'darwin') app.quit();
        });
    }

    setupKeyspy() {
        console.log('Keyspy started');

        this.listener = new GlobalKeyboardListener();

        this.listener.addListener((event, input) => {
            if (event.name === 'MOUSE RIGHT') {
                this.sendToPage('mouse-state', input['MOUSE RIGHT']);
            }
        });
    }

    sendToPage(channel, data) {
        if (this.crosshairWindow.window) {
            this.crosshairWindow.window.webContents.send(channel, data);
        }
    }

    cleanup() {
        if (this.listener) {
            this.listener = null;
        }
    }
}

export default App;