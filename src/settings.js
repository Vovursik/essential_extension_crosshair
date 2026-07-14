const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const listEl = document.getElementById('crosshair-list');
const loadBtn = document.getElementById('load-btn');
const pressRow = document.getElementById('mode-press');
const holdRow = document.getElementById('mode-hold');
const pressCountShowSub = document.getElementById('press-count-show-sub');
const pressCountHideSub = document.getElementById('press-count-hide-sub');

async function renderList() {
  const [crosshairs, activeId] = await Promise.all([
    invoke('list_crosshairs'),
    invoke('get_active_crosshair'),
  ]);

  listEl.innerHTML = '';

  if (crosshairs.length === 0) {
    const empty = document.createElement('div');
    empty.className = 'crosshair-empty';
    empty.textContent = 'EMPTY CROSSHAIR';
    listEl.appendChild(empty);
    return;
  }

  for (const c of crosshairs) {
    const item = document.createElement('div');
    item.className = 'crosshair-item' + (c.id === activeId ? ' active' : '');

    item.innerHTML = `
      <div class="crosshair-preview"><iframe class="preview-frame" scrolling="no" sandbox="allow-same-origin"></iframe></div>
      <div class="crosshair-name">${c.name}</div>
      <button class="crosshair-delete">✕</button>
    `;

    item.addEventListener('click', () => applyCrosshair(c.id));

    item.querySelector('.crosshair-delete').addEventListener('click', (e) => {
      e.stopPropagation();
      deleteCrosshair(c.id);
    });

    listEl.appendChild(item);

    invoke('get_crosshair_preview', { id: c.id }).then((preview) => {
      if (!preview) return;
      const frame = item.querySelector('.preview-frame');
      frame.srcdoc = `<style>html,body{margin:0;background:transparent;}${preview.css}</style>${preview.html}`;
    });
  }
}

function applyCrosshair(id) {
  invoke('apply_crosshair', { id });
}

function deleteCrosshair(id) {
  invoke('delete_crosshair', { id });
}

loadBtn.addEventListener('click', () => {
  invoke('load_crosshair');
});

listen('crosshairs-changed', () => {
  renderList();
}).catch((err) => {
  console.error('Не удалось подписаться на crosshairs-changed:', err);
});

function setModeUI(isPress) {
  pressRow.classList.toggle('selected', isPress);
  holdRow.classList.toggle('selected', !isPress);
  pressCountShowSub.classList.toggle('hidden', !isPress);
  pressCountHideSub.classList.toggle('hidden', !isPress);
}

pressRow.addEventListener('click', async () => {
  const current = await invoke('get_mode');
  if (!current) await invoke('toggle_mode');
  setModeUI(true);
});

holdRow.addEventListener('click', async () => {
  const current = await invoke('get_mode');
  if (current) await invoke('toggle_mode');
  setModeUI(false);
});

function setupStepper({ inputId, minusId, plusId, getCmd, setCmd }) {
  const input = document.getElementById(inputId);
  const minusBtn = document.getElementById(minusId);
  const plusBtn = document.getElementById(plusId);

  function commit(value) {
    const clamped = Math.max(1, parseInt(value, 10) || 1);
    input.value = clamped;
    invoke(setCmd, { count: clamped });
    return clamped;
  }

  input.addEventListener('change', () => commit(input.value));

  input.addEventListener('input', () => {
    input.value = input.value.replace(/[^0-9]/g, '');
  });

  minusBtn.addEventListener('click', () => {
    commit((parseInt(input.value, 10) || 1) - 1);
  });

  plusBtn.addEventListener('click', () => {
    commit((parseInt(input.value, 10) || 1) + 1);
  });

  return {
    async init() {
      const count = await invoke(getCmd);
      input.value = count;
    },
  };
}

const showStepper = setupStepper({
  inputId: 'press-count-show',
  minusId: 'count-show-minus',
  plusId: 'count-show-plus',
  getCmd: 'get_press_count_show',
  setCmd: 'set_press_count_show',
});

const hideStepper = setupStepper({
  inputId: 'press-count-hide',
  minusId: 'count-hide-minus',
  plusId: 'count-hide-plus',
  getCmd: 'get_press_count_hide',
  setCmd: 'set_press_count_hide',
});

async function init() {
  const isPress = await invoke('get_mode');
  setModeUI(isPress);

  await Promise.all([showStepper.init(), hideStepper.init()]);

  renderList();
}

init();