/// @description Insert description here
// You can write your code in this editor
event_inherited();
move = 0;
state = "idle";
localFrame = 0;
animationEnd = false;

spawnX = x;
spawnY = y;
patrolX = x+48;
patrolY = y;
knockX = 0;
knockY = 0;
knockDir = 0;
knock = false;

hurt = false;
alarm[0] = 180;
spd = 0.5;

goalX = 0;
goalY = 0;