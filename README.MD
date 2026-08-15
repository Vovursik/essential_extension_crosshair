# Essential Extension Crosshair

![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)
![Platform](https://img.shields.io/badge/platform-Windows-0078d4?style=flat-square&logo=windows)

<img src="./src-tauri/icons//icon.png" alt="Logo" width="100">

A tiny desktop app that draws a custom crosshair in the center of your screen.

## Create your own crosshair

You can load a completely custom crosshair by `.zip` file .

### ZIP structure

```
untitled.zip
├── index.html
└── style.css
```
### Requirements for your HTML

<table>
  <tr>
    <td width="50%" valign="top">
      Your <code>index.html</code> must contain an element with <code>id="crosshair"</code> — the app uses it to apply the hide/show animation:
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

## Requirements

| Tool | Version |
|---|---|
| [Node.js](https://nodejs.org) | 18+ |
| [Rust](https://rustup.rs) | stable |

## Getting Started

1. Clone the repository
1. Оpen a terminal, navigate to the project directory where <code>package.json</code> is located, and run commands below:
1. `npm install`
1. `npm run tauri dev`

### Build exe

``
npm run tauri build
``

Output: `src-tauri/target/release/essential_extension_crosshair.exe`