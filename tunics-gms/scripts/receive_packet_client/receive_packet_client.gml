// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function receive_packet_client(buffer){
	msgid = buffer_read(argument0,buffer_u8);
	switch(msgid){
		case network.player_connect:
			var _socket = buffer_read(argument0,buffer_u8);
			var _x = buffer_read(argument0,buffer_u16);
			var _y = buffer_read(argument0,buffer_u16);
			var _player = instance_create_layer(_x,_y,"Instances",oPlayer);
			_player.socket = _socket;
			ds_map_add(socket_to_instanceid,_socket,_player);
		break;
		
		case network.player_joined:
			var _socket = buffer_read(argument0,buffer_u8);
			var _x = buffer_read(argument0,buffer_u16);
			var _y = buffer_read(argument0,buffer_u16);
			var _puppet = instance_create_layer(_x,_y,"Instances",oOnlinePlayer);
			_puppet.socket = _socket;	
			ds_map_add(socket_to_instanceid,_socket,_puppet);
		break;
		
		case network.player_disconnect:
			var _socket = buffer_read(argument0,buffer_u8);
			var _puppet = ds_map_find_value(socket_to_instanceid,_socket);
			with(_puppet){
				instance_destroy();
			}
			ds_map_delete(socket_to_instanceid,_socket);
		break;
		case network.move:
			var _sock = buffer_read(argument0,buffer_u8);
			var movex = buffer_read(argument0,buffer_u16);
			var movey = buffer_read(argument0,buffer_u16);
			var sprite= buffer_read(argument0,buffer_u16);
			var image = buffer_read(argument0,buffer_u8);
			_puppet = ds_map_find_value(socket_to_instanceid,_sock);
			_puppet.x = movex;
			_puppet.y = movey;
			_puppet.sprite_index = sprite;
			_puppet.image_index = image;
		break;
	}
}