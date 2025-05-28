export type SlidingWindowDataType = {
  stat_data: StatDataType[];
  window_data: WindowDataType[];
};

// stat_data
export type StatDataType = {
  stock: StockType;
  max: number;
  min: number;
  mean: number;
  std_dev: number;
};

// window_data
export type WindowDataType = {
  stock_data: StockDataType;
  timestamp: string;
  id: number;
};

type StockDataType = {
  stock: StockType;
  open: number;
  high: number;
  low: number;
  close: number;
  timestamp: string;
};

// stock type
type StockType =
  | "StockA"
  | "StockB"
  | "StockC"
  | "StockD"
  | "StockE"
  | "StockF"
  | "StockG"
  | "StockH"
  | "StockI"
  | "StockJ"
  | "StockK"
  | "StockL"
  | "StockM"
  | "StockN"
  | "StockO"
  | "StockP"
  | "StockQ"
  | "StockR"
  | "StockS"
  | "StockT"
  | "StockU"
  | "StockV"
  | "StockW"
  | "StockX"
  | "StockY"
  | "StockZ";
