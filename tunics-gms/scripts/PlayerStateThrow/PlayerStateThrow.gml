// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateThrow(){
	PlayerTileCollideLite();
	PlayerAnimSpr();
	sprite_index = sPlayerThrow;
	if(animationEnd){
		lift_id.dir = inputDirection;
		lift_id.state = "thrown";
		lift_id = noone;
		state = "free";
	}
}