// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function receive_packet(buffer,socket){
	msgid = buffer_read(argument0,buffer_u8);
	switch(msgid){
		case network.move:
			var movex = buffer_read(argument0,buffer_u16);
			var movey = buffer_read(argument0,buffer_u16);
			
			var _player = ds_map_find_value(socket_to_instanceid,argument1);
			_player.x = movex;
			_player.y = movey;
			
			buffer_seek(server_buffer,buffer_seek_start,0); //PLACE WRITER AT BUFFER START
			buffer_write(server_buffer,buffer_u8,network.move); //SEND PACKET TYPE
			buffer_write(server_buffer,buffer_u16,movex); //SEND X BACK TO CLIENT
			buffer_write(server_buffer,buffer_u16,movey); //SEND Y BACK TO CLIENT
			network_send_packet(argument1,server_buffer,buffer_tell(server_buffer)); //SEND PACKET WITH BUFFER
		break;
	}
}