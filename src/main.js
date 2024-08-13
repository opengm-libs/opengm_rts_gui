const { invoke } = window.__TAURI__.tauri;
const { ask, open } = window.__TAURI__.dialog;
const { appDir, homeDir } = window.__TAURI__.path;

let sampleDir;
let data_dir;
let openDirEl;
let testResult;

async function openDir() {
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: await homeDir(),
    });

    if (Array.isArray(selected)) {
        // user selected multiple directories
    } else if (selected === null) {
        // user cancelled the selection
    } else {
        // user selected a single directory
        sampleDir = String(selected);
        data_dir.textContent = sampleDir;
    }
}

async function runTests() {
    testResult.innerHTML = "正在检测...";
    invoke("run_tests", { sampleDir: sampleDir })
        .then(result => {
            testResult.innerHTML = result;
        });
}

let info = "Copyright (c) 2024 The OpenGM Group <opengm@yeah.net>";

window.addEventListener("DOMContentLoaded", () => {
    data_dir = document.querySelector("#data_dir");
    testResult = document.querySelector("#test_result");
    testResult.innerHTML = info;
    document.querySelector("#open_dir").addEventListener("submit", (e) => {
        e.preventDefault();
        openDir();
    });
    document.querySelector("#run_tests").addEventListener("submit", (e) => {
        e.preventDefault();
        runTests();
    });
});
