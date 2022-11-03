const invoke = window.__TAURI__.invoke

export async function invokHello(name) {
    return await invoke("hello", {name:name});
}