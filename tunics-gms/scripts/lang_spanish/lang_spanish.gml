// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function lang_spanish(){
	var grid;
	
	grid = menu_pages[ds_menu_main];
	grid[# 0, 0] = "COMENZAR";
	grid[# 0, 1] = "OPCIONES";
	grid[# 0, 2] = "SALIR";
	
	grid = menu_pages[ds_menu_gametype];
	grid[# 0, 0] = "UN JUGADOR";
	grid[# 0, 1] = "MULTIJUGADOR";
	grid[# 0, 2] = "DIFICULTAD";
	grid[# 0, 3] = "< ATRAS";
	
	grid = menu_pages[ds_menu_multiplayer];
	grid[# 0, 0] = "ABRIR UNA PARTIDA";
	grid[# 0, 1] = "UNIRSE A PARTIDA";
	grid[# 0, 2] = "< ATRAS";
	
	grid = menu_pages[ds_menu_options];
	grid[# 0, 0] = "AUDIO";
	grid[# 0, 1] = "GRAFICOS";
	grid[# 0, 2] = "LENGUAJE";
	grid[# 0, 3] = "CONTROLES";
	grid[# 0, 4] = "< ATRAS";
	
	grid = menu_pages[ds_menu_audio];
	grid[# 0, 0] = "GENERAL";
	grid[# 0, 1] = "EFECTOS DE SONIDO";
	grid[# 0, 2] = "MUSICA";
	grid[# 0, 3] = "< ATRAS";
	
	grid = menu_pages[ds_menu_graphics];
	grid[# 0, 0] = "RESOLUCION";
	grid[# 0, 1] = "PANTALLA COMPLETA";
	grid[# 0, 2] = "< ATRAS";
	
	grid = menu_pages[ds_menu_language];
	grid[# 0, 0] = "LENGUAJE";
	grid[# 0, 1] = "< ATRAS";
	
	grid = menu_pages[ds_menu_controls];
	grid[# 0, 0] = "ARRIBA";
	grid[# 0, 1] = "ABAJO";
	grid[# 0, 2] = "IZQUIERDA";
	grid[# 0, 3] = "DERECHA";
	grid[# 0, 4] = "INTERACTUAR (A)";
	grid[# 0, 5] = "ATACAR (B)";
	grid[# 0, 6] = "APUNTAR (Z)";
	grid[# 0, 7] = "< BACK";
}