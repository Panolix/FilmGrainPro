const { test } = require('node:test');
const assert = require('node:assert/strict');
const vm = require('node:vm');
const fs = require('node:fs');
const path = require('node:path');

function fixture(invoke = async () => {}) {
    let source = fs.readFileSync(path.join(__dirname, '../main.js'), 'utf8').replace(/^import .*;$/gm, '');
    source = source.slice(0, source.indexOf('// Initialize the application')) + '\nglobalThis.Generator = FilmGrainGenerator;';
    const fields = {
        grainSize: { value: '1', type: 'range', min: '0.2', max: '5', step: '0.1' },
        filmStock: { value: 'Stock A', tagName: 'SELECT', options: [{value: 'Stock A'}, {value: 'Stock B'}] },
        movSeconds: { value: '10', type: 'number', min: '1', max: '60', step: '1' },
        ffmpegDirectory: { value: '' }, outputDirectory: { value: '' }
    };
    const context = { document: { getElementById: id => fields[id] }, invoke, clearTimeout };
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
