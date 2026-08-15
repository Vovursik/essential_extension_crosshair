const { listen } = window.__TAURI__.event;
import Crosshair from "./crosshair.js";

export default class App {
    constructor() {
        this.window = null;

        this.mouseEvent = null;
        this.updateEvent = null;
    }

    async init() {
        this.crosshair = new Crosshair();

        await this.setupMouseListener();
        await this.setupCrosshairListener();
    }

    async setupMouseListener() {
        this.unlistenMouse = await listen("mouse-state", (event) => {
            this.crosshair.setMouseState(Boolean(event.payload));
        });
    }

    async setupCrosshairListener() {
        this.unlistenCrosshair = await listen("set-crosshair", (event) => {
            console.log("set-crosshair:", event.payload);

            const { html, css } = event.payload;

            this.crosshair.create(html, css);
        });
    }
}