import { Route, Routes } from "react-router-dom";
import { HomePage, CountPage, TimePage } from "./routes";
import type { FC } from "react";

const App: FC = () => {
  return (
    <>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/time" element={<CountPage />} />
        <Route path="/count" element={<TimePage />} />
      </Routes>
    </>
  );
};

export default App;
