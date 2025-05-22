import type { FC } from "react";
import { SlidingWindowForm } from "../../components";

const CountPage: FC = () => {
  return (
    <div>
      <h1>Count Page</h1>
      <p>This is the Count page.</p>
      <SlidingWindowForm types="Count" />
    </div>
  );
};

export default CountPage;
