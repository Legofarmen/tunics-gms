// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function menu_change_language(){
	switch(argument0){
		case 0: //ENGLISH
			lang_english();
		break;
		case 1: //ESPAÑOL
			lang_spanish();
		break;
		case 2: //SVENSKA
			lang_swedish();
		break;
	}
}