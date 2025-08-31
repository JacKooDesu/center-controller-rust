import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

let listening: Map<string, UnlistenFn> = new Map();

export interface MessageData<T> {
    payload: T
};

export interface JPEGData {
    addr: string;
    data: [];
}

export interface ClientChangedData {
    add: string | null;
    remove: string | null;
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
            if (data.addr == id)
                cb(data.data);
        });
}

async function addListener<TData>(id: string, endpoint: string, cb: (data: TData) => void) {
    listening.set(id, await listen(endpoint, (data) => {
        cb(data.payload as TData);
    }));
}

export async function send(addr: string, msg: string) {
    console.log(addr);
    let arr = addr.split(':');
    console.log("Send", msg, "to", arr[0]);

    await invoke("send_msg", { addr: arr[0], msg: msg });
}

export async function query_play_history(): Promise<string> {
    return await invoke("query_play_histories");
}

export async function get_play_history(key: string | null | undefined) {
    if (typeof (key) === 'string') {
        let x = await invoke("get_history", { key: key });
        console.log(x);
        return x;
    }
}