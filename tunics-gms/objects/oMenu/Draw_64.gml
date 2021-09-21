/// @description draw menu
if(!global.pause && room!=rm_menu) exit;

var width = oCamera.view_width;
var height = oCamera.view_height;
var grid = menu_pages[page];
var grid_height = ds_grid_height(grid);
var b = c_black;
var y_buffer=16,x_buffer=16;
var start_y = (height/2) - ((((grid_height-1)/2)*y_buffer)) 
var start_x = width/2;
//Draw Background
var bg = make_colour_hsv(181, 124, 108);
draw_rectangle_color(0,0,width,height,bg,bg,bg,bg,0);

//Draw Elements on the left
draw_set_valign(fa_middle);
draw_set_halign(fa_right);

var ltx = start_x - x_buffer;
var lty = 0;

var yy = 0;
repeat(grid_height){
	var c = c_white;
	lty = start_y + (yy*y_buffer);
	if(yy == menu_option[page]){
		c = c_orange;
	}
	draw_text_color(ltx,lty+1, grid[# 0, yy],b,b,b,b,1);
	draw_text_color(ltx,lty, grid[# 0, yy],c,c,c,c,1);
	yy++;
}
draw_line(start_x,start_y,start_x,lty);

//Draw Elements on the right
draw_set_halign(fa_left);
draw_set_valign(fa_top);

var rtx = start_x+x_buffer, rty;
yy = 0;
repeat(grid_height){
	rty = start_y+(yy*y_buffer)-6;
	switch(grid[# 1, yy]){
		case menu_type.shift:
			var current_val = grid[# 3, yy];
			var current_options = grid[# 4, yy];
			var lshift = "<< ";
			var rshift = " >>";
			
			if(current_val==0){lshift = "";}
			if(current_val==array_length(current_options)-1){rshift = "";}
			
			c = c_white;
			if(inputting == true && yy == menu_option[page]){
				c = c_yellow;
			}
			draw_text_color(rtx,rty+1,lshift+current_options[current_val]+rshift,b,b,b,b,1);
			draw_text_color(rtx,rty,lshift+current_options[current_val]+rshift,c,c,c,c,1);
		break;
		
		case menu_type.slider:
			c = c_white;
			var len = 64;
			var current_val = grid[# 3, yy];
			var current_array = grid[# 4,yy];
			var circle_pos = ((current_val - current_array[0]) / (current_array[1] - current_array[0]));
			draw_line_width_color(rtx, rty+7, rtx+len, rty+7,2,c_black,c_black);
			draw_line_width(rtx, rty+6, rtx+len, rty+6,2);
			if(inputting == true && yy == menu_option[page]){
				c = c_yellow;
			}
			draw_circle_color(rtx+(circle_pos*len), rty+6,4,c,c,false);
			draw_text_color(rtx + len*1.2, rty+1, string(floor(circle_pos*100))+"%",b,b,b,b,1);
			draw_text_color(rtx + len*1.2, rty, string(floor(circle_pos*100))+"%",c,c,c,c,1);
		break;
		
		case menu_type.toggle:
			c = c_white;
			var current_val = grid[# 3,yy];
			var c1,c2;
			if(inputting == true && yy == menu_option[page]){
				c = c_yellow;
			}
			if(current_val == 0){c1=c;c2=c_ltgray;}
			else				{c1=c_ltgray;c2=c;}
			
			draw_text_color(rtx,rty+1,"ON",b,b,b,b,1);
			draw_text_color(rtx,rty,"ON",c1,c1,c1,c1,1);
			draw_text_color(rtx+16,rty+1,"OFF",b,b,b,b,1);
			draw_text_color(rtx+16,rty,"OFF",c2,c2,c2,c2,1);
		break;
		
		case menu_type.input:
			c = c_white;
			var current_val = grid[# 3,yy];
			var string_val;
			
			switch(current_val){
				case vk_up:		string_val = "UP ARROW";		break;
				case vk_left:	string_val = "LEFT ARROW";		break;
				case vk_down:	string_val = "DOWN ARROW";		break;
				case vk_right:	string_val = "RIGHT ARROW";		break;
				default:		string_val = chr(current_val);	break;
			}
			if(inputting == true && yy == menu_option[page]){
				c = c_yellow;
			}
			draw_text_color(rtx, rty+1, string_val,b,b,b,b,1);
			draw_text_color(rtx, rty, string_val,c,c,c,c,1);
		break;
	}
	yy++;
}