/// @description Initiate Server
enum network{
	player_connect,
	player_joined,
	player_disconnect,
	move
}
#macro PORT 35120
#macro MAX_CLIENTS 6
network_create_server(network_socket_tcp,PORT,MAX_CLIENTS);
server_buffer = buffer_create(1,buffer_grow,1);
socket = 0;
sockets = ds_list_create();
socket_to_instanceid = ds_map_create();

playerspawnx=100;
playerspawny=100;