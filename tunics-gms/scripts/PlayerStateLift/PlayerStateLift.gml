// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateLift(){
	var _x = x-(sprite_get_width(sprite_index)/2)+(sprite_get_width(lift_id.sprite_index)/2);
	var _y = y-(sprite_get_height(sprite_index)/2)
	var val = 1;
	sprite_index = sPlayerLift;
	PlayerTileCollideLite();
	PlayerAnimSpr();
	spd = 1;
	lift_id.x = approach(lift_id.x,_x,val);
	lift_id.y = approach(lift_id.y,_y,val);
	lift_id.z = approach(lift_id.z,18,val);
	if(animationEnd){
		state = "carry";
		lift_id.state = "carried";
	}
}