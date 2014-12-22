
/*
 * Receiver
 */


public interface Receiver {

	public void receive(Object message, int channel, int replyChannel);

}
