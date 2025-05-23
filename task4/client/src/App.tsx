import { Route, Routes } from "react-router-dom";
import { HomePage, CountPage, TimePage } from "./routes";
import type { FC } from "react";

const App: FC = () => {
  return (
    <>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/count" element={<CountPage />} />
        <Route path="/time" element={<TimePage />} />
      </Routes>
    </>
  );
};

export default App;
