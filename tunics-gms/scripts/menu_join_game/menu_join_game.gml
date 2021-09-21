// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function menu_join_game(){
			global.ip_address = get_string("Enter IP: ","");
			instance_create_depth(x,y,0,oClient);
			room_goto(rm_test);
}