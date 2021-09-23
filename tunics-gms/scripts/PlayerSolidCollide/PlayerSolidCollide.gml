///Solid collisions for the player
function PlayerSolidCollide(){
    //Horizontal
	if(place_meeting(x+moveX,y,oSolid)){
		repeat(abs(moveX)){
			if(!place_meeting(x+sign(moveX),y,oSolid)){
				x += sign(moveX)} else{break;}
			}
			moveX = 0;
		}
	x+= moveX;
	
	//Vertical
	if(place_meeting(x,y+moveY,oSolid)){
		repeat(abs(moveY)){
			if(!place_meeting(x,y+sign(moveY),oSolid)){
				y += sign(moveY)} else{break;}
			}
			moveY = 0;
		}
	y+= moveY;
}