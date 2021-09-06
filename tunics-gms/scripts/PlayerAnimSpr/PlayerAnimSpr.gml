// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerAnimSpr(){
var _cardinalDirection = round(direction/90);
var _totalFrames = sprite_get_number(sprite_index) / 4; //arriba, abajo, izquierda, derecha = 4.
image_index = localFrame + (_cardinalDirection * _totalFrames);
localFrame += sprite_get_speed(sprite_index)/room_speed; //60 FPS

if(localFrame>=_totalFrames){
		animationEnd = true;
		localFrame -= _totalFrames-0.1;
	}else{
		animationEnd = false;
	}
}