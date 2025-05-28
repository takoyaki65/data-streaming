package work.yukiosada.data_streaming.middle_server;

import javax.websocket.CloseReason;
import javax.websocket.OnClose;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

@ServerEndpoint("/")
public class App {
    @OnMessage
    public String connectClient(String message) {
        // Clientからの接続を確認
        System.out.println("Messgae from client: " + message);
        return ("Messgae from client:"  + message);
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
