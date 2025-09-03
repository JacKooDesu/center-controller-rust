import { useEffect, useState } from "react";
import { addJpgDecodedListener } from "./RustBridge";

import fallbackImg from "./assets/loading.svg";

// const fallbackB64 = 'data:image/svg+xml;base64,' + btoa(unescape(encodeURIComponent(fallbackImg)));

interface Props {
    addr: string;
    setFocus?: ((addr: string) => void) | null;
}

export default function DecoderView({ addr, setFocus }: Props) {
    const [, setJpegVersion] = useState<number>(0);
    const [error, setError] = useState<boolean>(true);
    const [jpegUrls, updateJpegUrls] = useState<string[]>([]);

    function disposeAppend(arr: string[], newData?: string) {
        let url: string | undefined = undefined;
        while (url = arr.pop()) {
            URL.revokeObjectURL(url);
        }
        if (newData)
            return [...arr, newData];
        else
            return arr;
    }

    useEffect(() => {
        if (addr === undefined)
            return;
        console.log("registering listener for", addr);
        addJpgDecodedListener(addr, bytes => {
            updateJpeg(bytes);
        });

        return () => {
            console.log(addr + " decoder view exited!");
            disposeAppend(jpegUrls);
        }
    }, [addr]);

    function updateJpeg(bytes: []) {
        const blob = new Blob([new Uint8Array(bytes)], { type: "image/jpeg" });
        const url = URL.createObjectURL(blob);

        updateJpegUrls(prev => disposeAppend(prev, url));

        setError(false);
        setJpegVersion(v => v + 1);
    }

    return (
        <>
            <img src={error || addr === undefined ? fallbackImg : jpegUrls[jpegUrls.length - 1]}
                alt={addr}
                onError={() => setError(true)}
                onClick={() => setFocus && setFocus(addr)} />
        </>
    );
}
