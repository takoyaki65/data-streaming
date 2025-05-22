import type { FC } from "react";
import { SlidingWindowForm } from "../../components";

const TimePage: FC = () => {
  return (
    <div>
      <h1>Time Page</h1>
      <p>This is the time page.</p>
      <SlidingWindowForm types="Time" />
    </div>
  );
};

export default TimePage;
