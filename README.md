# Visual Crosshair V

![Version](https://img.shields.io/badge/version-0.2.0-blue?style=flat-square)
![Platform](https://img.shields.io/badge/platform-Windows-0078d4?style=flat-square&logo=windows)

<table>
  <tr>
    <td width="150" align="center">
      <img src="./icon.png" alt="Logo" width="100">
    </td>
    <td>
      <strong>Visual Crosshair V</strong> is a tiny desktop app that draws a custom crosshair in the center of your screen — always on top, always transparent, never blocking your clicks.<br>
      Useful for games that don't have a built-in crosshair, or any situation where you need a fixed center point on screen.
    </td>
  </tr>
</table>

## 🦾 Features

- 🖥️ **Transparent overlay crosshair on top of all windows**
- 🖱️ **Hides on Right Click — smooth fade out, instant reappear**
- ⚙️ **System tray simple options**

## 🛠️ Creating your own crosshair

You can load a completely custom crosshair by `.zip` file .

### 📦 ZIP structure

```
my_crosshair.zip
├── index.html   <- required, must have <div id="crosshair">
└── style.css    <- optional, linked from index.html
```

### 📄❗ Requirements for your HTML

---

<table>
  <tr>
    <td width="50%" valign="top">
      Your <code>index.html</code> must contain an element with <code>id="crosshair"</code> — the app uses it to apply the hide/show animation on right click:
      <pre><code>&lt;div id="crosshair"&gt;
    &lt;!-- anything you want --&gt;
&lt;/div&gt;</code></pre>
    </td>
    <td width="50%" valign="top">
      The hide animation is handled automatically via CSS class <code>hidden</code>:
      <pre><code>#crosshair.hidden {
    opacity: 0;
    transition: opacity 1.4s ease;
}</code></pre>
     </td>
   </tr>
</table>

You can override the transition speed in your own `style.css`.

### 💡 How to load

---

1. Pack your files into a `.zip`
2. Open the app tray icon → **Load crosshair...**
3. Select your `.zip`
4. The crosshair updates and save instantly — no restart needed

## Requirements

| Tool | Version |
|---|---|
| [Node.js](https://nodejs.org) | 18+ |
| [Rust](https://rustup.rs) | stable |

### Install & run
---

```bash
git clone https://github.com/Vovursik/visual-crosshair-v
cd visual-crosshair-v
npm install
npm run tauri dev
```

### Build exe
---

```bash
npm run tauri build
```

Output: `src-tauri/target/release/visual-crosshair-v.exe`