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

if(inputZtarget){
	//First, check for enemies in z target radius & direction
	var _list = ds_list_create();
	var _num = collision_circle_list(x,y,128,oEnemyMole,0,0,_list,1);
	var _index = 0;
	if(_num > 0){
	//Second, choose the closest one
		ztargeting = true;
		ztarget_id = _list[| _index];
	}else{
		ztargeting = false;
		ztarget_id = noone;
		ds_list_destroy(_list);
	}
	//Third, either switch target or toggle off if num = 1.
	if(ztargeting){
		if(_num > 1){
			if(_index>ds_list_size(_list)-1){
				_index = 0;
			}else{ _index++;}
		}else{
			ztargeting = false;
			ztarget_id = noone;
			ds_list_destroy(_list);
		}
	}
}