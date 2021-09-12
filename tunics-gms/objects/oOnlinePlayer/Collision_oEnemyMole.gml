/// @description knockback and hurt
//Ahora es solo para el topo por falta de más enemigos, luego agregaré más
if(hurt = false){
	audio_play_sound(sndHit,0,0);
	audio_play_sound(sndPlayerHurt,0,0);
	flash = 0.8;
	alarm[0] = 30;
	alarm[1] = 10;
	var dir = point_direction(other.x,other.y,x,y);
	knockX = lengthdir_x(3,dir);
	knockY = lengthdir_y(3,dir);
	hurt = true;
	life-=0.25; //TEST VALUE
}