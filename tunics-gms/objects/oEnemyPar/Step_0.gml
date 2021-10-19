/// @description Death
if(life <=0){
	instance_create_depth(x,y,depth,oDeath);
	instance_destroy();
}