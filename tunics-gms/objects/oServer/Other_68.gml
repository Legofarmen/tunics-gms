/// @description Insert description here
// You can write your code in this editor
type_event = async_load[? "type"];

switch(type_event){
	case network_type_connect:
		socket = async_load[? "socket"]; //Get the socket of the connected client
		ds_list_add(sockets,socket); //Add it to the list to add data later
		var _player = instance_create_layer(playerspawnx,playerspawny,"Instances",oOnlinePlayer);
		ds_map_add(socket_to_instanceid,socket,_player); //Associate socket and player obj
		
		buffer_seek(server_buffer,buffer_seek_start,0);
		buffer_write(server_buffer,buffer_u8,network.player_connect);
		buffer_write(server_buffer,buffer_u8,socket);
		buffer_write(server_buffer,buffer_u16,_player.x);
		buffer_write(server_buffer,buffer_u16,_player.y);
		network_send_packet(socket,server_buffer,buffer_tell(server_buffer));
		
		
	break;
	
	case network_type_disconnect:
		socket = async_load[? "socket"]; //Get the socket of the disconnected client
		ds_list_delete(sockets,ds_list_find_index(sockets,socket)); //Delete disconnected socket from list
		with(ds_map_find_value(socket_to_instanceid,socket)){
			instance_destroy();
		}
	break;

	case network_type_data:
		var buffer = async_load[? "buffer"];
		var socket = async_load[? "id"];
		buffer_seek(buffer,buffer_seek_start,0);
		receive_packet(buffer,socket);
	break;
}