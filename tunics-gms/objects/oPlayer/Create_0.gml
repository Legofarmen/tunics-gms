/// @description Vars and Input declaration
inputMagnitude = 0;
inputDirection = 0;

spd = 1;
moveX = 0;
moveY = 0;
knockX = 0;
knockY = 0;
hurt = false;
flash = 0;

localFrame = 0;
animationEnd = false;

state = "free";
life = 3;
max_life = 3;
ztarget = noone;
socket = 0;

layer_id = layer_get_id("collision");
tilemap = layer_tilemap_get_id(layer_id);