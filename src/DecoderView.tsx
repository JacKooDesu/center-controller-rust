import { useState } from "react";

export default function DecoderView() {
    const [jpegData, setJpegData] = useState<string>("");


    return (
        <div>
            <h2>JPEG Decoder</h2>
            <img src={jpegData} alt="Decoded JPEG" width="300px" height="auto" />
        </div>
    );
}
