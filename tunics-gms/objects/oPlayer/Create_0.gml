/// @description Vars and Input declaration
_up = vk_up;
_down =  vk_down;
_left = vk_left;
_right = vk_right;
_atk = ord("X");
_interact = ord("Z")
_ztarget = ord("D");

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
ztargeting = false;
ztarget_id = noone;