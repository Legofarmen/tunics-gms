/// @description Hearts
if(interact_text != "") {
	draw_text_shadow(room_x_to_gui(x)+16, room_y_to_gui(y)-8, interact_text);
   }
var _vida = life
var _vidaFrac = frac(_vida); //Gettea el decimal de la _vida
_vida -= _vidaFrac;

var vida_x = 1;
var vida_y = 1;

var SEPARATION = 12;
var buffer = 12;

for(var i = 1; i <= max_life; i++){
    var _HeartPiece = (i > _vida);
    if( i == _vida+1){
        /*Chequea primero en booleano si VidaFrac es mayor a 0, 
        0.25 o 0.5 y lo suma a _HeartPiece, que es nuestro
        image_index artificial.
        */
        _HeartPiece += (_vidaFrac > 0); 
        _HeartPiece += (_vidaFrac > 0.25);
        _HeartPiece += (_vidaFrac > 0.5);
    }
    draw_sprite_ext(sHearts,_HeartPiece, vida_x + (i*SEPARATION), vida_y + buffer, 1,1,0,c_white,1);
}