
#
#  Makefile
#  Compiles the Java code into a .jar file
#


.PHONY: java
java:
	mkdir -p ./Resources
	javac -d ./Resources -cp ./Resources/computercraft.jar src/java/*.java
	cd Resources && jar cf ./mimic.jar ./*.class
	rm -f ./Resources/*.class

.PHONY: bundle
osxbundle: java
	rm -rf ./Mimic.app
	mkdir ./Mimic.app
	mkdir ./Mimic.app/Contents
	mkdir ./Mimic.app/Contents/MacOS
	mkdir ./Mimic.app/Contents/Resources
	cp src/config/Info.plist ./Mimic.app/Contents/Info.plist
	cp ./Resources/* ./Mimic.app/Contents/Resources
	cp ./target/mimic ./Mimic.app/Contents/MacOS/mimic
