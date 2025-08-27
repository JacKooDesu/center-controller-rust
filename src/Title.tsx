import { CommonProps, Mode } from "./App";

interface Props {
  com: CommonProps;
}

export default function Title({ com }: Props) {
  return (
    <div className="row">
      <button onClick={() => com.setMode(Mode.singleMode)}>一對一模式</button>
      <button onClick={() => com.setMode(Mode.multiMode)}>一對多模式</button>
    </div>
  );
}
