import { useReducer } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Title from "./Title";
import SingleModeScreen from "./SingleModeScreen";
import GameViewScreenBase from "./GameViewScreenBase";

export enum Mode {
  title,
  singleMode,
  multiMode
}

export interface CommonProps {
  setMode: (value: Mode) => void;
  currentMode: Mode;
}

type SetModeAction =
  | SetMode;

class SetMode {
  constructor(value: Mode) {
    this.value = value;
  }
  value: Mode = Mode.title;
}

function ModeReducer(current: Mode, action: SetModeAction): Mode {
  return action.value;
}

function App() {
  const [currentMode, modeDispatcher] = useReducer(ModeReducer, Mode.title);
  // const [currentMode, setCurrentMode] = useState<mode>(mode.title);

  const commonProps = {
    setMode: (value: Mode) => modeDispatcher(new SetMode(value)),
    currentMode: currentMode
  }

  async function backTitle() {
    await invoke("stop_udp");
    modeDispatcher(new SetMode(Mode.title));
  }

  async function startUdp() {
    await invoke("start_udp");
  }

  async function stopUdp() {
    await invoke("stop_udp");
  }

  switch (currentMode) {
    case Mode.title:
      return (
        <Title com={commonProps}>

        </ Title>
      );

    case Mode.singleMode:
    case Mode.multiMode:
      return (
        <GameViewScreenBase com={commonProps}></GameViewScreenBase>
      );
  }
}

export default App;
