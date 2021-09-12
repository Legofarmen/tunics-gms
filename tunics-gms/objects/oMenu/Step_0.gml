/// @description Insert description here
// You can write your code in this editor
if(keyboard_check_pressed(vk_down)){
	cur_index ++;
}

if(keyboard_check_pressed(vk_up)){
	cur_index --;
}

cur_index = clamp(cur_index,0,array_length(menu)-1);

if(keyboard_check_pressed(vk_enter)){
	switch(cur_index){
		case 0:
			//Host Game
			instance_create_depth(x,y,0,oServer);
			room_goto_next();
		break;
		
		case 1:
			//Join
			global.ip_address = get_string("Enter IP: ","");
			instance_create_depth(x,y,0,oClient);
			room_goto_next();
		break;
		
		case 2:
			//Singleplayer
			room_goto_next();
			break;
		case 3:
			//Exit
			game_end();
		break;
	}
}