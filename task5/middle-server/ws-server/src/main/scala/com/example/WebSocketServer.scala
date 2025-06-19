import cats.effect.*
import cats.effect.std.Queue
import cats.syntax.all.*
import fs2.concurrent.Topic
import fs2.{Pipe, Stream}
import org.http4s.*
import org.http4s.dsl.io.*
import org.http4s.ember.server.EmberServerBuilder
import org.http4s.server.websocket.WebSocketBuilder2
import org.http4s.websocket.WebSocketFrame
import com.comcast.ip4s.*
import scala.concurrent.duration.*

object WebSocketServer extends IOApp:

  // コネクション管理用のケースクラス
  case class ConnectionState(
      id: String,
      queue: Queue[IO, String]
  )

  // 各エンドポイント用のコネクション管理
  case class ConnectionManager(
      connections: Ref[IO, Map[String, ConnectionState]],
      topic: Topic[IO, String]
  )

  def run(args: List[String]): IO[ExitCode] =
    for
      // /time エンドポイント用のマネージャー
      timeManager <- createConnectionManager()
      // /count エンドポイント用のマネージャー
      countManager <- createConnectionManager()

      // サーバー起動
      _ <- EmberServerBuilder
        .default[IO]
        .withHost(ipv4"0.0.0.0")
        .withPort(port"7000")
        .withHttpWebSocketApp(wsApp(timeManager, countManager))
        .build
        .useForever
    yield ExitCode.Success

  def createConnectionManager(): IO[ConnectionManager] =
    for
      connections <- Ref.of[IO, Map[String, ConnectionState]](Map.empty)
      topic <- Topic[IO, String]
    yield ConnectionManager(connections, topic)

  def wsApp(
      timeManager: ConnectionManager,
      countManager: ConnectionManager
  )(wsb: WebSocketBuilder2[IO]): HttpApp[IO] =
    HttpRoutes
      .of[IO] {
        case GET -> Root / "time" =>
          handleBroadcastEndpoint(wsb, timeManager)

        case GET -> Root / "time-submit" =>
          handleSubmitEndpoint(wsb, timeManager)

        case GET -> Root / "count" =>
          handleBroadcastEndpoint(wsb, countManager)

        case GET -> Root / "count-submit" =>
          handleSubmitEndpoint(wsb, countManager)
      }
      .orNotFound

  // ブロードキャストエンドポイント（/time, /count）のハンドラー
  def handleBroadcastEndpoint(
      wsb: WebSocketBuilder2[IO],
      manager: ConnectionManager
  ): IO[Response[IO]] =
    val connectionId = java.util.UUID.randomUUID().toString

    for
      queue <- Queue.unbounded[IO, String]
      _ <- manager.connections.update(
        _.updated(connectionId, ConnectionState(connectionId, queue))
      )

      // キューからデータを読み取ってクライアントに送信
      toClient = Stream
        .repeatEval(queue.take)
        .map(msg => WebSocketFrame.Text(msg))

      // クライアントからのメッセージは無視（一方向通信）
      fromClient: Pipe[IO, WebSocketFrame, Unit] = _.evalMap {
        case WebSocketFrame.Close(_) =>
          manager.connections.update(_.removed(connectionId))
        case _ =>
          IO.unit
      }

      // トピックを購読してキューにデータを追加
      _ <- manager.topic
        .subscribe(10)
        .evalMap(msg => queue.offer(msg))
        .compile
        .drain
        .background
        .allocated

      response <- wsb.build(toClient, fromClient)
    yield response

  // サブミットエンドポイント（/time-submit, /count-submit）のハンドラー
  def handleSubmitEndpoint(
      wsb: WebSocketBuilder2[IO],
      manager: ConnectionManager
  ): IO[Response[IO]] =
    // クライアントからのメッセージを受信してブロードキャスト
    val fromClient: Pipe[IO, WebSocketFrame, Unit] = _.evalMap {
      case WebSocketFrame.Text(message, _) =>
        for
          connections <- manager.connections.get
          _ <- connections.values.toList.traverse { conn =>
            conn.queue.offer(message)
          }
        yield ()
      case _ =>
        IO.unit
    }

    // サーバーからはメッセージを送信しない（一方向通信）
    val toClient = Stream.empty

    wsb.build(toClient, fromClient)
