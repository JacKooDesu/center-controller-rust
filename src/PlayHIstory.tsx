import { useEffect, useState } from "react"
import NavBar from "./NavBar"
import { CommonProps } from "./App"
import { get_play_history, query_play_history } from "./RustBridge";

import "./PlayHistory.css"
import fallbackImg from "./assets/loading.svg";
import PlayDetail, { PlayMissionData } from "./PlayDetail";

interface Props {
    com: CommonProps;
}

interface PlayData {
    userId: string;
    missionDatas: Array<PlayMissionData>;
}

export default function PlayHistory({ com }: Props) {
    const [histories, setHistories] = useState<Map<string, string | null>>();
    const [currentData, setCurrentData] = useState<PlayData | null>(null);

    useEffect(() => {
        query_play_history().then(json => {
            let obj = JSON.parse(json);
            setHistories(new Map(Object.entries(obj)));
        });
    }, [com.currentMode]);

    async function handlePlayerSelect(name: string) {
        let key = histories?.get(name);
        let data = (await get_play_history(key) as PlayData);
        console.log(data);
        setCurrentData(data);
    }

    function playerSelector() {
        if (histories === undefined)
            return (<img src={fallbackImg} alt='loading'></img>);

        return (
            <div style={{
                display: 'block',
                width: '20%',
                maxHeight: '75vh',
                maxWidth: '200px',
            }}>
                <div style={{
                    maxHeight: '100%',
                    display: 'grid',
                    alignItems: 'start',
                    gap: '8px',
                    overflow: 'hidden scroll',
                }}>
                    {Array.from(histories?.keys())
                        .map(k => (<button
                            className={k === currentData?.userId ? "selected" : "unselected"}
                            key={k}
                            onClick={() => handlePlayerSelect(k)}>{k}</button>))}
                </div>
            </div>
        );
    }

    return (
        <>
            <NavBar com={com}></NavBar>
            <div style={{
                display: 'block',
                position: 'absolute',
                top: '60px',
                right: '5px',
                left: '5px',
                bottom: '10px'
            }}>
                <div style={{
                    display: 'flex',
                    flexDirection: 'row',
                    minHeight: 'inhirit',
                    height: '100%'
                }}>
                    {playerSelector()}
                    <PlayDetail missionDatas={currentData?.missionDatas}></PlayDetail>
                </div>
            </div>
        </>
    )
}