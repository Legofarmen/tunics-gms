/// @description declare var
enum network{
	move
}
var localhost = "127.0.0.1";
client = network_create_socket(network_socket_tcp);
buffer = buffer_create(1, buffer_grow, 1);
network_connect(client,localhost,PORT);