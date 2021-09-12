/// @description Write Coordinates Buffer
if(instance_exists(oClient)){
buffer_seek(oClient.buffer,buffer_seek_start,0);
buffer_write(oClient.buffer,buffer_u8,network.move); //Send Packet ID
buffer_write(oClient.buffer,buffer_u16,x); //Send X
buffer_write(oClient.buffer,buffer_u16,y); //Send Y
buffer_write(oClient.buffer,buffer_u16,sprite_index); //Send Sprite
buffer_write(oClient.buffer,buffer_u8,image_index); //Send Image
network_send_packet(oClient.client,oClient.buffer,buffer_tell(oClient.buffer));}