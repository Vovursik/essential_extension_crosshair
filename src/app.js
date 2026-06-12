const { listen } = window.__TAURI__.event;
import WindowCrosshair from "./window_crosshair.js";

export default class App {
    constructor() {
        this.crosshairWindow = null;
        this.unlistenMouse = null;
        this.unlistenCrosshair = null;
    }

    async init() {
        this.crosshairWindow = new WindowCrosshair();

        await this.setupMouseListener();
        await this.setupCrosshairListener();
    }

    async setupMouseListener() {
        console.log("registering mouse listener");

        this.unlistenMouse = await listen("mouse-state", (event) => {
            this.crosshairWindow.setMouseState(Boolean(event.payload));
        });

        console.log("mouse listener registered");
    }

    async setupCrosshairListener() {
        console.log("registering crosshair listener");

        this.unlistenCrosshair = await listen("set-crosshair", (event) => {
            console.log("set-crosshair:", event.payload);

            const { html, css } = event.payload;

            this.crosshairWindow.setCrosshair(html, css);
        });

        console.log("crosshair listener registered");
    }
}