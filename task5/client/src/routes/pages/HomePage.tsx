import type { FC } from "react";
import { Link } from "react-router-dom";

const HomePage: FC = () => {
  return (
    <div>
      <Link to={`/count`}>Count Window</Link>
      <Link to={`/time`}>Time Window</Link>
    </div>
  );
};

export default HomePage;
