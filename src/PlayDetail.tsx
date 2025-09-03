import "./PlayHistory.css"
import { useState } from "react";

export interface PlayMissionData {
    name: string;
    time: number;
    complete: boolean;
    stgDatas: Array<PlayStgData>;
}

export interface PlayStgData {
    stgName: string;
    score: number;
    time: number;
}

interface Props {
    missionDatas?: Array<PlayMissionData>
}

export default function PlayDetail({ missionDatas }: Props) {
    const [currentFocus, setCurrentFocus] = useState(0);

    function missionSelect(index: number) {
        setCurrentFocus(index);
    }

    function renderStgDatas() {
        if (missionDatas === undefined || missionDatas.length === 0)
            return <></>;

        return (
            <table className="detail-table">
                <thead>
                    <tr>
                        <th>任務名稱</th>
                        <th>分數</th>
                        <th>時間</th>
                    </tr>
                </thead>
                {missionDatas[currentFocus].stgDatas.map(x =>
                    <tr>
                        <td>{x.stgName}</td>
                        <td>{x.score}</td>
                        <td>{x.time.toFixed(2)}</td>
                    </tr>
                )}
                <tr>
                    <td>--</td>
                    <td>{missionDatas[currentFocus].stgDatas.reduce(
                        (acc, v) => acc += v.score, 0)}</td>
                    <td>{missionDatas[currentFocus].stgDatas.reduce(
                        (acc, v) => acc += v.time, 0).toFixed(2)}</td>
                </tr>
            </table>
        )

        // return missionDatas[currentFocus].stgDatas.map(x =>
        //     <div style={{ display: 'flow' }}>
        //         <table>
        //             <td>{x.stgName}</td>
        //             <td>{x.score}</td>
        //             <td>{x.time}</td>
        //         </table>
        //     </div>
        // );
    }

    return (
        <div style={{
            height: '100%',
            width: '100%',
            display: 'flow',
            marginTop: '-50px',
        }}>
            <div style={{
                display: 'flex',
                height: '10%',
                maxHeight: '50px',
                minHeight: '30px',
                gap: '5px'
            }}>
                {missionDatas?.map((x, i) =>
                    <button
                        className={i === currentFocus ? "selected" : "unselected"}
                        key={x.name}
                        onClick={() => missionSelect(i)}>{x.name}</button>)}
            </div>
            <div style={{
                marginTop: '10px',
                maxHeight: '-webkit-fill-available',
                overflow: 'hidden scroll',
                maxWidth: '-webkit-fill-available'
            }}>
                {renderStgDatas()}
            </div>
        </div >
    )
}