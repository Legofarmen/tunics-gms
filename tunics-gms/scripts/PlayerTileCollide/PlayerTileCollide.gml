///Tile collisions for the player
function PlayerTileCollide(){
    //Horizontal
	if(tile_meeting(x+moveX,y,"collision")){
		repeat(abs(moveX)){
			if(!tile_meeting(x+sign(moveX),y,"collision")){
				x += sign(moveX)} else{break;}
			}
		moveX = 0;
	}
    x+= moveX;

    //Vertical
	if(tile_meeting(x,y+moveY,"collision")){
		repeat(abs(moveY)){
			if(!tile_meeting(x,y+sign(moveY),"collision")){
				y += sign(moveY)} else{break;}
			}
			moveY = 0;
		}
    y+= moveY;
}