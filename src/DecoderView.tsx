import { useEffect, useState } from "react";
import { addJpgDecodedListener } from "./RustBridge";

import fallbackImg from "./assets/loading.svg";

// const fallbackB64 = 'data:image/svg+xml;base64,' + btoa(unescape(encodeURIComponent(fallbackImg)));

interface Props {
    addr: string;
    setFocus?: ((addr: string) => void) | null;
}

let jpegUrlLast: string = "";

export default function DecoderView({ addr, setFocus }: Props) {
    const [jpegUrl, setJpeg] = useState<string>("");
    const [error, setError] = useState<boolean>(false);

    useEffect(() => {
        if (addr === undefined)
            return;
        console.log("registering listener for", addr);
        addJpgDecodedListener(addr, bytes => {
            updateJpeg(bytes);
        });
    }, [addr]);

    function updateJpeg(bytes: []) {
        if (jpegUrlLast.length > 0) {
            URL.revokeObjectURL(jpegUrlLast);
        }

        const blob = new Blob([new Uint8Array(bytes)], { type: "image/jpeg" });
        const url = URL.createObjectURL(blob);

        jpegUrlLast = url;

        setError(false);
        setJpeg(url);
    }

    return (
        <>
            <img src={error || addr === undefined ? fallbackImg : jpegUrl}
                alt={addr}
                onError={() => setError(true)}
                onClick={() => setFocus && setFocus(addr)} />
        </>
    );
}
