use rusttype::Scale;

pub const MARGINS: [i32; 18] = [
/* 1    2    3    4    5    6    7    8    9  */
   56,  87,  118, 150, 182, 214, 248, 281, 314,
/* 10   11   12   13   14   15   16   17   18  */
   346, 379, 412, 445, 478, 511, 544, 576, 607
];

pub const FONT_SCALE: Scale = Scale { x: 22., y: 22. };
