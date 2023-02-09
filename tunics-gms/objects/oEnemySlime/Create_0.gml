/// @description Inherit the parent event
event_inherited();
move = 0;
state = "idle";

spd = 0.5;
hurt = false;
spawnX = x;
spawnY = y;
patrolX = x+48;
patrolY = y;
knockX = 0;
knockY = 0;
knockDir = 0;
knock = false;
attack_cooldown = 0; //1 segundo (60*1)