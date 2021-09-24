/// @description Insert description here
// You can write your code in this editor
switch(state){
	case "idle":	image_speed = 0; break;
	case "carried": image_speed = 0; break;
	case "destroy" : 
		image_speed = 0; 
		image_index = image_number-1;
	break;
	case "thrown":	
		image_speed = 0;
		z--;
		z = clamp(z,0,18);
		var len = 2;
		moveX = lengthdir_x(len,dir);
		moveY = lengthdir_y(len,dir);
		if(z <= 0){
			moveX = 0;
			moveY = 0;
			image_speed = 1;
		}
		if(!tile_meeting(x+moveX,y,"collision")){
				x += moveX;
			}
		if(!tile_meeting(x,y+moveY,"collision")){
				y += moveY;
			}
	break;
}