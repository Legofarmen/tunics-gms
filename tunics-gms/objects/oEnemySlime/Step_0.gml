/// @description Inherit (oEnemyPar
// Inherit the parent event
var target = collision_circle(x,y,82,oPlayer,false,true);
if(attack_cooldown > 0) attack_cooldown--;

if(target){
var	jumpX = lengthdir_x(16,point_direction(x,y,target.x,target.y));
var	jumpY = lengthdir_y(16,point_direction(x,y,target.x,target.y));
if(target.x > x) image_xscale =  1;
if(target.x < x) image_xscale = -1;
}


if(!place_meeting(x+knockX,y,oSolid)){
    x += knockX;}
if(!place_meeting(x,y+knockY,oSolid)){
    y += knockY;}

switch(move){
	case 0: image_speed = 0.5; break;
	case 1: image_speed = 1; break;
}

switch(state){
	case "idle":
		move = 0;
		goalX = x;
		goalY = y;
		if(target)state="alert";
		break;
		
	case "patrol":
		move = 1; spd = 0.3;
		if(x == patrolX && y == patrolY){
			goalX = spawnX;
			goalY = spawnY;
		}
		if(x == spawnX && y == spawnY){
			goalX = patrolX;
			goalY = patrolY;
		}
		if(target)state="alert";
		break;
		
	case "alert":
		sprite_index = sSlimeIdle;
		move = 1;
		if(target){
		goalX = target.x;
		goalY = target.y;
		if(collision_circle(x,y,48,oPlayer,0,1) && attack_cooldown <= 0)
			{
				state = "attack1";
			}
		}else{
		state = "patrol";
		goalX = spawnX; goalY = spawnY;
		}
		break;
		
	case "attack1":
		image_speed = 1;
		state = "attack2";
		spd = 1.6;
		break;
	
	case "attack2":
		sprite_index = sSlimeJump;
		z++;
		z = clamp(z,0,16);
		if(z >=16){
			sprite_index = sSlimeFall; 
			image_index = 0; 
			state="attack3";
			}
		goalX = x+jumpX;
		goalY = y+jumpY;
		break;
		
	case "attack3":
		z--;
		z = clamp(z,0,16);
		break;
}

if(x != goalX || y != goalY){mp_potential_step_object(goalX,goalY,spd,oSolid);}

event_inherited();
var dmg_x = 0; var dmg_y = 0;
var _thrown = place_meeting(x,y,oJar);
var _atkd = place_meeting(x,y,oSensor);
var _contact = false;

if(_atkd && target){
	_contact = true;
	dmg_x = target.x;
	dmg_y = target.y;
}

if(_thrown && _thrown.state == "thrown"){
	_contact = true;
	dmg_x = _thrown.x;
	dmg_y = _thrown.y;
}

if(_contact){
	knockDir = point_direction(dmg_x,dmg_y,x,y);
	if(!hurt){
		audio_play_sound(sndHit,0,0);
		life--;
		hurt = true;
		knock = true;
		flash = 0.6;
		alarm[1] = 20;
		alarm[2] = 10;
	}
}

if(knock){
	knockX = lengthdir_x(3,knockDir);
	knockY = lengthdir_y(3,knockDir);
}else{
	knockX = 0;
	knockY = 0;
}