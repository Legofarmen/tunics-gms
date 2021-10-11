// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateThrow(){
	PlayerTileCollideLite();
	PlayerAnimSpr();
	sprite_index = sPlayerThrow;
	lift_id.dir = direction;
	lift_id.state = "thrown";
	if(animationEnd){
		lift_id = noone;
		state = "free";
	}
}