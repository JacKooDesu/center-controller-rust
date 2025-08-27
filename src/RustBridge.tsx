import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

let listening: Map<string, UnlistenFn> = new Map();

export interface JPEGData {
    payload: {
        addr: string;
        data: [];
    };
}

export interface ClientChangedData {
    payload: {
        add: string | null;
        remove: string | null;
    };
}

export async function startUdp() {
    await invoke("start_udp");
}

export async function stopUdp() {
    await invoke("stop_udp");

    listening.forEach((unlisten) => unlisten());
    listening.clear();
}

export async function addClientChangeListener(id: string, cb: (data: ClientChangedData) => void) {
    await addListener<ClientChangedData>(
        id + "_clientListener",
        "fm://client_changed",
        cb);
}

export async function addJpgDecodedListener(id: string, cb: (bytes: []) => void) {
    await addListener<JPEGData>(
        id + "_jpegListener",
        "fm://jpeg_decoded",
        data => {
            if (data.payload.addr == id)
                cb(data.payload.data);
        });
}

async function addListener<TData>(id: string, endpoint: string, cb: (data: TData) => void): Promise<Boolean> {
    if (listening.has(id)) return false;
    listening.set(id, await listen(endpoint, (data) => {
        console.log("Received " + id + " packet:", data);
        cb(data as TData);
    }));

    return true;
}