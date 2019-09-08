pub static FONT8X8: &[[u8; 8]; 256] = &[
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], /* 0 */
  [0x7e, 0x81, 0xa5, 0x81, 0xbd, 0x99, 0x81, 0x7e], /* 1 */
  [0x7e, 0xff, 0xdb, 0xff, 0xc3, 0xe7, 0xff, 0x7e], /* 2 */
  [0x36, 0x7f, 0x7f, 0x7f, 0x3e, 0x1c, 0x08, 0x00], /* 3 */
  [0x08, 0x1c, 0x3e, 0x7f, 0x3e, 0x1c, 0x08, 0x00], /* 4 */
  [0x1c, 0x3e, 0x1c, 0x7f, 0x7f, 0x6b, 0x08, 0x1c], /* 5 */
  [0x08, 0x08, 0x1c, 0x3e, 0x7f, 0x3e, 0x08, 0x1c], /* 6 */
  [0x00, 0x00, 0x18, 0x3c, 0x3c, 0x18, 0x00, 0x00], /* 7 */
  [0xff, 0xff, 0xe7, 0xc3, 0xc3, 0xe7, 0xff, 0xff], /* 8 */
  [0x00, 0x3c, 0x66, 0x42, 0x42, 0x66, 0x3c, 0x00], /* 9 */
  [0xff, 0xc3, 0x99, 0xbd, 0xbd, 0x99, 0xc3, 0xff], /* 10 */
  [0xf0, 0xc0, 0xa0, 0xbe, 0x21, 0x21, 0x21, 0x1e], /* 11 */
  [0x3c, 0x42, 0x42, 0x42, 0x3c, 0x18, 0x7e, 0x18], /* 12 */
  [0xfc, 0x84, 0xfc, 0x04, 0x04, 0x06, 0x07, 0x03], /* 13 */
  [0xfc, 0x84, 0xfc, 0x84, 0xc4, 0xe6, 0x67, 0x03], /* 14 */
  [0x18, 0xdb, 0x3c, 0xe7, 0xe7, 0x3c, 0xdb, 0x18], /* 15 */
  [0x01, 0x07, 0x1f, 0x7f, 0x1f, 0x07, 0x01, 0x00], /* 16 */
  [0x40, 0x70, 0x7c, 0x7f, 0x7c, 0x70, 0x40, 0x00], /* 17 */
  [0x18, 0x3c, 0x7e, 0x18, 0x18, 0x7e, 0x3c, 0x18], /* 18 */
  [0x24, 0x24, 0x24, 0x24, 0x24, 0x00, 0x24, 0x00], /* 19 */
  [0xfe, 0x49, 0x49, 0x4e, 0x48, 0x48, 0x48, 0x00], /* 20 */
  [0x7c, 0xc6, 0x1c, 0x22, 0x22, 0x1c, 0x33, 0x1e], /* 21 */
  [0x00, 0x00, 0x00, 0x00, 0x7e, 0x7e, 0x7e, 0x00], /* 22 */
  [0x18, 0x3c, 0x7e, 0x18, 0x7e, 0x3c, 0x18, 0xff], /* 23 */
  [0x08, 0x1c, 0x3e, 0x2a, 0x08, 0x08, 0x08, 0x00], /* 24 */
  [0x08, 0x08, 0x08, 0x2a, 0x3e, 0x1c, 0x08, 0x00], /* 25 */
  [0x00, 0x18, 0x30, 0x7f, 0x30, 0x18, 0x00, 0x00], /* 26 */
  [0x00, 0x0c, 0x06, 0x7f, 0x06, 0x0c, 0x00, 0x00], /* 27 */
  [0x00, 0x00, 0x02, 0x02, 0x02, 0x7e, 0x00, 0x00], /* 28 */
  [0x00, 0x24, 0x66, 0xff, 0x66, 0x24, 0x00, 0x00], /* 29 */
  [0x00, 0x08, 0x1c, 0x3e, 0x7f, 0x7f, 0x00, 0x00], /* 30 */
  [0x00, 0x7f, 0x7f, 0x3e, 0x1c, 0x08, 0x00, 0x00], /* 31 */
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], /* 32 */
  [0x08, 0x1c, 0x1c, 0x08, 0x08, 0x00, 0x08, 0x00], /* 33 */
  [0x24, 0x24, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00], /* 34 */
  [0x24, 0x24, 0x7e, 0x24, 0x7e, 0x24, 0x24, 0x00], /* 35 */
  [0x18, 0x7c, 0x02, 0x3c, 0x40, 0x3e, 0x18, 0x00], /* 36 */
  [0x00, 0x46, 0x26, 0x10, 0x08, 0x64, 0x62, 0x00], /* 37 */
  [0x0c, 0x12, 0x0c, 0x6a, 0x11, 0x11, 0x6e, 0x00], /* 38 */
  [0x08, 0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00], /* 39 */
  [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08, 0x00], /* 40 */
  [0x04, 0x08, 0x10, 0x10, 0x10, 0x08, 0x04, 0x00], /* 41 */
  [0x00, 0x22, 0x1c, 0x7f, 0x1c, 0x22, 0x00, 0x00], /* 42 */
  [0x00, 0x08, 0x08, 0x3e, 0x08, 0x08, 0x00, 0x00], /* 43 */
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x04], /* 44 */
  [0x00, 0x00, 0x00, 0x7e, 0x00, 0x00, 0x00, 0x00], /* 45 */
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x08, 0x00], /* 46 */
  [0x00, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x00], /* 47 */
  [0x3c, 0x42, 0x62, 0x52, 0x4a, 0x46, 0x3c, 0x00], /* 48 */
  [0x08, 0x0c, 0x0a, 0x08, 0x08, 0x08, 0x3e, 0x00], /* 49 */
  [0x3c, 0x42, 0x40, 0x30, 0x0c, 0x42, 0x7e, 0x00], /* 50 */
  [0x3c, 0x42, 0x40, 0x38, 0x40, 0x42, 0x3c, 0x00], /* 51 */
  [0x10, 0x18, 0x14, 0x12, 0x7f, 0x10, 0x38, 0x00], /* 52 */
  [0x7e, 0x02, 0x3e, 0x40, 0x40, 0x42, 0x3c, 0x00], /* 53 */
  [0x38, 0x04, 0x02, 0x3e, 0x42, 0x42, 0x3c, 0x00], /* 54 */
  [0x7e, 0x42, 0x20, 0x10, 0x08, 0x08, 0x08, 0x00], /* 55 */
  [0x3c, 0x42, 0x42, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 56 */
  [0x3c, 0x42, 0x42, 0x7c, 0x40, 0x20, 0x1c, 0x00], /* 57 */
  [0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0x08, 0x00], /* 58 */
  [0x00, 0x08, 0x08, 0x00, 0x00, 0x08, 0x08, 0x04], /* 59 */
  [0x10, 0x08, 0x04, 0x02, 0x04, 0x08, 0x10, 0x00], /* 60 */
  [0x00, 0x00, 0x7e, 0x00, 0x00, 0x7e, 0x00, 0x00], /* 61 */
  [0x08, 0x10, 0x20, 0x40, 0x20, 0x10, 0x08, 0x00], /* 62 */
  [0x3c, 0x42, 0x40, 0x20, 0x10, 0x00, 0x10, 0x00], /* 63 */
  [0x3c, 0x42, 0x7a, 0x4a, 0x7a, 0x02, 0x3c, 0x00], /* 64 */
  [0x18, 0x24, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x00], /* 65 */
  [0x3e, 0x44, 0x44, 0x3c, 0x44, 0x44, 0x3e, 0x00], /* 66 */
  [0x38, 0x44, 0x02, 0x02, 0x02, 0x44, 0x38, 0x00], /* 67 */
  [0x1e, 0x24, 0x44, 0x44, 0x44, 0x24, 0x1e, 0x00], /* 68 */
  [0x7e, 0x44, 0x14, 0x1c, 0x14, 0x44, 0x7e, 0x00], /* 69 */
  [0x7e, 0x44, 0x14, 0x1c, 0x14, 0x04, 0x0e, 0x00], /* 70 */
  [0x38, 0x44, 0x02, 0x02, 0x72, 0x44, 0x78, 0x00], /* 71 */
  [0x42, 0x42, 0x42, 0x7e, 0x42, 0x42, 0x42, 0x00], /* 72 */
  [0x1c, 0x08, 0x08, 0x08, 0x08, 0x08, 0x1c, 0x00], /* 73 */
  [0x70, 0x20, 0x20, 0x20, 0x22, 0x22, 0x1c, 0x00], /* 74 */
  [0x46, 0x24, 0x14, 0x0c, 0x14, 0x24, 0xc6, 0x00], /* 75 */
  [0x0e, 0x04, 0x04, 0x04, 0x04, 0x44, 0x7e, 0x00], /* 76 */
  [0xc6, 0xaa, 0x92, 0x82, 0x82, 0x82, 0x82, 0x00], /* 77 */
  [0x46, 0x4a, 0x52, 0x62, 0x42, 0x42, 0x42, 0x00], /* 78 */
  [0x18, 0x24, 0x42, 0x42, 0x42, 0x24, 0x18, 0x00], /* 79 */
  [0x3e, 0x44, 0x44, 0x3c, 0x04, 0x04, 0x0e, 0x00], /* 80 */
  [0x3c, 0x42, 0x42, 0x42, 0x52, 0x3c, 0xc0, 0x00], /* 81 */
  [0x3e, 0x44, 0x44, 0x3c, 0x14, 0x24, 0x4e, 0x00], /* 82 */
  [0x3c, 0x42, 0x02, 0x3c, 0x40, 0x42, 0x3c, 0x00], /* 83 */
  [0xfe, 0x92, 0x10, 0x10, 0x10, 0x10, 0x38, 0x00], /* 84 */
  [0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 85 */
  [0x82, 0x82, 0x82, 0x82, 0x44, 0x28, 0x10, 0x00], /* 86 */
  [0x82, 0x82, 0x82, 0x92, 0x92, 0x92, 0x6c, 0x00], /* 87 */
  [0x82, 0x44, 0x28, 0x10, 0x28, 0x44, 0x82, 0x00], /* 88 */
  [0x82, 0x44, 0x28, 0x10, 0x10, 0x10, 0x38, 0x00], /* 89 */
  [0xfe, 0x42, 0x20, 0x10, 0x08, 0x84, 0xfe, 0x00], /* 90 */
  [0x1e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x1e, 0x00], /* 91 */
  [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x00], /* 92 */
  [0x1e, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1e, 0x00], /* 93 */
  [0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00, 0x00], /* 94 */
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff], /* 95 */
  [0x08, 0x08, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00], /* 96 */
  [0x00, 0x00, 0x3c, 0x40, 0x7c, 0x42, 0xfc, 0x00], /* 97 */
  [0x06, 0x04, 0x04, 0x74, 0x8c, 0x8c, 0x74, 0x00], /* 98 */
  [0x00, 0x00, 0x3c, 0x42, 0x02, 0x42, 0x3c, 0x00], /* 99 */
  [0x60, 0x40, 0x40, 0x5c, 0x62, 0x62, 0xdc, 0x00], /* 100 */
  [0x00, 0x00, 0x3c, 0x42, 0x7e, 0x02, 0x3c, 0x00], /* 101 */
  [0x30, 0x48, 0x08, 0x1c, 0x08, 0x08, 0x1c, 0x00], /* 102 */
  [0x00, 0x00, 0xbc, 0x42, 0x42, 0x7c, 0x40, 0x3e], /* 103 */
  [0x06, 0x04, 0x34, 0x4c, 0x44, 0x44, 0x46, 0x00], /* 104 */
  [0x08, 0x00, 0x0c, 0x08, 0x08, 0x08, 0x1c, 0x00], /* 105 */
  [0x40, 0x00, 0x60, 0x40, 0x40, 0x42, 0x42, 0x3c], /* 106 */
  [0x06, 0x04, 0x24, 0x14, 0x0c, 0x14, 0x64, 0x00], /* 107 */
  [0x0c, 0x08, 0x08, 0x08, 0x08, 0x08, 0x1c, 0x00], /* 108 */
  [0x00, 0x00, 0x6e, 0x92, 0x92, 0x92, 0x92, 0x00], /* 109 */
  [0x00, 0x00, 0x3a, 0x46, 0x42, 0x42, 0x42, 0x00], /* 110 */
  [0x00, 0x00, 0x3c, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 111 */
  [0x00, 0x00, 0x36, 0x4c, 0x4c, 0x34, 0x04, 0x0e], /* 112 */
  [0x00, 0x00, 0x6c, 0x32, 0x32, 0x2c, 0x20, 0x70], /* 113 */
  [0x00, 0x00, 0x36, 0x4c, 0x44, 0x04, 0x0e, 0x00], /* 114 */
  [0x00, 0x00, 0x7c, 0x02, 0x3c, 0x40, 0x3e, 0x00], /* 115 */
  [0x08, 0x08, 0x3e, 0x08, 0x08, 0x48, 0x30, 0x00], /* 116 */
  [0x00, 0x00, 0x42, 0x42, 0x42, 0x62, 0x5c, 0x00], /* 117 */
  [0x00, 0x00, 0x82, 0x82, 0x44, 0x28, 0x10, 0x00], /* 118 */
  [0x00, 0x00, 0x82, 0x92, 0x92, 0x92, 0x6c, 0x00], /* 119 */
  [0x00, 0x00, 0x22, 0x14, 0x08, 0x14, 0x22, 0x00], /* 120 */
  [0x00, 0x00, 0x42, 0x42, 0x42, 0x7c, 0x40, 0x3e], /* 121 */
  [0x00, 0x00, 0x3e, 0x10, 0x08, 0x04, 0x3e, 0x00], /* 122 */
  [0x30, 0x08, 0x08, 0x06, 0x08, 0x08, 0x30, 0x00], /* 123 */
  [0x08, 0x08, 0x08, 0x00, 0x08, 0x08, 0x08, 0x00], /* 124 */
  [0x0c, 0x10, 0x10, 0x60, 0x10, 0x10, 0x0c, 0x00], /* 125 */
  [0x4c, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], /* 126 */
  [0x00, 0x10, 0x28, 0x44, 0x82, 0x82, 0xfe, 0x00], /* 127 */
  [0x3c, 0x42, 0x02, 0x42, 0x3c, 0x30, 0x40, 0x3c], /* 128 */
  [0x00, 0x22, 0x00, 0x22, 0x22, 0x22, 0x7c, 0x00], /* 129 */
  [0x30, 0x00, 0x3c, 0x42, 0x7e, 0x02, 0x3c, 0x00], /* 130 */
  [0x3c, 0x42, 0x1c, 0x20, 0x3c, 0x22, 0x7c, 0x00], /* 131 */
  [0x42, 0x00, 0x1c, 0x20, 0x3c, 0x22, 0x7c, 0x00], /* 132 */
  [0x0c, 0x00, 0x1c, 0x20, 0x3c, 0x22, 0x7c, 0x00], /* 133 */
  [0x08, 0x00, 0x1c, 0x20, 0x3c, 0x22, 0x7c, 0x00], /* 134 */
  [0x00, 0x00, 0x3c, 0x02, 0x02, 0x3c, 0x60, 0x38], /* 135 */
  [0x3c, 0x42, 0x3c, 0x42, 0x7e, 0x02, 0x3c, 0x00], /* 136 */
  [0x42, 0x00, 0x3c, 0x42, 0x7e, 0x02, 0x3c, 0x00], /* 137 */
  [0x0c, 0x00, 0x3c, 0x42, 0x7e, 0x02, 0x3c, 0x00], /* 138 */
  [0x24, 0x00, 0x18, 0x10, 0x10, 0x10, 0x38, 0x00], /* 139 */
  [0x3e, 0x41, 0x0c, 0x08, 0x08, 0x08, 0x1c, 0x00], /* 140 */
  [0x0c, 0x00, 0x18, 0x10, 0x10, 0x10, 0x38, 0x00], /* 141 */
  [0x42, 0x18, 0x24, 0x42, 0x7e, 0x42, 0x42, 0x00], /* 142 */
  [0x18, 0x18, 0x00, 0x3c, 0x42, 0x7e, 0x42, 0x00], /* 143 */
  [0x30, 0x00, 0x3e, 0x04, 0x1c, 0x04, 0x3e, 0x00], /* 144 */
  [0x00, 0x00, 0xcc, 0x30, 0xfc, 0x22, 0xdc, 0x00], /* 145 */
  [0xf8, 0x24, 0x22, 0xfe, 0x22, 0x22, 0xe2, 0x00], /* 146 */
  [0x18, 0x24, 0x00, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 147 */
  [0x00, 0x42, 0x00, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 148 */
  [0x04, 0x08, 0x00, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 149 */
  [0x18, 0x24, 0x00, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 150 */
  [0x04, 0x08, 0x00, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 151 */
  [0x00, 0x42, 0x00, 0x42, 0x42, 0x7c, 0x40, 0x3c], /* 152 */
  [0x42, 0x18, 0x24, 0x42, 0x42, 0x24, 0x18, 0x00], /* 153 */
  [0x42, 0x00, 0x42, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 154 */
  [0x10, 0x10, 0x7c, 0x02, 0x02, 0x7c, 0x10, 0x10], /* 155 */
  [0x18, 0x24, 0x04, 0x0e, 0x04, 0x42, 0x3e, 0x00], /* 156 */
  [0x22, 0x14, 0x3e, 0x08, 0x3e, 0x08, 0x08, 0x00], /* 157 */
  [0x1f, 0x32, 0x1e, 0x22, 0xf2, 0x22, 0xa2, 0x67], /* 158 */
  [0x38, 0x48, 0x08, 0x3e, 0x08, 0x08, 0x09, 0x06], /* 159 */
  [0x30, 0x00, 0x1c, 0x20, 0x3c, 0x22, 0x7c, 0x00], /* 160 */
  [0x30, 0x00, 0x18, 0x10, 0x10, 0x10, 0x38, 0x00], /* 161 */
  [0x20, 0x10, 0x00, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 162 */
  [0x00, 0x20, 0x10, 0x42, 0x42, 0x42, 0x3c, 0x00], /* 163 */
  [0x4c, 0x32, 0x00, 0x3e, 0x42, 0x42, 0x42, 0x00], /* 164 */
  [0x2c, 0x32, 0x00, 0x46, 0x4a, 0x52, 0x62, 0x00], /* 165 */
  [0x3c, 0x22, 0x22, 0x7c, 0x00, 0x7e, 0x00, 0x00], /* 166 */
  [0x1c, 0x22, 0x22, 0x1c, 0x00, 0x3e, 0x00, 0x00], /* 167 */
  [0x08, 0x00, 0x08, 0x04, 0x02, 0x42, 0x3c, 0x00], /* 168 */
  [0x00, 0x00, 0x00, 0x7e, 0x02, 0x02, 0x00, 0x00], /* 169 */
  [0x00, 0x00, 0x00, 0x7e, 0x40, 0x40, 0x00, 0x00], /* 170 */
  [0x42, 0x23, 0x12, 0x6f, 0x94, 0xc2, 0x31, 0xf8], /* 171 */
  [0x42, 0x23, 0x52, 0x6f, 0x54, 0xfa, 0x41, 0x40], /* 172 */
  [0x00, 0x08, 0x00, 0x08, 0x08, 0x08, 0x08, 0x00], /* 173 */
  [0x00, 0x48, 0x24, 0x12, 0x24, 0x48, 0x00, 0x00], /* 174 */
  [0x00, 0x12, 0x24, 0x48, 0x24, 0x12, 0x00, 0x00], /* 175 */
  [0x44, 0x11, 0x44, 0x11, 0x44, 0x11, 0x44, 0x11], /* 176 */
  [0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55], /* 177 */
  [0xdb, 0xee, 0xdb, 0x77, 0xdb, 0xee, 0xdb, 0x77], /* 178 */
  [0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08], /* 179 */
  [0x08, 0x08, 0x08, 0x08, 0x0f, 0x08, 0x08, 0x08], /* 180 */
  [0x08, 0x08, 0x0f, 0x08, 0x0f, 0x08, 0x08, 0x08], /* 181 */
  [0x28, 0x28, 0x28, 0x28, 0x2f, 0x28, 0x28, 0x28], /* 182 */
  [0x00, 0x00, 0x00, 0x00, 0x3f, 0x28, 0x28, 0x28], /* 183 */
  [0x00, 0x00, 0x0f, 0x08, 0x0f, 0x08, 0x08, 0x08], /* 184 */
  [0x28, 0x28, 0x2f, 0x20, 0x2f, 0x28, 0x28, 0x28], /* 185 */
  [0x28, 0x28, 0x28, 0x28, 0x28, 0x28, 0x28, 0x28], /* 186 */
  [0x00, 0x00, 0x3f, 0x20, 0x2f, 0x28, 0x28, 0x28], /* 187 */
  [0x28, 0x28, 0x2f, 0x20, 0x3f, 0x00, 0x00, 0x00], /* 188 */
  [0x28, 0x28, 0x28, 0x28, 0x3f, 0x00, 0x00, 0x00], /* 189 */
  [0x08, 0x08, 0x0f, 0x08, 0x0f, 0x00, 0x00, 0x00], /* 190 */
  [0x00, 0x00, 0x00, 0x00, 0x0f, 0x08, 0x08, 0x08], /* 191 */
  [0x08, 0x08, 0x08, 0x08, 0xf8, 0x00, 0x00, 0x00], /* 192 */
  [0x08, 0x08, 0x08, 0x08, 0xff, 0x00, 0x00, 0x00], /* 193 */
  [0x00, 0x00, 0x00, 0x00, 0xff, 0x08, 0x08, 0x08], /* 194 */
  [0x08, 0x08, 0x08, 0x08, 0xf8, 0x08, 0x08, 0x08], /* 195 */
  [0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00], /* 196 */
  [0x08, 0x08, 0x08, 0x08, 0xff, 0x08, 0x08, 0x08], /* 197 */
  [0x08, 0x08, 0xf8, 0x08, 0xf8, 0x08, 0x08, 0x08], /* 198 */
  [0x28, 0x28, 0x28, 0x28, 0xe8, 0x28, 0x28, 0x28], /* 199 */
  [0x28, 0x28, 0xe8, 0x08, 0xf8, 0x00, 0x00, 0x00], /* 200 */
  [0x00, 0x00, 0xf8, 0x08, 0xe8, 0x28, 0x28, 0x28], /* 201 */
  [0x28, 0x28, 0xef, 0x00, 0xff, 0x00, 0x00, 0x00], /* 202 */
  [0x00, 0x00, 0xff, 0x00, 0xef, 0x28, 0x28, 0x28], /* 203 */
  [0x28, 0x28, 0xe8, 0x08, 0xe8, 0x28, 0x28, 0x28], /* 204 */
  [0x00, 0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0x00], /* 205 */
  [0x28, 0x28, 0xef, 0x00, 0xef, 0x28, 0x28, 0x28], /* 206 */
  [0x08, 0x08, 0xff, 0x00, 0xff, 0x00, 0x00, 0x00], /* 207 */
  [0x28, 0x28, 0x28, 0x28, 0xff, 0x00, 0x00, 0x00], /* 208 */
  [0x00, 0x00, 0xff, 0x00, 0xff, 0x08, 0x08, 0x08], /* 209 */
  [0x00, 0x00, 0x00, 0x00, 0xff, 0x28, 0x28, 0x28], /* 210 */
  [0x28, 0x28, 0x28, 0x28, 0xf8, 0x00, 0x00, 0x00], /* 211 */
  [0x08, 0x08, 0xf8, 0x08, 0xf8, 0x00, 0x00, 0x00], /* 212 */
  [0x00, 0x00, 0xf8, 0x08, 0xf8, 0x08, 0x08, 0x08], /* 213 */
  [0x00, 0x00, 0x00, 0x00, 0xf8, 0x28, 0x28, 0x28], /* 214 */
  [0x28, 0x28, 0x28, 0x28, 0xff, 0x28, 0x28, 0x28], /* 215 */
  [0x08, 0x08, 0xff, 0x08, 0xff, 0x08, 0x08, 0x08], /* 216 */
  [0x08, 0x08, 0x08, 0x08, 0x0f, 0x00, 0x00, 0x00], /* 217 */
  [0x00, 0x00, 0x00, 0x00, 0xf8, 0x08, 0x08, 0x08], /* 218 */
  [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], /* 219 */
  [0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff], /* 220 */
  [0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f], /* 221 */
  [0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, 0xf0], /* 222 */
  [0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00], /* 223 */
  [0x00, 0x00, 0x8c, 0x52, 0x22, 0x52, 0x8c, 0x00], /* 224 */
  [0x00, 0x3c, 0x42, 0x3e, 0x42, 0x3e, 0x02, 0x02], /* 225 */
  [0x00, 0x7e, 0x42, 0x02, 0x02, 0x02, 0x02, 0x00], /* 226 */
  [0x00, 0xfc, 0x2a, 0x28, 0x28, 0x28, 0x28, 0x00], /* 227 */
  [0x7e, 0x42, 0x04, 0x18, 0x04, 0x42, 0x7e, 0x00], /* 228 */
  [0x00, 0x00, 0x7c, 0x12, 0x12, 0x12, 0x0c, 0x00], /* 229 */
  [0x00, 0x22, 0x22, 0x22, 0x5e, 0x02, 0x02, 0x01], /* 230 */
  [0x00, 0xcc, 0x32, 0x10, 0x10, 0x10, 0x10, 0x00], /* 231 */
  [0x3e, 0x08, 0x1c, 0x22, 0x22, 0x1c, 0x08, 0x3e], /* 232 */
  [0x18, 0x24, 0x42, 0x7e, 0x42, 0x24, 0x18, 0x00], /* 233 */
  [0x18, 0x24, 0x42, 0x42, 0x24, 0x24, 0x66, 0x00], /* 234 */
  [0x38, 0x04, 0x18, 0x3c, 0x42, 0x42, 0x3c, 0x00], /* 235 */
  [0x00, 0x46, 0xa9, 0x91, 0xa9, 0x46, 0x00, 0x00], /* 236 */
  [0x40, 0x20, 0x3c, 0x52, 0x4a, 0x3c, 0x02, 0x01], /* 237 */
  [0x30, 0x08, 0x04, 0x3c, 0x04, 0x08, 0x30, 0x00], /* 238 */
  [0x3c, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00], /* 239 */
  [0x00, 0x7e, 0x00, 0x7e, 0x00, 0x7e, 0x00, 0x00], /* 240 */
  [0x08, 0x08, 0x3e, 0x08, 0x08, 0x00, 0x3e, 0x00], /* 241 */
  [0x08, 0x10, 0x20, 0x10, 0x08, 0x00, 0x7e, 0x00], /* 242 */
  [0x10, 0x08, 0x04, 0x08, 0x10, 0x00, 0x7e, 0x00], /* 243 */
  [0x30, 0x48, 0x48, 0x08, 0x08, 0x08, 0x08, 0x08], /* 244 */
  [0x08, 0x08, 0x08, 0x08, 0x08, 0x09, 0x09, 0x06], /* 245 */
  [0x18, 0x18, 0x00, 0x7e, 0x00, 0x18, 0x18, 0x00], /* 246 */
  [0x00, 0x4c, 0x32, 0x00, 0x4c, 0x32, 0x00, 0x00], /* 247 */
  [0x0c, 0x12, 0x12, 0x0c, 0x00, 0x00, 0x00, 0x00], /* 248 */
  [0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00], /* 249 */
  [0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00], /* 250 */
  [0xf0, 0x10, 0x10, 0x10, 0x10, 0x13, 0x14, 0x18], /* 251 */
  [0x1e, 0x22, 0x22, 0x22, 0x22, 0x00, 0x00, 0x00], /* 252 */
  [0x0c, 0x12, 0x08, 0x04, 0x1e, 0x00, 0x00, 0x00], /* 253 */
  [0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00], /* 254 */
  [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], /* 255 */
];

pub static PALETTE: &[(u8, u8, u8, u8); 16] = &[
  (0x14, 0x0c, 0x1c, 0xff),
  (0x44, 0x24, 0x34, 0xff),
  (0x30, 0x34, 0x6d, 0xff),
  (0x4e, 0x4a, 0x4e, 0xff),
  (0x85, 0x4c, 0x30, 0xff),
  (0x34, 0x65, 0x24, 0xff),
  (0xd0, 0x46, 0x48, 0xff),
  (0x75, 0x71, 0x61, 0xff),
  (0x59, 0x7d, 0xce, 0xff),
  (0xd2, 0x7d, 0x2c, 0xff),
  (0x85, 0x95, 0xa1, 0xff),
  (0x6d, 0xaa, 0x2c, 0xff),
  (0xd2, 0xaa, 0x99, 0xff),
  (0x6d, 0xc2, 0xca, 0xff),
  (0xda, 0xd4, 0x5e, 0xff),
  (0xde, 0xee, 0xd6, 0xff),
];

#[rustfmt::skip]
pub static ASCII_HEX_DECODER: &[u8; 256] = &[
  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  0,  0,  0,  0,  0,  0,
	0, 10, 11, 12, 13, 14, 15,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0, 10, 11, 12, 13, 14, 15,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
	0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
pub static ADCPM_INDEX_TABLE: &[i32; 16] = &[
  -1, -1, -1, -1, 2, 4, 6, 8,
  -1, -1, -1, -1, 2, 4, 6, 8
];

pub static ADCPM_STEP_TABLE: &[u16; 89] = &[
  7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66, 73,
  80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449, 494,
  544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272, 2499,
  2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493, 10442, 11497,
  12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

pub const PIX_AUDIO_VOICES: usize = 16;
