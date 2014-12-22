
/*
 * Modem Peripheral
 */


import java.util.List;
import java.util.ArrayList;

import dan200.computercraft.api.lua.ILuaContext;
import dan200.computercraft.api.lua.LuaException;
import dan200.computercraft.api.peripheral.IComputerAccess;
import dan200.computercraft.api.peripheral.IPeripheral;


public class Modem implements IPeripheral, Receiver {

	private static Network network = new Network();

	private IComputerAccess attachedComputer;
	private List<Integer> openChannels;

	/*
	 * Create a new modem peripheral.
	 */
	public Modem() {
		this.attachedComputer = null;
		this.openChannels = new ArrayList<Integer>();
	}

	/*
	 * Parses a channel at the given index from the given set of arguments.
	 */
	private static int parseChannel(Object[] arguments, int index) throws LuaException {
		if (arguments.length <= index || !(arguments[index] instanceof Double)) {
			throw new LuaException("Expected number");
		}

		int channel = (int) ((Double) arguments[index]).doubleValue();
		if (channel < 0 || channel > 65535) {
			throw new LuaException("Expected number in range 0-65535");
		}

		return channel;
	}

	/*
	 * Returns true if the given channel is currently open.
	 */
	public boolean isOpen(int channel) {
		return this.openChannels.contains(new Integer(channel));
	}

	/*
	 * Opens a channel for receiving.
	 */
	public void open(int channel) throws LuaException {
		if (!this.isOpen(channel)) {
			if (this.openChannels.size() >= 128) {
				throw new LuaException("Too many open channels");
			}

			network.listen(this, channel);
			this.openChannels.add(new Integer(channel));
		}
	}

	/*
	 * Closes the channel so that no more messages will be received on it.
	 */
	public void close(int channel) {
		if (this.isOpen(channel)) {
			network.unlisten(this, channel);
			this.openChannels.remove(new Integer(channel));
		}
	}

	/*
	 * Closes all open channels.
	 */
	public void closeAll() {
		for (Integer open : this.openChannels) {
			network.unlisten(this, open);
		}
	}

	/*
	 * Transmits the given message over the given channel.
	 */
	public void transmit(Object message, int channel, int replyChannel) {
		network.transmit(message, channel, replyChannel);
	}

	/*
	 * A callback triggered when a message is received on one of our open channels.
	 */
	public void receive(Object message, int channel, int replyChannel) {
		if (this.attachedComputer != null && this.isOpen(channel)) {
			this.attachedComputer.queueEvent("modem_message", new Object[] {
				this.attachedComputer.getAttachmentName(),
				new Integer(channel),
				new Integer(replyChannel),
				message,
				new Integer(1),
			});
		}
	}


	/*
	 * Peripheral
	 */

	@Override
	public String getType() {
		return "modem";
	}

	@Override
	public String[] getMethodNames() {
		return new String[] {"open", "isOpen", "close", "closeAll", "transmit", "isWireless"};
	}

	@Override
	public Object[] callMethod(IComputerAccess computer, ILuaContext context, int method,
			Object[] arguments) throws LuaException, InterruptedException {
		if (method == 0) {
			// Open
			int channel = Modem.parseChannel(arguments, 0);
			this.open(channel);
		} else if (method == 1) {
			// Is open
			int channel = Modem.parseChannel(arguments, 0);
			return new Object[] {new Boolean(this.isOpen(channel))};
		} else if (method == 2) {
			// Close
			int channel = Modem.parseChannel(arguments, 0);
			this.close(channel);
		} else if (method == 3) {
			// Close all
			this.closeAll();
		} else if (method == 4) {
			// Transmit
			int channel = Modem.parseChannel(arguments, 0);
			int replyChannel = Modem.parseChannel(arguments, 1);
			Object message = arguments.length >= 3 ? arguments[2] : null;
			this.transmit(message, channel, replyChannel);
		} else if (method == 5) {
			// Is wireless
			return new Object[] {new Boolean(true)};
		}

		return new Object[] {};
	}

	@Override
	public void attach(IComputerAccess computer) {
		this.attachedComputer = computer;
	}

	@Override
	public void detach(IComputerAccess computer) {
		this.attachedComputer = null;
	}

	@Override
	public boolean equals(IPeripheral peripheral) {
		return peripheral.getType() == this.getType();
	}

}
