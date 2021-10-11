/// @description Death
if(life <=0){
	instance_create_depth(x-(sprite_get_width(sprite_index)/2),y,depth,oDeath);
	instance_destroy();
}