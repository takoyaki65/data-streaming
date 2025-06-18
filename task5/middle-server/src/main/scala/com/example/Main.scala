package com.example

// apache flink
import org.apache.flinkx.api.*
import org.apache.flinkx.api.serializers.*
import org.apache.flink.streaming.api.functions.sink.PrintSinkFunction
import org.apache.flink.streaming.api.windowing.assigners.SlidingProcessingTimeWindows
import java.time.Duration
import org.apache.flink.streaming.api.windowing.windows.TimeWindow
import org.apache.flinkx.api.function.ProcessAllWindowFunction
import org.apache.flink.util.Collector
import upickle.default.*
import org.apache.flink.streaming.api.windowing.windows.GlobalWindow

case class Stock(
    stock: String,
    open: Double,
    high: Double,
    low: Double,
    close: Double,
    timestamp: String
) derives ReadWriter

case class SlidingWindowData(
    stat_data: Array[StatData],
    window_data: Array[WindowData]
) derives ReadWriter

case class StatData(
    stock: String,
    max: Double,
    min: Double,
    mean: Double,
    std_dev: Double
) derives ReadWriter

case class WindowData(
    stock_data: Stock,
    timestamp: String,
    id: Int
) derives ReadWriter

class TimeBasedAggregator
    extends ProcessAllWindowFunction[WindowData, String, TimeWindow] {
  def process(
      context: Context,
      elements: Iterable[WindowData],
      out: Collector[String]
  ): Unit = {
    val stock = elements.head
    val windowData = elements.toArray
    val windowDataPerKey = windowData.groupBy(_.stock_data.stock)
    val statDataPerKey = windowDataPerKey.map {
      case (stockName, windowDataArr) =>
        val max = windowDataArr.map(_.stock_data.high).max
        val min = windowDataArr.map(_.stock_data.low).min
        val mean =
          windowDataArr.map(_.stock_data.close).sum / windowDataArr.length
        val std_dev = math.sqrt(
          windowDataArr
            .map(w => math.pow(w.stock_data.close - mean, 2))
            .sum / windowDataArr.length
        )
        StatData(stockName, max, min, mean, std_dev)
    }.toArray
    val slidingWindowData = SlidingWindowData(statDataPerKey, windowData)
    val json_str = write(slidingWindowData)
    out.collect(json_str)
  }
}

class CountBasedAggregator
    extends ProcessAllWindowFunction[WindowData, String, GlobalWindow] {
  def process(
      context: Context,
      elements: Iterable[WindowData],
      out: Collector[String]
  ): Unit = {
    val windowData = elements.toArray
    val windowDataPerKey = windowData.groupBy(_.stock_data.stock)
    val statDataPerKey = windowDataPerKey.map {
      case (stockName, windowDataArr) =>
        val max = windowDataArr.map(_.stock_data.high).max
        val min = windowDataArr.map(_.stock_data.low).min
        val mean =
          windowDataArr.map(_.stock_data.close).sum / windowDataArr.length
        val std_dev = math.sqrt(
          windowDataArr
            .map(w => math.pow(w.stock_data.close - mean, 2))
            .sum / windowDataArr.length
        )
        StatData(stockName, max, min, mean, std_dev)
    }.toArray
    val slidingWindowData = SlidingWindowData(statDataPerKey, windowData)
    val json_str = write(slidingWindowData)
    out.collect(json_str)
  }
}

object Main {
  def main(args: Array[String]): Unit = {
    // flink stream exec environment
    val env = StreamExecutionEnvironment.getExecutionEnvironment
    // connect socket
    val socketStream = env.socketTextStream("localhost", 5000, '\n')
    env.setParallelism(1)

    // logging
    // stock,open,high,low,close,timestamp
    // Record sample: StockA,309.65,768.04,190.54,415.46,2025-04-25 06:42:20.905995563
    val windowDataStream = socketStream
      .map { line =>
        val fields = line.split(",")
        val stock_data = Stock(
          fields(0),
          fields(1).toDouble,
          fields(2).toDouble,
          fields(3).toDouble,
          fields(4).toDouble,
          fields(5)
        )
        val processing_timestamp_str = new java.text.SimpleDateFormat(
          "yyyy-MM-dd HH:mm:ss.SSS"
        ).format(new java.util.Date())
        WindowData(stock_data, processing_timestamp_str, 0)
      }

    if (args(0) == "time") {
      val downStream = windowDataStream
        .windowAll(
          SlidingProcessingTimeWindows
            .of(Duration.ofSeconds(5), Duration.ofSeconds(2))
        )
        .process(new TimeBasedAggregator)
      downStream.print()

      val iterator = downStream.executeAndCollect()
      while (iterator.hasNext) {
        val json_str = iterator.next()
        println(json_str)
        // websocketServer.broadcast(json_str)
      }
    } else if (args(0) == "count") {
      val downStream = windowDataStream
        .countWindowAll(100, 20)
        .process(new CountBasedAggregator)
      downStream.print()

      val iterator = downStream.executeAndCollect()
      while (iterator.hasNext) {
        val json_str = iterator.next()
        println(json_str)
        // websocketServer.broadcast(json_str)
      }
    } else {
      println("Invalid argument")
      sys.exit(1)
    }

    // env.execute("Start Socket Stream")
  }
}
