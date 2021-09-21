/// @description input and states
// You can write your code in this editor
depth = -y;
inputU = keyboard_check(global.key_up);
inputL = keyboard_check(global.key_left);
inputR = keyboard_check(global.key_right);
inputD = keyboard_check(global.key_down);
inputAtk = keyboard_check(global.key_attack);
inputZtarget = keyboard_check_pressed(_ztarget);
inputHoldInteract = keyboard_check(global.key_interact);

inputMagnitude = (inputD - inputU != 0) || (inputR - inputL != 0);
inputDirection = point_direction(0,0,inputR-inputL,inputD-inputU);

//State Machine
switch(state){
	case "free": PlayerStateFree(); break;
	case "atk": PlayerStateAtk(); break;
}