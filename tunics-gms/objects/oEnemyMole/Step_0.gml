/// @description Insert description here
// You can write your code in this editor
depth = -y;
direction = point_direction(x,y,goalX,goalY);
var target = collision_circle(x,y,56,oPlayer,false,true);

if(!place_meeting(x+knockX,y,oSolid)){
    x += knockX;}
if(!place_meeting(x,y+knockY,oSolid)){
    y += knockY;}

switch(move){
	case 0: sprite_index = sMoleIdle; break;
	case 1: sprite_index = sMoleWalk; break;
}
PlayerAnimSpr();
switch(state){
	case "idle":
		move = 0;
		goalX = x;
		goalY = y;
		if(target)state="alert";
		break;
	case "patrol":
		move = 1; spd = 0.5;
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
		spd = 0.8;
		move = 1;
		if(target){
		goalX = target.x;
		goalY = target.y;
		}else{
		state = "patrol";
		goalX = spawnX; goalY = spawnY;
		}
		break;
}

if(x != goalX || y != goalY){mp_potential_step_object(goalX,goalY,spd,oSolid);}

if(life <=0){
	instance_create_depth(x,y,depth,oDeath);
	instance_destroy();
}

if(place_meeting(x,y,oSensor)){
	knockDir = point_direction(target.x,target.y,x,y);
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