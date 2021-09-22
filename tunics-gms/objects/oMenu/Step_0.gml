/// @description get inputs
input_up    = keyboard_check_pressed(global.key_up);
input_down  = keyboard_check_pressed(global.key_down);
input_left  = keyboard_check_pressed(global.key_left);
input_right = keyboard_check_pressed(global.key_right);
input_enter = keyboard_check_pressed(global.key_enter);

var grid = menu_pages[page];
var grid_height = ds_grid_height(grid);

if(inputting){
		switch(grid[# 1, menu_option[page]]){
		case menu_type.input:			
				var kk = keyboard_lastkey;
				if(kk != vk_enter && kk!=vk_escape){
					if(kk != grid[# 3, menu_option[page]]){	
					grid[# 3, menu_option[page]] = kk;
					variable_global_set(grid[# 2, menu_option[page]],kk);
					}
				}
			break;
		case menu_type.shift:
			var hinput = input_right - input_left;
			if(hinput != 0){
				grid[# 3, menu_option[page]] += hinput;
				grid[# 3, menu_option[page]] = clamp(grid[# 3, menu_option[page]], 0, array_length(grid[# 4, menu_option[page]])-1);
			}
			break;
		case menu_type.slider:			
			var hinput = keyboard_check(global.key_right) - keyboard_check(global.key_left);
			if(hinput != 0){
				grid[# 3, menu_option[page]] += hinput*0.01;
				grid[# 3, menu_option[page]] = clamp(grid[# 3, menu_option[page]], 0, 1);
				script_execute(grid[# 2, menu_option[page]],grid[# 3, menu_option[page]]);
			}
			break;
		case menu_type.toggle:			
			var hinput = input_right - input_left;
			if(hinput != 0){
				grid[# 3, menu_option[page]] += hinput;
				grid[# 3, menu_option[page]] = clamp(grid[# 3, menu_option[page]], 0, 1);
			}
			break;
	}
}else{
	var ochange = input_down - input_up;
	if(ochange!=0){
		menu_option[page] += ochange;
		if(menu_option[page] > grid_height-1)	{menu_option[page]=0;}
		if(menu_option[page] < 0)				{menu_option[page]=grid_height-1;}
	}
}

if(input_enter){
	switch(grid[# 1, menu_option[page]]){
		case menu_type.script_run:		script_execute(grid[# 2, menu_option[page]]); break;
		case menu_type.page_transfer:	page = grid[# 2, menu_option[page]]; break;
		case menu_type.input: inputting = !inputting; break;
		case menu_type.shift: //No breaks in here, so we can apply the inputting toggle to all of these.
		case menu_type.toggle: if(inputting){ script_execute(grid[# 2, menu_option[page]],grid[# 3, menu_option[page]]);}
		case menu_type.slider:
			inputting = !inputting;
			break;
	}
}