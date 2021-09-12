/// @description Manage connection
type_event = async_load[? "type"];

switch(type_event){
	case network_type_data:
		client_buffer = async_load[? "buffer"];
		buffer_seek(client_buffer,buffer_seek_start,0);
		receive_packet_client(client_buffer);
	break;
}