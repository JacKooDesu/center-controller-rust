import { useEffect, useState } from "react";
import { addJpgDecodedListener } from "./RustBridge";

interface Props {
    addr: string;
    // setFocus: (key: string) => void;
}

export default function DecoderView({ addr }: Props) {
    const [jpegUrl, setJpeg] = useState<string>("");
    useEffect(() => {
        addJpgDecodedListener(addr, bytes => {
            updateJpeg(bytes);
        });
    }, [addr]);

    function updateJpeg(bytes: []) {
        if (jpegUrl.length > 0)
            URL.revokeObjectURL(jpegUrl);

        const blob = new Blob([new Uint8Array(bytes)], { type: "image/jpeg" });
        const url = URL.createObjectURL(blob);

        setJpeg(url);
    }

    return (
        <>
            <img src={jpegUrl} alt={addr} width="100%" height="auto" />
        </>
    );
}
