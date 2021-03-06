# Command code generator
This folder contains python code to parse reports generated by mojang's server jar file, and generate the rust code needed to parse all valid commands. 
These reports are the same as the ones used by feather-data. If you want to read more about them check out this [link](https://wiki.vg/Data_Generators). This wiki is excellent, and you should check it out, especially the section on the [Command graph](https://wiki.vg/Command_Data). At the bottom of this page someone has generated a [image](https://wiki.vg/File:Command_graph_18w22c.png), that you might not have seen. That page is considered mandatory reading to understand what this tool does.

At the time of writing this, the utility has not been used on any os except ubuntu 20.04.


# How to use
1) Download the server jar of the version of minecraft you want to generate the commands for. 

2) In a terminal run this command to get the commands.json file. 
	java -cp minecraft_server.jar net.minecraft.data.Main --release

3) Put the generated commands file in this folder.

4) Make a copy of all the files in ./generated/ so that you can use a diffing tool to determine what new commands have been generated.

5) Run this command in a terminal
	python3 ./main.py
	
6) If your version of minecraft introduced a new parser, then you need to give them a name. If prompted by the output of ./main.py, then edit ./mapping_and_logs/parser_mapping.json, so that the new parsers are given a name. Remember that these names are names of Rust structs, so they must be names that could compile. After giving *All* the new parsers a new name, then run the command from step 5) again.

7) The ./generated folder now contains new implementations for all non-infinitly recursive commands. You should now use a diffing program (like meld on linux) to figure out what commands were added. 



 

