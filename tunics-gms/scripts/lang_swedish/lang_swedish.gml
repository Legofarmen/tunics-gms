// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function lang_swedish(){
	var grid;
	
	grid = menu_pages[ds_menu_main];
	grid[# 0, 0] = "STARTA SPELET";
	grid[# 0, 1] = "ALTERNATIV";
	grid[# 0, 2] = "AVSLUTA";
	
	grid = menu_pages[ds_menu_gametype];
	grid[# 0, 0] = "ENSPELARLAGE";
	grid[# 0, 1] = "FLERSPELARLAGE";
	grid[# 0, 2] = "SVARIGHET";
	grid[# 0, 3] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_multiplayer];
	grid[# 0, 0] = "VARTSPEL";
	grid[# 0, 1] = "GA MED I SPEL";
	grid[# 0, 2] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_options];
	grid[# 0, 0] = "AUDIO";
	grid[# 0, 1] = "GRAFIK";
	grid[# 0, 2] = "SPRAK";
	grid[# 0, 3] = "KONTROLLER";
	grid[# 0, 4] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_audio];
	grid[# 0, 0] = "ALLMAN";
	grid[# 0, 1] = "LJUDEFFEKTER";
	grid[# 0, 2] = "MUSIK";
	grid[# 0, 3] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_graphics];
	grid[# 0, 0] = "UPPLOSNING";
	grid[# 0, 1] = "FULLSKARM";
	grid[# 0, 2] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_language];
	grid[# 0, 0] = "SPRAK";
	grid[# 0, 1] = "< AVBRYT";
	
	grid = menu_pages[ds_menu_controls];
	grid[# 0, 0] = "UPP";
	grid[# 0, 1] = "NER";
	grid[# 0, 2] = "VANSTER";
	grid[# 0, 3] = "HOGER";
	grid[# 0, 4] = "INTERAGERA (A)";
	grid[# 0, 5] = "ANFALL (B)";
	grid[# 0, 6] = "SIKTA (Z)";
	grid[# 0, 7] = "< AVBRYT";
}