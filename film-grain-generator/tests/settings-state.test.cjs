const { test } = require('node:test');
const assert = require('node:assert/strict');
const vm = require('node:vm');
const fs = require('node:fs');
const path = require('node:path');

function fixture(invoke = async () => {}, win = {}) {
    let source = fs.readFileSync(path.join(__dirname, '../main.js'), 'utf8').replace(/^import .*;$/gm, '');
    source = source.slice(0, source.indexOf('// Initialize the application')) + '\nglobalThis.Generator = FilmGrainGenerator;';
    const fields = {
        grainSize: { value: '1', type: 'range', min: '0.2', max: '5', step: '0.1' },
        filmStock: { value: 'Stock A', tagName: 'SELECT', options: [{value: 'Stock A'}, {value: 'Stock B'}] },
        movSeconds: { value: '10', type: 'number', min: '1', max: '60', step: '1' },
        ffmpegDirectory: { value: '' }, outputDirectory: { value: '' }
    };
    const context = { getCurrentWindow: () => win, document: { getElementById: id => fields[id] }, invoke, clearTimeout };
    vm.createContext(context);
    vm.runInContext(source, context);
    const app = Object.create(context.Generator.prototype);
    app.pageDefaults = { 'basic-tab': { grainSize: '1', filmStock: 'Stock A' },
        'output-tab': { movSeconds: '10' }, 'settings-tab': { ffmpegDirectory: '', outputDirectory: '' } };
    app.saveQueue = Promise.resolve();
    app.refreshParameterDisplays = app.updateFilmInfo = app.regenerateGrain = app.scheduleSettingsSave = () => {};
    return { app, fields };
}

test('restores saved values, rejects invalid menus and numeric ranges/steps', () => {
    const { app, fields } = fixture();
    app.applyValues({ grainSize: '0.5', filmStock: 'Stock B', movSeconds: '24' });
    assert.equal(fields.grainSize.value, '0.5');
    assert.equal(fields.filmStock.value, 'Stock B');
    app.applyValues({ grainSize: '0.55', filmStock: 'Unknown', movSeconds: '61', unknown: 'x' });
    assert.equal(fields.grainSize.value, '0.5');
    assert.equal(fields.filmStock.value, 'Stock B');
    assert.equal(fields.movSeconds.value, '24');
    app.applyValues({ grainSize: 'NaN', movSeconds: '1.5' });
    assert.equal(fields.movSeconds.value, '24');
});

test('Restore Defaults affects only its own page', () => {
    const { app, fields } = fixture();
    app.applyValues({ grainSize: '0.5', filmStock: 'Stock B', movSeconds: '30', outputDirectory: 'D:\\Grain' });
    app.restorePageDefaults('basic-tab');
    assert.equal(fields.grainSize.value, '1');
    assert.equal(fields.filmStock.value, 'Stock A');
    assert.equal(fields.movSeconds.value, '30');
    assert.equal(fields.outputDirectory.value, 'D:\\Grain');
    app.restorePageDefaults('settings-tab');
    assert.equal(fields.outputDirectory.value, '');
    assert.equal(fields.movSeconds.value, '30');
});

test('serializes saves and flushes latest snapshot even after an earlier failure', async () => {
    let release, active = 0, peak = 0;
    const writes = [];
    const { app, fields } = fixture(async (_, { settings }) => {
        active++; peak = Math.max(peak, active);
        writes.push(settings);
        try {
            if (writes.length === 1) { await new Promise(resolve => release = resolve); throw Error('Not writable'); }
        } finally { active--; }
    });
    const first = app.persistSettings();
    const failed = assert.rejects(first, /Not writable/);
    await Promise.resolve(); await Promise.resolve();
    fields.grainSize.value = '0.5';
    fields.outputDirectory.value = 'E:\\Grain';
    const final = app.persistSettings();
    release();
    await failed; await final;
    assert.equal(peak, 1);
    assert.equal(writes[0].ui_values.grainSize, '1');
    assert.equal(writes[1].ui_values.grainSize, '0.5');
    assert.equal(writes[1].output_directory, 'E:\\Grain');
});


test('window placement keeps negative monitors and recovers disconnected/smaller displays', () => {
    const { app } = fixture();
    const monitors = [{position: {x: 0, y: 0}, size: {width: 1920, height: 1080}, scaleFactor: 1},
        {position: {x: -2560, y: 0}, size: {width: 2560, height: 1440}, scaleFactor: 1.5}];
    const saved = {x: -2300, y: 100, width: 1400, height: 800, maximized: true};
    const retained = app.fitWindowState(saved, monitors);
    assert.equal(retained.x, -2300);
    assert.equal(retained.maximized, true);
    const recovered = app.fitWindowState(saved, [monitors[0]]);
    assert(recovered.x >= 0 && recovered.y >= 0);
    assert(recovered.x + recovered.width <= 1920);
    assert(recovered.y + recovered.height < 1080);
    const smaller = app.fitWindowState({...saved, x: 0, width: 3000, height: 2000}, [monitors[0]]);
    assert(smaller.width < 1920 && smaller.height < 1080);
    assert.equal(app.fitWindowState({...saved, width: -1}, monitors), null);
});

test('maximized and minimized windows preserve the last normal bounds', async () => {
    let minimized = false, maximized = true;
    const win = {isMinimized: async () => minimized, isMaximized: async () => maximized,
        outerPosition: async () => ({x: -1000, y: 50}), innerSize: async () => ({width: 1800, height: 1200}),
        scaleFactor: async () => 1.5};
    const { app } = fixture(undefined, win);
    app.windowState = {x: 100, y: 100, width: 1400, height: 900};
    await app.captureWindowState();
    assert.equal(app.windowState.width, 1400);
    assert.equal(app.windowState.maximized, true);
    minimized = true; maximized = false;
    await app.captureWindowState();
    assert.equal(app.windowState.maximized, true);
    minimized = false;
    await app.captureWindowState();
    assert.equal(app.windowState.width, 1200);
    assert.equal(app.windowState.height, 800);
    assert.equal(app.windowState.x, -1000);
    assert.equal(app.windowState.maximized, false);
});
