// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function menu_host_game(){
		instance_create_depth(x,y,0,oServer);
		room_goto(rm_test);
}