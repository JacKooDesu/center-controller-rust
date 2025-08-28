import { useReducer } from "react";
// import reactLogo from "./assets/react.svg";
import "./App.css";
import Title from "./Title";
import GameViewScreenBase from "./GameViewScreenBase";

export enum Mode {
  title,
  singleMode,
  multiMode,
  layoutTest
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

function ModeReducer(_current: Mode, action: SetModeAction): Mode {
  return action.value;
}

function App() {
  const [currentMode, modeDispatcher] = useReducer(ModeReducer, Mode.title);
  // const [currentMode, setCurrentMode] = useState<mode>(mode.title);

  const commonProps = {
    setMode: (value: Mode) => modeDispatcher(new SetMode(value)),
    currentMode: currentMode
  };

  let content = (() => {
    switch (currentMode) {
      case Mode.title:
        return (
          <Title com={commonProps}>

          </ Title>
        );

      case Mode.singleMode:
      case Mode.multiMode:
      case Mode.layoutTest:
        return (
          <GameViewScreenBase com={commonProps}></GameViewScreenBase>
        );
    }
  })();

  return (
    <div className="container">
      {content}
    </div>
  )
}


export default App;
