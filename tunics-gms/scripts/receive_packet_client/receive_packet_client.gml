// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function receive_packet_client(buffer){
	msgid = buffer_read(argument0,buffer_u8);
	switch(msgid){
		case network.move:
			var movex = buffer_read(argument0,buffer_u16);
			var movey = buffer_read(argument0,buffer_u16);
			
			oPlayer.x = movex;
			oPlayer.y = movey;
		break;
	}
}