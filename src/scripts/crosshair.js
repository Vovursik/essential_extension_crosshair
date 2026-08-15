export default class Crosshair {
    constructor() {
        this.root = document.getElementById("crosshair-root");
        this.crosshair = null;
        this.style = null;
    }

    create(html, css) {
        if (!this.root) return;

        if (css) {
            if (!this.style) {
                this.style = document.createElement("style");
                document.head.appendChild(this.style);
            }

            this.style.textContent = css;
        }
        this.root.innerHTML = html;

        this.crosshair = document.getElementById('crosshair');
    }

    setMouseState(pressed) {
        if (this.crosshair)
            this.crosshair.classList.toggle('hidden', pressed);
    }
}