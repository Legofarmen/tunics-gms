// Script assets have changed for v2.3.0 see
// SCRIPT WHEN SERVER RECEIVES PACKET
function receive_packet(buffer,socket){
	msgid = buffer_read(argument0,buffer_u8);
	switch(msgid){
		case network.move:
			var movex = buffer_read(argument0,buffer_u16);
			var movey = buffer_read(argument0,buffer_u16);
			var sprite = buffer_read(argument0,buffer_u16);
			var image = buffer_read(argument0,buffer_u8);
			
			var _player = ds_map_find_value(socket_to_instanceid,argument1);
			_player.x = movex;
			_player.y = movey;
			_player.sprite_index = sprite;
			_player.image_index = image;
			
			var i = 0;
			repeat(ds_list_size(sockets)){
				var _sock = ds_list_find_value(sockets,i);
				if(_sock != argument1){ //Send to all clients except original remittent
				buffer_seek(server_buffer,buffer_seek_start,0); //PLACE WRITER AT BUFFER START
				buffer_write(server_buffer,buffer_u8,network.move); //SEND PACKET TYPE
				buffer_write(server_buffer,buffer_u8,argument1); //GET SOCKET
				buffer_write(server_buffer,buffer_u16,movex); //SEND X BACK TO CLIENT
				buffer_write(server_buffer,buffer_u16,movey); //SEND Y BACK TO CLIENT
				buffer_write(server_buffer,buffer_u16,sprite); //SEND SPRITEINDEX BACK TO CLIENT
				buffer_write(server_buffer,buffer_u8,image); //SEND IMAGEINDEX BACK TO CLIENT
				network_send_packet(_sock,server_buffer,buffer_tell(server_buffer)); //SEND PACKET WITH BUFFER
				i++;
				}
			}
			break;
	}
}