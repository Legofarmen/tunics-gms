// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateFree(){
	
	moveX = lengthdir_x(inputMagnitude * spd, inputDirection)+knockX;
	moveY = lengthdir_y(inputMagnitude * spd, inputDirection)+knockY;

	PlayerTileCollide();

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
	
	//Lift jar/grass/etc
	var obj = noone;
	var aim_x = lengthdir_x(16,direction);
	var aim_y = lengthdir_y(16,direction);
	if(moveX=0 && moveY=0){
		obj = collision_line(x,y,x+aim_x,y+aim_y,oJar,1,1);
	}
	
	if(obj != noone){
		if(obj.state=="idle" && inputPressInteract){
			lift_id = obj;
			state = "lift";
		}
	}
}