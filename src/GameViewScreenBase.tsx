import { useEffect, useRef, useState } from "react";
import { CommonProps, Mode } from "./App";
import { addClientChangeListener, ClientChangedData, startUdp, stopUdp } from "./RustBridge";
import DecoderView from "./DecoderView";

interface GameViewProps {

}

interface Props {
    com: CommonProps;
}

export default function GameViewScreenBase({ com }: Props) {
    const [clients, setClients] = useState<string[]>([]);
    const [focusing, setFocusing] = useState<string>("");

    useEffect(() => {
        console.log("Initialize Network!");

        startUdp();
        addClientChangeListener("GameViewScreenBase", onClientChange);
    }, [com.currentMode])

    async function onReturnClick() {
        await stopUdp();
        com.setMode(Mode.title)
    }

    async function onClientChange(data: ClientChangedData) {
        console.log(data.payload);

        let arr = clients;
        if (data.payload.add) {
            setClients([...arr, data.payload.add]);

        }
        else if (data.payload.remove) {
            setClients(arr.filter(c => c != data.payload.remove));

        }
    }

    console.log("Clients:", clients);

    return (
        <div>
            <button onClick={onReturnClick}>返回選單</button>
            <br />
            {com.currentMode === Mode.singleMode ?
                <DecoderView key={clients[0]} addr={clients[0]}></DecoderView> :
                clients.map(c => <DecoderView key={c} addr={c}></DecoderView>)
            }
        </div >
    );
}
