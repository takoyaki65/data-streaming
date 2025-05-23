import type { FC } from "react";
import type { StatDataType } from "../../model/slidingWindowData";
import { BarChart } from "@mui/x-charts/BarChart";

type Props = {
  result: StatDataType[];
};

const chartSetting = {
  yAxis: [
    {
      label: "Price",
      width: 60,
    },
  ],
  height: 300,
};

const SlidingWindowChart: FC<Props> = (props) => {
  return (
    <div>
      <h1>Sliding Window Chart</h1>
      <BarChart
        dataset={props.result}
        xAxis={[{ dataKey: "stock" }]}
        series={[
          { dataKey: "max", label: "Max" },
          { dataKey: "min", label: "Min" },
          { dataKey: "mean", label: "Mean" },
          { dataKey: "std_dev", label: "Standard Deviation" },
        ]}
        {...chartSetting}
      />
    </div>
  );
};

export default SlidingWindowChart;
