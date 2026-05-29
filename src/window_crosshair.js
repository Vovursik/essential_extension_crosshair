import { BrowserWindow, screen } from 'electron';

class WindowCrosshair {
    constructor(height, width, page, x_offset = 0, y_offset = 0) {
        this.width = width;
        this.height = height;
        this.x_offset = x_offset;
        this.y_offset = y_offset;
        this.page = page;

        this.window = new BrowserWindow({
            width: this.width,
            height: this.height,

            transparent: true,
            frame: false,

            alwaysOnTop: true,
            skipTaskbar: false,

            resizable: false,
            focusable: false,
            hasShadow: false,

            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false
            }
        });

        this.init();
    }

    init = () => {
        this.window.setIgnoreMouseEvents(true, {forward: true});
        this.window.loadFile(this.page);

        const { width, height } = screen.getPrimaryDisplay().bounds;
        this.window.setPosition(
            Math.floor((width - 40) / 2 + this.x_offset),
            Math.floor((height - 40) / 2 + this.y_offset)
        );
    }

    window = () => this.window;
}

export default WindowCrosshair;