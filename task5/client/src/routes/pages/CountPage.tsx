import { useState, type FC } from "react";
import {
  SlidingWindowChart,
  SlidingWindowForm,
  SlidingWindowTable,
} from "../../components";
import type { SlidingWindowDataType } from "../../model/slidingWindowData";

const CountPage: FC = () => {
  // init result useState
  const [result, setResult] = useState<SlidingWindowDataType | null>(null);
  return (
    <div>
      <h1>Count Page</h1>
      <p>This is the Count page.</p>
      <SlidingWindowForm types="count" setResult={setResult} />
      {typeof result?.window_data == "undefined" ? (
        <></>
      ) : (
        <SlidingWindowChart result={result?.stat_data} />
      )}
      {typeof result?.window_data == "undefined" ? (
        <></>
      ) : (
        <SlidingWindowTable result={result?.window_data} />
      )}
    </div>
  );
};

export default CountPage;
