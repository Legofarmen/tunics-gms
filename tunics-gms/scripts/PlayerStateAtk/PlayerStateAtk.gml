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

if(inputAtk){
	localFrame = 0;
	audio_play_sound(sndSwing,0,0);
	}
#region Colision Lite
if(!place_meeting(x+knockX,y,oSolid)){
	x+=knockX;
	}
if(!place_meeting(x,y+knockY,oSolid)){
	y+=knockY;
	}
#endregion
}