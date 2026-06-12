export default class WindowCrosshair {
    constructor() {
        this.root = document.getElementById("crosshair-root");
        this.crosshair = null;
        this.styleTag = null;
    }

    setMouseState(pressed) {
        if (this.crosshair)
            this.crosshair.classList.toggle('hidden', pressed);
    }

    setCrosshair(html, css) {
        if (!this.root) return;

        if (css) {
            if (!this.styleTag) {
                this.styleTag = document.createElement("style");
                document.head.appendChild(this.styleTag);
            }

            this.styleTag.textContent = css;
        }

        this.root.innerHTML = html;

        this.crosshair = document.getElementById('crosshair');
    }
}