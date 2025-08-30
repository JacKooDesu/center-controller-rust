import { ReactNode } from "react";
import { CommonProps, Mode } from "./App";

interface Props {
    com: CommonProps;
    onReturnCb?: () => void;
    children?: ReactNode;
}

export default function NavBar({ com, children, onReturnCb }: Props) {
    async function onReturnClick() {
        (onReturnCb || (() => { }))();
        com.setMode(Mode.title)
    }

    return (
        <div className="navbar">
            <button onClick={onReturnClick}>返回</button>
            <div style={{ display: "flex", gap: "8px" }}>
                {children}
            </div>
        </div>
    );

}