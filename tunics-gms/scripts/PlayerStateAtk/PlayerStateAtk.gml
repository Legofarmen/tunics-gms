// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateAtk(){
sprite_index = sPlayerAtk;
//Update Image
PlayerAnimSpr();
if(animationEnd){
	state = "free";
	with(oSensor) instance_destroy();
	}

PlayerTileCollideLite();
}