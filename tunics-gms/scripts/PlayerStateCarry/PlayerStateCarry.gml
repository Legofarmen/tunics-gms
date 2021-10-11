// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerStateCarry(){
	var _x = x-(sprite_get_width(sprite_index)/2)+(sprite_get_width(lift_id.sprite_index))-2;
	var _y = y-(sprite_get_height(sprite_index)/4)
	
	moveX = lengthdir_x(inputMagnitude * spd, inputDirection)+knockX;
	moveY = lengthdir_y(inputMagnitude * spd, inputDirection)+knockY;

	PlayerTileCollide();
	if(instance_exists(oSolid)){
		PlayerSolidCollide();
	}
	
	//Update Sprite
	var _oldSprite = sprite_index;
	if(inputMagnitude!=0){
		direction = inputDirection;
		sprite_index = sPlayerCarry;
		image_speed = 1;
	}else{
		image_speed = 0;
		localFrame = 3;
	}
	if(_oldSprite != sprite_index) localFrame = 0;

	//Update Image
	PlayerAnimSpr();
	
	lift_id.x = _x;
	lift_id.y = _y;
	lift_id.z = 18;
	lift_id.depth = depth-1;

	if(inputAtk || inputPressInteract){
		localFrame = 0;
		lift_id.dir = inputDirection;
		state = "throw";
		}
}