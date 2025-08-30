import { useEffect, useState } from "react"
import NavBar from "./NavBar"
import { CommonProps } from "./App"
import { query_play_history } from "./RustBridge";

interface Props {
    com: CommonProps;
}

export default function PlayHistory({ com }: Props) {
    const [histories, setHistories] = useState<Map<string, string | null>>();

    useEffect(() => {
        query_play_history().then(json => {
            let obj = JSON.parse(json);
            setHistories(new Map(Object.entries(obj)));
        });
    }, [com.currentMode]);

    return (
        <div>
            <NavBar com={com}></NavBar>

            {
                histories === undefined ?
                    (<></>) :
                    Array.from(histories?.keys()).map(k => (<li key={k}>{k}</li>))
            }
            <ul>
            </ul>
        </div>
    )
}