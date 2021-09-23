// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function PlayerTileCollideLite(){
if(!tile_meeting(x+knockX,y,"collision")){
		x += knockX;
		}
if(!tile_meeting(x,y+knockY,"collision")){
		y += knockY;
		}
}