import { useEffect, useState } from "react";
import { addJpgDecodedListener } from "./RustBridge";

import fallbackImg from "./assets/loading.svg";

// const fallbackB64 = 'data:image/svg+xml;base64,' + btoa(unescape(encodeURIComponent(fallbackImg)));

interface Props {
    addr: string;
    setFocus?: ((addr: string) => void) | null;
}

let jpegUrls: string[] = [];

function disposeJpegs() {
    let url: string | undefined = undefined;
    while ((url = jpegUrls.pop()) !== undefined) {
        URL.revokeObjectURL(url);
    }
}

export default function DecoderView({ addr, setFocus }: Props) {
    const [, setJpegVersion] = useState<number>(0);
    const [error, setError] = useState<boolean>(false);

    useEffect(() => {
        if (addr === undefined)
            return;
        console.log("registering listener for", addr);
        addJpgDecodedListener(addr, bytes => {
            updateJpeg(bytes);
        });

        return () => disposeJpegs();
    }, [addr]);

    function updateJpeg(bytes: []) {
        disposeJpegs();

        const blob = new Blob([new Uint8Array(bytes)], { type: "image/jpeg" });
        const url = URL.createObjectURL(blob);

        jpegUrls.push(url);

        setError(false);
        setJpegVersion(v => v + 1);
    }

    return (
        <>
            <img src={error || addr === undefined ? fallbackImg : jpegUrls[0]}
                alt={addr}
                onError={() => setError(true)}
                onClick={() => setFocus && setFocus(addr)} />
        </>
    );
}
