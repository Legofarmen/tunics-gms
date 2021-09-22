// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function lang_english(){
	var grid;
	
	grid = menu_pages[ds_menu_main];
	grid[# 0, 0] = "START GAME";
	grid[# 0, 1] = "OPTIONS";
	grid[# 0, 2] = "QUIT";
	
	grid = menu_pages[ds_menu_gametype];
	grid[# 0, 0] = "SINGLE PLAYER";
	grid[# 0, 1] = "MULTIPLAYER";
	grid[# 0, 2] = "DIFFICULTY";
	grid[# 0, 3] = "< BACK";
	
	grid = menu_pages[ds_menu_multiplayer];
	grid[# 0, 0] = "HOST GAME";
	grid[# 0, 1] = "JOIN GAME";
	grid[# 0, 2] = "< BACK";
	
	grid = menu_pages[ds_menu_options];
	grid[# 0, 0] = "AUDIO";
	grid[# 0, 1] = "GRAPHICS";
	grid[# 0, 2] = "LANGUAGE";
	grid[# 0, 3] = "CONTROLS";
	grid[# 0, 4] = "< BACK";
	
	grid = menu_pages[ds_menu_audio];
	grid[# 0, 0] = "GENERAL";
	grid[# 0, 1] = "SOUND FX";
	grid[# 0, 2] = "MUSIC";
	grid[# 0, 3] = "< BACK";
	
	grid = menu_pages[ds_menu_graphics];
	grid[# 0, 0] = "RESOLUTION";
	grid[# 0, 1] = "FULLSCREEN";
	grid[# 0, 2] = "< BACK";
	
	grid = menu_pages[ds_menu_language];
	grid[# 0, 0] = "LANGUAGE";
	grid[# 0, 1] = "< BACK";
	
	grid = menu_pages[ds_menu_controls];
	grid[# 0, 0] = "UP";
	grid[# 0, 1] = "DOWN";
	grid[# 0, 2] = "LEFT";
	grid[# 0, 3] = "RIGHT";
	grid[# 0, 4] = "INTERACT (A)";
	grid[# 0, 5] = "ATTACK (B)";
	grid[# 0, 6] = "TARGET (Z)";
	grid[# 0, 7] = "< BACK";
}