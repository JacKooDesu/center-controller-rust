import { useEffect, useState } from "react";
import { CommonProps, Mode } from "./App";
import { addClientChangeListener, ClientChangedData, send, startUdp, stopUdp } from "./RustBridge";
import DecoderView from "./DecoderView";
import "./GameViewScreen.css"
import NavBar from "./NavBar";

interface Props {
    com: CommonProps;
}

let clients: string[] = [];

export default function GameViewScreenBase({ com }: Props) {
    const [, setClientCount] = useState<number>(0);
    const [focusTarget, setFocusing] = useState<string>("");
    const IsFocusing = () =>
        com.currentMode === Mode.multiMode &&
        focusTarget.length > 0;
    const handleFocus = com.currentMode === Mode.singleMode ?
        async () => { } :
        async (s: string) => {
            clients.forEach(async (client) => {
                await send(client, client === s ? "screen2" : "screen5");
            });
            console.log("Focus changed to:", s);
            setFocusing(s);
        };

    useEffect(() => {
        console.log("Initialize Network!");

        startUdp();
        addClientChangeListener("GameViewScreenBase", onClientChange);

        return () => { clients = []; };
    }, [com.currentMode]);

    function navbarElement() {
        async function onResizeClick() {
            if (!IsFocusing()) return;

            await handleFocus("");
        }

        async function onExportClick() {
            if (com.currentMode === Mode.singleMode) {
                await send(clients[0], "export");
            } else if (com.currentMode === Mode.multiMode && IsFocusing()) {
                await send(focusTarget, "export");
            }
        }

        const resizeBtn =
            (com.currentMode === Mode.multiMode && IsFocusing()) ?
                (<button onClick={onResizeClick}>縮放</button>) :
                (<></>);

        const exportBtn =
            (com.currentMode === Mode.singleMode ||
                (com.currentMode === Mode.multiMode && IsFocusing())) ?
                (<button onClick={onExportClick}>導出學習歷程</button>) :
                (<></>);

        return (
            <NavBar com={com} onReturnCb={async () => await stopUdp()}>
                {
                    clients.length > 0 && (
                        <>
                            {exportBtn}
                            {resizeBtn}
                        </>
                    )
                }
            </NavBar >
        );
    }

    async function onClientChange({ add, remove }: ClientChangedData) {
        if (add) {
            if (com.currentMode === Mode.singleMode) {
                if (clients.length == 0)
                    await send(add, "screen1");
            } else {
                await send(add, IsFocusing() ? "screen5" : "screen2");
            }

            clients.push(add);
        } else if (remove) {
            clients = clients.filter(c => c != remove);
        }

        setClientCount(clients.length);
    }

    console.log("Clients:", clients);

    return (
        <div>
            {navbarElement()}
            {com.currentMode === Mode.singleMode ?
                (
                    <div className="singleView">
                        <DecoderView addr={clients[0]}></DecoderView>
                    </div>
                ) :
                IsFocusing() ?
                    (
                        <div className="singleView">
                            <DecoderView addr={focusTarget}></DecoderView>
                        </div>
                    ) :
                    (
                        <div className="multiView">
                            {clients.map(c => <DecoderView key={c} addr={c} setFocus={handleFocus} />)}
                        </div>
                    )
            }
        </div >
    );
}
