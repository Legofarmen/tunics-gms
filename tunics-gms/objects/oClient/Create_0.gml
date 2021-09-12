/// @description declare var
var localhost = "127.0.0.1";
client = network_create_socket(network_socket_tcp);
buffer = buffer_create(1, buffer_grow, 1);
socket_to_instanceid = ds_map_create();
network_connect(client,global.ip_address,PORT);