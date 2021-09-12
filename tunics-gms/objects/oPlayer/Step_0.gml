/// @description input and states
// You can write your code in this editor
depth = -y;
inputU = keyboard_check(_up) || keyboard_check(vk_up);
inputL = keyboard_check(_left) || keyboard_check(vk_left);
inputR = keyboard_check(_right) || keyboard_check(vk_right);
inputD = keyboard_check(_down) || keyboard_check(vk_down);
inputAtk = keyboard_check_pressed(_atk) || mouse_check_button_pressed(mb_left);
inputZtarget = keyboard_check_pressed(_ztarget);
inputHoldInteract = keyboard_check(_interact);

inputMagnitude = (inputD - inputU != 0) || (inputR - inputL != 0);
inputDirection = point_direction(0,0,inputR-inputL,inputD-inputU);

//State Machine
switch(state){
	case "free": PlayerStateFree(); break;
	case "atk": PlayerStateAtk(); break;
}