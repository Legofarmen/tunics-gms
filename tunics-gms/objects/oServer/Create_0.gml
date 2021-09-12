/// @description Initiate Server
#macro PORT 35000
#macro MAX_CLIENTS 6
network_create_server(network_socket_tcp,PORT,MAX_CLIENTS);
server_buffer = buffer_create(1,buffer_grow,1);
clients = ds_map_create();
sockets = ds_list_create();