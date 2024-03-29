
/*
 * Minion
 *
 * Compile with:
 * javac -cp Resources/computercraft.jar -d ./Resources src/Minion.java
 */


import java.io.File;
import java.io.IOException;
import java.util.ArrayList;

import dan200.computercraft.api.filesystem.IMount;
import dan200.computercraft.api.filesystem.IWritableMount;
import dan200.computercraft.core.filesystem.ComboMount;
import dan200.computercraft.core.filesystem.JarMount;
import dan200.computercraft.core.filesystem.FileMount;
import dan200.computercraft.core.computer.IComputerEnvironment;
import dan200.computercraft.core.computer.Computer;
import dan200.computercraft.core.terminal.Terminal;


public class Minion implements IComputerEnvironment {

	public int id;
	public boolean advanced;

	private double ticks;

	private Computer computer;
	private Terminal terminal;

	private String storageDirectory;
	private String romDirectory;

	private long spaceLimit;

	public Minion(int id, boolean advanced, int width, int height,
			String storageDirectory, String romDirectory, long spaceLimit) {
		this.id = id;
		this.advanced = advanced;
		this.storageDirectory = storageDirectory;
		this.romDirectory = romDirectory;
		this.spaceLimit = spaceLimit;
		this.terminal = new Terminal(width, height);
		this.computer = new Computer(this, this.terminal, id);
		this.computer.turnOn();
	}

	public String getLine(int line) {
		return this.terminal.getLine(line);
	}

	public String getColorLine(int line) {
		return this.terminal.getColourLine(line);
	}

	public void advance(double delta) {
		this.ticks += delta * 20;
		this.computer.advance(delta);
	}

	public void destroy() {
		this.computer.shutdown();
		this.computer.unload();
	}


	/*
	 * Properties
	 */

	public int getCursorX() {
		return this.terminal.getCursorX();
	}

	public int getCursorY() {
		return this.terminal.getCursorY();
	}

	public int getCursorColor() {
		return this.terminal.getTextColour();
	}

	public boolean getCursorBlink() {
		return this.terminal.getCursorBlink();
	}


	/*
	 * Events
	 */

	public void attachModem() {
		Modem peripheral = new Modem();
		this.computer.setPeripheral(1, peripheral);
	}

	public void detachModem() {
		this.computer.setPeripheral(2, null);
	}

	public void paste(String contents) {
		this.computer.queueEvent("paste", new Object[] {contents});
	}

	public void keyEvent(int key) {
		this.computer.queueEvent("key", new Object[] {new Integer(key)});
	}

	public void charEvent(String character) {
		this.computer.queueEvent("char", new Object[] {character});
	}

	public void pasteEvent(String text) {
		this.computer.queueEvent("paste", new Object[] {text});
	}

	public void mouseClickEvent(int button, int x, int y) {
		this.computer.queueEvent("mouse_click", new Object[] {
			new Integer(button),
			new Integer(x),
			new Integer(y),
		});
	}

	public void mouseDragEvent(int button, int x, int y) {
		this.computer.queueEvent("mouse_drag", new Object[] {
			new Integer(button),
			new Integer(x),
			new Integer(y),
		});
	}

	public void mouseScrollEvent(int direction, int x, int y) {
		this.computer.queueEvent("mouse_scroll", new Object[] {
			new Integer(direction),
			new Integer(x),
			new Integer(y),
		});
	}

	public void terminate() {
		this.computer.queueEvent("terminate", new Object[] {});
	}

	public void shutdown() {
		this.computer.shutdown();
	}

	public void reboot() {
		if (this.computer.isOn()) {
			this.computer.reboot();
		} else {
			this.computer.turnOn();
		}
	}


	/*
	 * Computer API
	 */

	@Override
	public int getDay() {
		return (int)(1.0D + Math.floor(((long) this.ticks) / 24000L));
	}

	@Override
	public double getTimeOfDay() {
		return this.ticks % 24000.0D / 1000.0D;
	}

	@Override
	public boolean isColour() {
		return this.advanced;
	}

	@Override
	public long getComputerSpaceLimit() {
		return this.spaceLimit;
	}

	@Override
	public int assignNewID() {
		return this.id;
	}

	@Override
	public IWritableMount createSaveDirMount(String path, long size) {
		// To create the save folder for a computer
		// Example:
		//  path: computer/0
		//  capacity: 2097152

		String[] components = path.split(File.separator);
		String computerID = components[components.length - 1];
		File file = new File(this.storageDirectory + File.separator + "computers", computerID);
		return new FileMount(file, size);
	}

	@Override
	public IMount createResourceMount(String origin, String path) {
		// To load resources (eg. /rom)
		// Example:
		//  origin: computercraft
		//  path: lua/rom

		try {
			String root = Computer.class.getProtectionDomain().getCodeSource()
				.getLocation().getPath().replace("%20", " ");
			if (root.indexOf("!") != -1) {
				root = root.substring(0, root.indexOf("!"));
			}

			File location = new File(root);

			// Add the ROM folder
			ArrayList<IMount> mounts = new ArrayList<IMount>();
			String assets = "assets/" + origin + "/" + path;
			IMount jarMount = new JarMount(location, assets);
			mounts.add(jarMount);

			// Add additional programs folder
			File additional = new File(romDirectory);
			if (additional.exists() && additional.isDirectory()) {
				mounts.add(new FileMount(additional, 0L));
			}

			if (mounts.size() > 1) {
				IMount[] mountArray = new IMount[mounts.size()];
				mounts.toArray(mountArray);
				return new ComboMount(mountArray);
			}

			return jarMount;
		} catch (IOException e) {
			e.printStackTrace();
		}

		return null;
	}

}
