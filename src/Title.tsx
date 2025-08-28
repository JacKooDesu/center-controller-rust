import { CommonProps, Mode } from "./App";

interface Props {
  com: CommonProps;
}

export default function Title({ com }: Props) {
  return (
    <>
      <h1>Center Controller Extend</h1>
      <div className="row">
        <button onClick={() => com.setMode(Mode.singleMode)}>一對一</button>
        <button onClick={() => com.setMode(Mode.multiMode)}>一對多</button>
        <button>學習歷程</button>
        <button onClick={() => com.setMode(Mode.layoutTest)}>測試</button>
      </div>
    </>
  );
}
