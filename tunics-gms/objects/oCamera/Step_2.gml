/// @description Insert description here
// You can write your code in this editor
#macro VIEW view_camera[0]
camera_set_view_size(VIEW,view_width,view_height);

if(instance_exists(oPlayer)){
	var _x = clamp(oPlayer.x-(view_width/2),0,room_width-view_width);
	var _y = clamp(oPlayer.y-(view_height/2),0,room_height-view_height);
	
	
	var cur_x = camera_get_view_x(VIEW);
	var cur_y = camera_get_view_y(VIEW);
	var spd = 0.1;
	camera_set_view_pos(VIEW,
						lerp(_x,cur_x,spd),
						lerp(_y,cur_y,spd)
						);
}