/// @description Insert description here
// You can write your code in this editor
depth = -y-z;
switch(state){
	case "idle":	image_speed = 0; break;
	case "carried": image_speed = 0; break;
	case "destroy" : 
		image_speed = 0; 
		image_index = image_number-1;
		image_angle = 0;
		depth = 0;
	break;
	case "thrown":	
		image_speed = 0;
		z+=z_increment;
		z_increment-=0.2;
		z = clamp(z,0,28);
		image_angle -= 2;
		var len = 2.5;
		moveX = lengthdir_x(len,dir);
		moveY = lengthdir_y(len,dir);
		if(z <= 0){
			moveX = 0;
			moveY = 0;
			image_speed = 1;
			if(image_index >= 1){
				image_angle = 0;
				}
		}
		if(!tile_meeting(x+moveX,y,"collision")){
				x += moveX;
			}
		if(!tile_meeting(x,y+moveY,"collision")){
				y += moveY;
			}
	break;
}