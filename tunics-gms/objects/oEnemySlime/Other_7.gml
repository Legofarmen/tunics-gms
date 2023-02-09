/// @description Insert description here
// You can write your code in this editor
if(sprite_index == sSlimeJump){
	image_speed = 0;
	image_index = image_number-1;
}

if(sprite_index == sSlimeFall){
	image_speed = 0;
	image_index = image_number-1;
}

if(state == "attack3"){
	attack_cooldown = 60*1;
	spd = 0.4;
	state = "alert";
}