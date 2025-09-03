import { addClientChangeListener, addHistorySavedListener } from "./RustBridge.tsx";
import { CommonProps } from "./App.tsx"
import { useEffect, useState } from "react";

import "./MessageQueue.css"

interface Props {
    com: CommonProps,
}

interface MessageQueueItem {
    msg: string;
    out: boolean;
    id: number;
}

const showMiliSec = 3000;
const slideOutDuration = 500;
const slideOutAni = `slide-out ${slideOutDuration / 1000}s forwards`;

interface MessageQueueState {
    arr: Array<MessageQueueItem>;
    version: number;
}

export default function MessageQueue({ com }: Props) {
    const [messageQueue, updateMessageQueue] = useState<MessageQueueState>({
        arr: [],
        version: 0
    });

    useEffect(() => {
        addHistorySavedListener("messagePopup", (data) => {
            appendMessage(`使用者 ${data.userId} 遊玩資料已儲存至 ${data.filePath}。`);
            updateMessageQueue(({ arr, version }) => ({
                arr,
                version: version + 1
            }));
        });

        addClientChangeListener("messagePopup", (data) => {
            if (data.add) {
                appendMessage(`使用者 ${data.add} 已連入。`);
            }
            if (data.remove) {
                appendMessage(`使用者 ${data.remove} 已斷線。`);
            }
            updateMessageQueue(({ arr, version }) => ({
                arr,
                version: version + 1
            }));
        });
    }, [com.currentMode]);

    async function appendMessage(msg: string) {
        updateMessageQueue(({ arr, version }) => ({
            arr: [...arr, { msg, out: false, id: version }],
            version: version + 1
        }));
        setTimeout(() => updateMessageQueue(({ arr, version }) => {
            let first = arr[0];
            first.out = true;
            return { arr: [...arr], version };
        }), showMiliSec);
        setTimeout(() => updateMessageQueue(({ arr, version }) => ({
            arr: arr.slice(1),
            version
        })), slideOutDuration + showMiliSec);
    }

    return (
        <>
            <div className="message-queue-container">
                {messageQueue.arr.map((item) => (
                    <QueueItemElement
                        key={item.id}
                        data={item} />
                ))}
            </div>

            {/* <button onClick={() => {
                appendMessage(messageQueue.version.toString());
            }}>
                Add Message
            </button>
            <button onClick={() => {
                updateMessageQueue(({ arr, version }) => ({
                    arr: arr.slice(1),
                    version
                }));
            }}>
                Pop Message
            </button> */}
        </>
    );
}

interface QueueItemElementProp {
    // currentIter: number;
    data: MessageQueueItem;
}

function QueueItemElement({ data }: QueueItemElementProp) {
    return (
        <div className="message-popup"
            style={{ animation: data.out ? slideOutAni : "" }}>
            <div>
                <p>{data.msg}</p>
            </div>
        </div>
    );
}