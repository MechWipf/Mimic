
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