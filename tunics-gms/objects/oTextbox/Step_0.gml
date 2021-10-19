/// @description aaa
var der = oCamera.view_width;
lerpProgress += (1 - lerpProgress)/50;
textProgress += 0.75;

x1 = lerp(x1,0,lerpProgress);
x2 = lerp(x2,der,lerpProgress);

if(keyboard_check_pressed(global.key_interact)){
	var _length = string_length(text[page]);
	var _pages = array_length(text);
	if(textProgress >= _length){
		if(page < _pages-1){
				page += 1;
				textProgress = 0;
		}else{
			instance_destroy();
		}
	}else{
		if(textProgress > 2){
			textProgress = _length;
		}
	}
}