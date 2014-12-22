
/*
 * Network
 */


import java.util.Map;
import java.util.HashMap;
import java.util.List;
import java.util.ArrayList;


public class Network {

	private static Map<Integer, List<Receiver>> listeners =
		new HashMap<Integer, List<Receiver>>();

	/*
	 * Attach a receiver to be notified of messages on a particular channel
	 */
	public static void listen(Receiver receiver, int channel) {
		if (listeners.containsKey(new Integer(channel))) {
			listeners.get(new Integer(channel)).add(receiver);
		} else {
			List<Receiver> receivers = new ArrayList<Receiver>();
			receivers.add(receiver);
			listeners.put(new Integer(channel), receivers);
		}
	}

	/*
	 * Remove a receiver from listening to the given channel.
	 */
	public static void unlisten(Receiver receiver, int channel) {
		if (listeners.containsKey(new Integer(channel))) {
			listeners.get(new Integer(channel)).remove(receiver);
		}
	}

	/*
	 * Transmit a given message on the given channel.
	 */
	public static void transmit(Object message, int channel, int replyChannel) {
		if (listeners.containsKey(new Integer(channel))) {
			for (Receiver receiver : listeners.get(new Integer(channel))) {
				receiver.receive(message, channel, replyChannel);
			}
		}
	}

}
