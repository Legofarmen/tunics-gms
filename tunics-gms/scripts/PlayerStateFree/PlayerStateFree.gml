// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateFree(){
	
moveX = lengthdir_x(inputMagnitude * spd, inputDirection)+knockX;
moveY = lengthdir_y(inputMagnitude * spd, inputDirection)+knockY;

#region Colisiones
if(place_meeting(x+moveX,y,oSolid)){
	repeat(abs(moveX)){
		if(!place_meeting(x+sign(moveX),y,oSolid)){
			x += sign(moveX)} else{break;}
		}
		moveX = 0;
	}
	
if(place_meeting(x,y+moveY,oSolid)){
	repeat(abs(moveY)){
		if(!place_meeting(x,y+sign(moveY),oSolid)){
			y += sign(moveY)} else{break;}
		}
		moveY = 0;
	}
	
x+= moveX;
y+= moveY;
#endregion

//Update Sprite
var _oldSprite = sprite_index;
if(inputMagnitude!=0){
	direction = inputDirection;
	if(inputHoldInteract){
	sprite_index = sPlayerRun;
	spd = 1.6;
	}else{
	sprite_index = sPlayerWalk;
	spd = 1;
	}
}else{
	sprite_index = sPlayerIdle;
	}
if(_oldSprite != sprite_index) localFrame = 0;

//Update Image
PlayerAnimSpr();

if(inputAtk){
	localFrame = 0;
	audio_play_sound(sndSwing,0,0);
	state = "atk";
	}
}