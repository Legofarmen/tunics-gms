/// @description Insert description here
// You can write your code in this editor
type_event = async_load[? "type"];

switch(type_event){
	case network_type_connect:
		var socket = async_load[? "socket"]; //Get the socket of the connected client
		ds_list_add(sockets,socket); //Add it to the list to add data later
	break;
	
	case network_type_disconnect:
		var socket = async_load[? "socket"]; //Get the socket of the disconnected client
		ds_list_delete(sockets,ds_list_find_index(sockets,socket)); //Delete disconnected socket from list
	break;

	case network_type_data:
		var buffer = async_load[? "buffer"];
		var socket = async_load[? "id"];
		buffer_seek(buffer,buffer_seek_start,0);
		receive_packet(buffer,socket);
	break;
}