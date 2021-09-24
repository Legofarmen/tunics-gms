/// @description menu array
global.key_enter	= vk_enter;
global.key_left		= vk_left;
global.key_right	= vk_right;
global.key_up		= vk_up;
global.key_down		= vk_down;
global.key_attack   = ord("X");
global.key_interact = ord("Z");
global.key_target = ord("D");

#region enums
enum menu_page{
	mainmenu,
	gametype,
	multiplayer,
	options,
	audio,
	graphics,
	language,
	controls
}

enum menu_type{
	script_run,
	page_transfer,
	slider,
	shift,
	toggle,
	input
}
#endregion
#region Menu Page creations
//MAIN MENU
ds_menu_main = create_menu_page(
	["START GAME",	menu_type.page_transfer,	menu_page.gametype],
	["OPTIONS",		menu_type.page_transfer,	menu_page.options],
	["QUIT",		menu_type.script_run,		menu_quit] //Quit just ends the application altogether
);

ds_menu_gametype = create_menu_page(
	["SINGLE PLAYER",	menu_type.script_run,		menu_singleplayer],
	["MULTIPLAYER",		menu_type.page_transfer,	menu_page.multiplayer],
	["DIFFICULTY",		menu_type.shift,			menu_change_difficulty, 0, ["SIMPLE","OKAY","GAMER","HARDCORE","IMPOSSIBLE"]],
	["< BACK",			menu_type.page_transfer,	menu_page.mainmenu] 
);

ds_menu_multiplayer = create_menu_page(
	["HOST GAME",		menu_type.script_run,		menu_host_game],
	["JOIN GAME",		menu_type.script_run,		menu_join_game],
	["< BACK",			menu_type.page_transfer,	menu_page.gametype]
);

ds_menu_options = create_menu_page(
	["AUDIO",		menu_type.page_transfer,	menu_page.audio],
	["GRAPHICS",	menu_type.page_transfer,	menu_page.graphics],
	["LANGUAGE",	menu_type.page_transfer,	menu_page.language],
	["CONTROLS",	menu_type.page_transfer,	menu_page.controls],
	["< BACK",		menu_type.page_transfer,	menu_page.mainmenu]
);

ds_menu_audio = create_menu_page(
	["GENERAL",		menu_type.slider,	menu_change_volume, 1, [0,1]],
	["SOUND FX",	menu_type.slider,	menu_change_volume, 1, [0,1]],
	["MUSIC",		menu_type.slider,	menu_change_volume, 1, [0,1]],
	["< BACK",		menu_type.page_transfer,	menu_page.options]
);

ds_menu_graphics = create_menu_page(
	["RESOLUTION",	menu_type.shift,	menu_change_resolution, oCamera.window_scale-1, ["320x180","640x360","960x540","1280x720","1600x900","1920x1080"]],
	["FULLSCREEN",	menu_type.toggle,	menu_change_fullscreen, 1, ["FULLSCREEN","WINDOW"]],
	["< BACK",		menu_type.page_transfer,	menu_page.options]
);

ds_menu_language = create_menu_page(
	["LANGUAGE",	menu_type.shift,			menu_change_language, 0, ["ENGLISH","ESPANOL","SVENSKA"]],
	["< BACK",		menu_type.page_transfer,	menu_page.options]
);

ds_menu_controls = create_menu_page(
	["UP",		menu_type.input, "key_up", vk_up],
	["DOWN",	menu_type.input, "key_down", vk_down],
	["LEFT",	menu_type.input, "key_left", vk_left],
	["RIGHT",	menu_type.input, "key_right", vk_right],
	["INTERACT (A)",menu_type.input, "key_interact", ord("Z")],
	["ATTACK (B)",menu_type.input, "key_attack", ord("X")],
	["TARGET (Z)",menu_type.input, "key_target", ord("D")],
	["< BACK",	menu_type.page_transfer,	menu_page.options]
);
#endregion

page = 0;
menu_pages = [ds_menu_main,ds_menu_gametype,ds_menu_multiplayer,ds_menu_options,ds_menu_audio,
			  ds_menu_graphics,ds_menu_language,ds_menu_controls];
			  
var i = 0, array_len = array_length(menu_pages);
repeat(array_len){
	menu_option[i] = 0;
	i++;
}

inputting = false;

audio_group_load(audiogroup_soundfx);
audio_group_load(audiogroup_music);

//Networking, do not delete
global.ip_address = "127.0.0.1";