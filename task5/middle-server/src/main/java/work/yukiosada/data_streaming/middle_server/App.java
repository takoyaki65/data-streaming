package work.yukiosada.data_streaming.middle_server;

import jakarta.websocket.OnClose;
import jakarta.websocket.OnMessage;
import jakarta.websocket.OnOpen;
import jakarta.websocket.Session;
import jakarta.websocket.server.ServerEndpoint;

@ServerEndpoint("/")
public class App {

    @OnMessage
    public String connectClient(String message) {
        // Clientからの接続を確認
        System.out.println("Message from client: " + message);
        return ("Message from client:" + message);
    }

    @OnOpen
    public void onOpen(Session session) {
        // セッションが確立
        System.out.println("Establish Connection!");
    }

    @OnClose
    public void onClose(Session session) {
        // セッションを終了する際の処理を実装
        System.out.println("Close Connection!");
    }
}
