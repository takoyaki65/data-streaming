package com.example

import org.apache.flinkx.api.*
import org.apache.flinkx.api.serializers.*
import org.apache.flink.streaming.api.functions.sink.PrintSinkFunction

case class Stock(
    stock: String,
    open: Double,
    high: Double,
    low: Double,
    close: Double,
    timestamp: String
)

object App {
  def main(args: Array[String]): Unit = {
    // flink stream exec environment
    val env = StreamExecutionEnvironment.getExecutionEnvironment
    // connect socket
    val socketStream = env.socketTextStream("localhost", 5000)
    env.setParallelism(1)

    // logging
    // stock,open,high,low,close,timestamp
    // Record sample: StockA,309.65,768.04,190.54,415.46,2025-04-25 06:42:20.905995563
    socketStream
      .map { line =>
        println(s"RECEIVED: $line")
        line
      }
      .print()

    env.execute("Start Socket Stream")
  }
}
