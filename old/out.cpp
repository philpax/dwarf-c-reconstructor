// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/TgaLoader.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

typedef struct { //19
	unsigned char Header[12]; //20
} TGAHeader; //21

typedef struct { //24
	unsigned char header[6]; //25
	GLuint bytesPerPixel; //26
	GLuint imageSize; //27
	GLuint temp; //28
	GLuint type; //29
	GLuint Height; //30
	GLuint Width; //31
	GLuint Bpp; //32
} TGA; //33

class TgaLoader { //37
private:
	TGAHeader tgaheader; //46
	TGA tga; //47
	unsigned char uTGAcompare[12]; //49
	unsigned char ugTGAcompare[12]; //50
	unsigned char cTGAcompare[12]; //51
	unsigned char gTGAcompare[12]; //52
	private: void TgaLoader() //40
	;
	private: int LoadImage() //43
	;
	2: int LoadUncompressedTGA() //55
	;
	2: int LoadGrayscaleTGA() //56
	;
	2: int LoadCompressedTGA() //57
	;
};

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/TgaLoader.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/encdec.cpp

int read_line(int ind, unsigned char *src, unsigned char *line) //176
{
	{
		char c; //178
		int index; //179
		int ind0; //180
	}
}

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/encdec.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/facebin.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //9
	UNKNOWN = 0,
	OPEN = 1,
	CREATE = 2,
	EDIT = 3,
} openStatus; //14

typedef struct { //16
	unsigned int offset; //17
	unsigned int size; //18
} textureInfo; //19

typedef struct { //21
	int magic; //22
	unsigned int mpb_offset; //23
	unsigned int mpb_size; //24
	textureInfo tex[18]; //25
	textureInfo map[3]; //26
} faceHeader; //27

class faceBin { //29
private:
	faceHeader header; //49
	unsigned char *pmpb; //50
	unsigned char *pimg[18]; //51
	unsigned char *pmap[3]; //52
	FILE *fd; //53
	openStatus status; //54
	int isInfoSet; //55
	unsigned int info[32]; //56
	private: void faceBin() //32
	;
	private: void ~faceBin() //33
	;
	private: int createFile() //35
	;
	private: int openFile() //36
	;
	private: int editFile() //37
	;
	private: int closeFile() //38
	;
	private: int addImage() //39
	;
	private: int addMap() //40
	;
	private: int setMPB() //41
	;
	private: int getImage() //42
	;
	private: int getMap() //43
	;
	private: int getMPB() //44
	;
	private: int setInfo() //45
	;
	private: int getInfo() //46
	;
};

extern struct __sFILE __sF; //154

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/facebin.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/gl_util_io.cpp

struct stat; { //44
	long long unsigned int st_dev; //45
	unsigned char __pad0[4]; //46
	long unsigned int __st_ino; //48
	unsigned int st_mode; //49
	unsigned int st_nlink; //50
	long unsigned int st_uid; //52
	long unsigned int st_gid; //53
	long long unsigned int st_rdev; //55
	unsigned char __pad3[4]; //56
	long long int st_size; //58
	long unsigned int st_blksize; //59
	long long unsigned int st_blocks; //60
	long unsigned int st_atime; //62
	long unsigned int st_atime_nsec; //63
	long unsigned int st_mtime; //65
	long unsigned int st_mtime_nsec; //66
	long unsigned int st_ctime; //68
	long unsigned int st_ctime_nsec; //69
	long long unsigned int st_ino; //71
};

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef struct mpTexture { //55
	GLuint textureName; //57
} mpTexture; //58

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

typedef struct { //19
	unsigned char Header[12]; //20
} TGAHeader; //21

typedef struct { //24
	unsigned char header[6]; //25
	GLuint bytesPerPixel; //26
	GLuint imageSize; //27
	GLuint temp; //28
	GLuint type; //29
	GLuint Height; //30
	GLuint Width; //31
	GLuint Bpp; //32
} TGA; //33

class TgaLoader { //37
private:
	TGAHeader tgaheader; //46
	TGA tga; //47
	unsigned char uTGAcompare[12]; //49
	unsigned char ugTGAcompare[12]; //50
	unsigned char cTGAcompare[12]; //51
	unsigned char gTGAcompare[12]; //52
	private: void TgaLoader() //40
	;
	private: int LoadImage() //43
	;
	2: int LoadUncompressedTGA() //55
	;
	2: int LoadGrayscaleTGA() //56
	;
	2: int LoadCompressedTGA() //57
	;
};

typedef struct { //251
	char signature[8]; //252
	int length; //253
	int chunk_type; //254
	int width; //255
	int height; //256
	char depth; //257
	char colorType; //258
	char compression; //259
	char filter; //260
	char interlace; //261
	int crc; //262
} PNGHeader; //263

char* findExtention(const char *name) //241
{
	{
		int len; //243
		const char *p; //244
	}
}

int checkPNGcolorType(const char *file) //267
{
	{
		FILE *fp; //269
		PNGHeader header; //270
	}
}

int readImageFile(const char *file, mpeImage *image) //41
{
	{
		class TgaLoader tga; //43
		char *ext; //44
		{
			int ctype; //53
			unsigned int w, unsigned int h; //54
			unsigned char *buf, unsigned char *ibuf, unsigned char *src, unsigned char *dst; //55
			int ccomp, int ssize; //56
			{
				unsigned int i; //80
			}
		}
	}
}

int readImageFileWithoutExt(const char *file, mpeImage *image) //102
{
	{
		char fname[1024]; //104
	}
}

int loadMap(const char *file, unsigned char **buf, int *width, int *height) //119
{
	{
		mpeImage image; //121
	}
}

int loadTexture( //137
				const char *dirname, const char *filename, unsigned char color, //138
				struct mpTexture **tex) //139
{
	{
		char file[1024]; //141
		mpeImage image; //142
		unsigned char *imageData; //143
		GLenum format; //144
		{
			int x, int y; //158
			unsigned char *src; //159
			unsigned char *dest; //160
		}
	}
}

void closeTexture(struct mpTexture *tex) //202
;

int loadFileData( //208
				const char *dir, const char *file, unsigned int *size, char **data) //209
{
	{
		FILE *fp; //211
		char filePath[1024]; //212
		struct stat buf; //213
	}
}

extern mpErrorCode errCode; //171

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/gl_util_io.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/lodepng.cpp

typedef struct LodeZlib_DecompressSettings { //317
	unsigned int ignoreAdler32; //319
} LodeZlib_DecompressSettings; //320

typedef struct LodeZlib_CompressSettings { //332
	unsigned int btype; //335
	unsigned int useLZ77; //336
	unsigned int windowSize; //337
} LodeZlib_CompressSettings; //338

typedef struct LodePNG_InfoColor { //356
	unsigned int colorType; //359
	unsigned int bitDepth; //360
	unsigned char *palette; //383
	size_t palettesize; //384
	unsigned int key_defined; //394
	unsigned int key_r; //395
	unsigned int key_g; //396
	unsigned int key_b; //397
} LodePNG_InfoColor; //398

typedef struct LodePNG_Time { //434
	unsigned int year; //436
	unsigned char month; //437
	unsigned char day; //438
	unsigned char hour; //439
	unsigned char minute; //440
	unsigned char second; //441
} LodePNG_Time; //442

typedef struct LodePNG_Text { //458
	size_t num; //462
	char **keys; //463
	char **strings; //464
} LodePNG_Text; //465

typedef struct LodePNG_IText { //485
	size_t num; //489
	char **keys; //490
	char **langtags; //491
	char **transkeys; //492
	char **strings; //493
} LodePNG_IText; //494

typedef struct LodePNG_UnknownChunks { //512
	unsigned char *data[3]; //522
	unsigned int datasize[3]; //523
} LodePNG_UnknownChunks; //525

typedef struct LodePNG_InfoPng { //537
	unsigned int width; //546
	unsigned int height; //547
	unsigned int compressionMethod; //548
	unsigned int filterMethod; //549
	unsigned int interlaceMethod; //550
	LodePNG_InfoColor color; //551
	unsigned int background_defined; //566
	unsigned int background_r; //567
	unsigned int background_g; //568
	unsigned int background_b; //569
	LodePNG_Text text; //572
	LodePNG_IText itext; //575
	unsigned char time_defined; //578
	LodePNG_Time time; //579
	unsigned int phys_defined; //582
	unsigned int phys_x; //583
	unsigned int phys_y; //584
	unsigned char phys_unit; //585
	LodePNG_UnknownChunks unknown_chunks; //591
} LodePNG_InfoPng; //594

typedef struct LodePNG_InfoRaw { //607
	LodePNG_InfoColor color; //609
} LodePNG_InfoRaw; //610

typedef struct LodePNG_DecodeSettings { //638
	LodeZlib_DecompressSettings zlibsettings; //640
	unsigned int ignoreCrc; //642
	unsigned int color_convert; //643
	unsigned int readTextChunks; //646
	unsigned int rememberUnknownChunks; //650
} LodePNG_DecodeSettings; //652

typedef struct LodePNG_Decoder { //661
	LodePNG_DecodeSettings settings; //663
	LodePNG_InfoRaw infoRaw; //664
	LodePNG_InfoPng infoPng; //665
	unsigned int error; //666
} LodePNG_Decoder; //667

typedef struct LodePNG_EncodeSettings { //704
	LodeZlib_CompressSettings zlibsettings; //706
	unsigned int autoLeaveOutAlphaChannel; //708
	unsigned int force_palette; //709
	unsigned int add_id; //711
	unsigned int text_compression; //712
} LodePNG_EncodeSettings; //714

typedef struct LodePNG_Encoder { //724
	LodePNG_EncodeSettings settings; //726
	LodePNG_InfoPng infoPng; //727
	LodePNG_InfoRaw infoRaw; //728
	unsigned int error; //729
} LodePNG_Encoder; //730

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef struct vector { //59
	void *data; //61
	size_t size; //62
	size_t allocsize; //63
	unsigned int typesize; //64
} vector; //65

typedef struct uivector { //138
	unsigned int *data; //140
	size_t size; //141
	size_t allocsize; //142
} uivector; //143

typedef struct ucvector { //213
	unsigned char *data; //215
	size_t size; //216
	size_t allocsize; //217
} ucvector; //218

typedef struct Coin { //399
	uivector symbols; //401
	float weight; //402
} Coin; //403

typedef struct HuffmanTree { //460
	uivector tree2d; //462
	uivector tree1d; //463
	uivector lengths; //464
	unsigned int maxbitlen; //465
	unsigned int numcodes; //466
} HuffmanTree; //467

void* vector_get(vector *p, size_t index) //127
;

void uivector_init(uivector *p) //178
;

void uivector_swap(uivector *p, uivector *q) //200
{
	{
		size_t tmp; //202
		unsigned int *tmpp; //203
	}
}

unsigned int HuffmanTree_getCode(const HuffmanTree *tree, unsigned int index) //711
;

unsigned int HuffmanTree_getLength(const HuffmanTree *tree, unsigned int index) //716
;

unsigned int huffmanDecodeSymbol( //768
				const unsigned char *in, size_t *bp, //768
				const HuffmanTree *codetree, size_t inbitlength) //769
{
	{
		unsigned int treepos, unsigned int ct; //771
	}
}

unsigned int getHash(const unsigned char *data, size_t size, size_t pos) //1241
{
	{
		unsigned int result; //1243
		size_t amount, size_t i; //1244
	}
}

unsigned int update_adler32(unsigned int adler, const unsigned char *data, unsigned int len) //1748
{
	{
		unsigned int s1; //1750
		unsigned int s2; //1751
		{
			unsigned int amount; //1756
		}
	}
}

void Crc32_make_crc_table() //1958
{
	{
		unsigned int c, unsigned int k, unsigned int n; //1960
	}
}

unsigned int Crc32_update_crc(const unsigned char *buf, unsigned int crc, size_t len) //1977
{
	{
		unsigned int c; //1979
		size_t n; //1980
	}
}

unsigned char readBitFromReversedStream(size_t *bitpointer, const unsigned char *bitstream) //2000
{
	{
		unsigned char result; //2002
	}
}

unsigned int getNumColorChannels(unsigned int colorType) //2200
;

int abs(int __n) //83
;

unsigned char paethPredictor(short int a, short int b, short int c) //2828
{
	{
		short int pa; //2830
		short int pb; //2831
		short int pc; //2832
	}
}

void setBitOfReversedStream0(size_t *bitpointer, unsigned char *bitstream, unsigned char bit) //2019
;

void filterScanline(unsigned char *out, const unsigned char *scanline, const unsigned char *prevline, size_t length, size_t bytewidth, unsigned char filterType) //4020
{
	{
		size_t i; //4022
	}
}

unsigned int isPaletteFullyOpaque(const unsigned char *palette, size_t palettesize) //4367
{
	{
		size_t i; //4369
	}
}

unsigned int ucvector_resize(ucvector *p, size_t size) //227
{
	{
		size_t newsize; //231
		void *data; //232
	}
}

unsigned int ucvector_push_back(ucvector *p, unsigned char c) //272
;

void addBitsToStreamReversed(size_t *bitpointer, ucvector *bitstream, unsigned int value, size_t nbits) //340
{
	{
		size_t i; //342
	}
}

void writeLZ77data(size_t *bp, ucvector *out, const uivector *lz77_encoded, const HuffmanTree *codes, const HuffmanTree *codesD) //1446
{
	{
		size_t i; //1448
		{
			unsigned int val; //1451
			{
				unsigned int length_index; //1455
				unsigned int n_length_extra_bits; //1456
				unsigned int length_extra_bits; //1457
				unsigned int distance_code; //1459
				unsigned int distance_index; //1461
				unsigned int n_distance_extra_bits; //1462
				unsigned int distance_extra_bits; //1463
			}
		}
	}
}

void LodePNG_set32bitInt(unsigned char *buffer, unsigned int value) //2040
;

unsigned int string_resize(char **out, size_t size) //283
{
	{
		char *data; //285
	}
}

unsigned int uivector_resize(uivector *p, size_t size) //152
{
	{
		size_t newsize; //156
		void *data; //157
	}
}

unsigned int HuffmanTree_make2DTree(HuffmanTree *tree) //497
{
	{
		unsigned int nodefilled; //499
		unsigned int treepos; //500
		unsigned int n, unsigned int i; //501
		{
			unsigned char bit; //512
		}
	}
}

unsigned int HuffmanTree_makeFromLengths2(HuffmanTree *tree) //545
{
	{
		uivector blcount; //547
		uivector nextcode; //548
		unsigned int bits, unsigned int n, unsigned int error; //549
	}
}

unsigned int HuffmanTree_makeFromLengths(HuffmanTree *tree, const unsigned int *bitlen, size_t numcodes, unsigned int maxbitlen) //581
{
	{
		unsigned int i; //583
	}
}

unsigned int uivector_push_back(uivector *p, unsigned int c) //185
;

size_t searchCodeIndex(const unsigned int *array, size_t array_size, size_t value) //1142
{
	{
		size_t left; //1149
		size_t right; //1150
		{
			size_t mid; //1153
		}
	}
}

unsigned int uivector_copy(uivector *p, const uivector *q) //192
{
	{
		size_t i; //194
	}
}

unsigned int readBitsFromStream(size_t *bitpointer, const unsigned char *bitstream, size_t nbits) //358
{
	{
		unsigned int result, unsigned int i; //360
	}
}

void HuffmanTree_init(HuffmanTree *tree) //482
;

void getTreeInflateFixed(HuffmanTree *tree, HuffmanTree *treeD) //797
;

unsigned int vector_resize(vector *p, size_t size) //67
{
	{
		size_t newsize; //71
		void *data; //72
	}
}

void vector_init(vector *p, unsigned int typesize) //111
;

unsigned int countInitialZeros(const unsigned char *data, size_t size, size_t pos) //1252
{
	{
		size_t max_count; //1254
		size_t i; //1255
	}
}

void addLengthDistance(uivector *values, size_t length, size_t distance) //1161
{
	{
		unsigned int length_code; //1169
		unsigned int extra_length; //1170
		unsigned int dist_code; //1171
		unsigned int extra_distance; //1172
	}
}

void vector_cleanup(void *p) //98
;

unsigned int deflateFixed(ucvector *out, const unsigned char *data, size_t datasize, const LodeZlib_CompressSettings *settings) //1689
{
	{
		HuffmanTree codes; //1691
		HuffmanTree codesD; //1692
		unsigned int BFINAL; //1694
		unsigned int error; //1695
		size_t i, size_t bp; //1696
		{
			uivector lz77_encoded; //1710
		}
	}
}

void Coin_init(Coin *c) //405
;

void Coin_sort(Coin *data, size_t amount) //432
{
	{
		size_t gap; //434
		unsigned char swapped; //435
		{
			size_t i; //438
			{
				size_t j; //445
				{
					float temp; //448
				}
			}
		}
	}
}

unsigned int HuffmanTree_fillInCoins(vector *coins, const unsigned int *frequencies, unsigned int numcodes, size_t sum) //592
{
	{
		unsigned int i; //594
		{
			Coin *coin; //597
		}
	}
}

unsigned int vector_resized(vector *p, size_t size, void(*)() *dtor) //85
{
	{
		size_t i; //87
	}
}

void vector_swap(vector *p, vector *q) //118
{
	{
		size_t tmp; //120
		void *tmpp; //121
	}
}

void Coin_copy(Coin *c1, const Coin *c2) //415
;

void addCoins(Coin *c1, const Coin *c2) //421
{
	{
		size_t i; //423
	}
}

void vector_cleanupd(vector *p, void(*)() *dtor) //105
;

unsigned int deflateDynamic(ucvector *out, const unsigned char *data, size_t datasize, const LodeZlib_CompressSettings *settings) //1472
{
	{
		unsigned int error; //1484
		uivector lz77_encoded; //1486
		HuffmanTree codes; //1487
		HuffmanTree codesD; //1488
		HuffmanTree codelengthcodes; //1489
		uivector frequencies; //1490
		uivector frequenciesD; //1491
		uivector amounts; //1492
		uivector lldl; //1493
		uivector lldll; //1494
		uivector clcls; //1495
		unsigned int BFINAL; //1497
		size_t numcodes, size_t numcodesD, size_t i; //1498
		size_t bp; //1499
		unsigned int HLIT, unsigned int HDIST, unsigned int HCLEN; //1500
		{
			unsigned int symbol; //1542
			{
				unsigned int dist; //1546
			}
		}
		{
			unsigned int j; //1570
			{
				size_t k; //1591
				unsigned int num, unsigned int rest; //1592
			}
		}
	}
}

unsigned char readBitFromStream(size_t *bitpointer, const unsigned char *bitstream) //351
{
	{
		unsigned char result; //353
	}
}

unsigned int inflateNoCompression(ucvector *out, const unsigned char *in, size_t *bp, size_t *pos, size_t inlength) //1066
{
	{
		size_t p; //1069
		unsigned int LEN, unsigned int NLEN, unsigned int n, unsigned int error; //1070
	}
}

unsigned int deflateNoCompression(ucvector *out, const unsigned char *data, size_t datasize) //1409
{
	{
		size_t i, size_t j, size_t numdeflateblocks; //1413
		unsigned int datapos; //1414
		{
			unsigned int BFINAL, unsigned int BTYPE, unsigned int LEN, unsigned int NLEN; //1417
			unsigned char firstbyte; //1418
		}
	}
}

void ucvector_init_buffer(ucvector *p, unsigned char *buffer, size_t size) //265
;

unsigned int adler32(const unsigned char *data, unsigned int len) //1772
;

void ucvector_init(ucvector *p) //257
;

unsigned int LodePNG_read32bitInt(const unsigned char *buffer) //2035
;

unsigned char LodePNG_chunk_type_equals(const unsigned char *chunk, const char *type) //2072
;

unsigned int addChunk_PLTE(ucvector *out, const LodePNG_InfoColor *info) //3803
{
	{
		unsigned int error; //3805
		size_t i; //3806
		ucvector PLTE; //3807
	}
}

unsigned int LodePNG_compress(unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize, const LodeZlib_CompressSettings *settings) //1943
;

unsigned int getBpp(unsigned int colorType, unsigned int bitDepth) //2213
;

void Adam7_interlace(unsigned char *out, const unsigned char *in, unsigned int w, unsigned int h, unsigned int bpp) //4227
{
	{
		unsigned int passw[7], unsigned int passh[7]; //4230
		unsigned int filter_passstart[8], unsigned int padded_passstart[8], unsigned int passstart[8]; //4231
		unsigned int i; //4232
		{
			unsigned int x, unsigned int y, unsigned int b; //4240
			size_t bytewidth; //4241
			{
				size_t pixelinstart; //4245
				size_t pixeloutstart; //4246
			}
		}
		{
			unsigned int x, unsigned int y, unsigned int b; //4258
			unsigned int ilinebits; //4259
			unsigned int olinebits; //4260
			size_t obp, size_t ibp; //4261
			{
				unsigned char bit; //4269
			}
		}
	}
}

void Adam7_deinterlace(unsigned char *out, const unsigned char *in, unsigned int w, unsigned int h, unsigned int bpp) //3039
{
	{
		unsigned int passw[7], unsigned int passh[7]; //3043
		unsigned int filter_passstart[8], unsigned int padded_passstart[8], unsigned int passstart[8]; //3044
		unsigned int i; //3045
		{
			unsigned int x, unsigned int y, unsigned int b; //3053
			size_t bytewidth; //3054
			{
				size_t pixelinstart; //3058
				size_t pixeloutstart; //3059
			}
		}
		{
			unsigned int x, unsigned int y, unsigned int b; //3071
			unsigned int ilinebits; //3072
			unsigned int olinebits; //3073
			size_t obp, size_t ibp; //3074
			{
				unsigned char bit; //3082
			}
		}
	}
}

void string_cleanup(char **out) //300
;

unsigned int LodePNG_InfoPng_copy(LodePNG_InfoPng *dest, const LodePNG_InfoPng *source) //2528
{
	{
		unsigned int error; //2530
	}
}

unsigned int readBitsFromReversedStream(size_t *bitpointer, const unsigned char *bitstream, size_t nbits) //2007
{
	{
		unsigned int result; //2009
		size_t i; //2010
	}
}

unsigned int LodePNG_decompress(unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize, const LodeZlib_DecompressSettings *settings) //1936
;

unsigned int ucvector_resizev(ucvector *p, size_t size, unsigned char value) //247
{
	{
		size_t oldsize, size_t i; //249
	}
}

unsigned int isFullyOpaque(const unsigned char *image, unsigned int w, unsigned int h, const LodePNG_InfoColor *info) //4378
{
	{
		unsigned int i, unsigned int numpixels; //4382
	}
}

void writeSignature(ucvector *out) //3770
;

unsigned int addChunk_IHDR(ucvector *out, unsigned int w, unsigned int h, unsigned int bitDepth, unsigned int colorType, unsigned int interlaceMethod) //3783
{
	{
		unsigned int error; //3785
		ucvector header; //3786
	}
}

unsigned int addChunk_bKGD(ucvector *out, const LodePNG_InfoPng *info) //3955
{
	{
		unsigned int error; //3957
		ucvector bKGD; //3958
	}
}

unsigned int addChunk_pHYs(ucvector *out, const LodePNG_InfoPng *info) //4002
{
	{
		unsigned int error; //4004
		ucvector data; //4005
	}
}

unsigned int addChunk_IDAT(ucvector *out, const unsigned char *data, size_t datasize, LodeZlib_CompressSettings *zlibsettings) //3856
{
	{
		ucvector zlibdata; //3858
		unsigned int error; //3859
	}
}

unsigned int addChunk_tIME(ucvector *out, const LodePNG_Time *time) //3985
{
	{
		unsigned int error; //3987
		unsigned char *data; //3988
	}
}

unsigned int addChunk_zTXt(ucvector *out, const char *keyword, const char *textstring, LodeZlib_CompressSettings *zlibsettings) //3894
{
	{
		unsigned int error; //3896
		ucvector data, ucvector compressed; //3897
		size_t i, size_t textsize; //3898
	}
}

unsigned int addChunk_iTXt(ucvector *out, unsigned int compressed, const char *keyword, const char *langtag, const char *transkey, const char *textstring, LodeZlib_CompressSettings *zlibsettings) //3918
{
	{
		unsigned int error; //3920
		ucvector data, ucvector compressed_data; //3921
		size_t i, size_t textsize; //3922
	}
}

unsigned int addChunk_IEND(ucvector *out) //3870
{
	{
		unsigned int error; //3872
	}
}

unsigned int addUnknownChunks(ucvector *out, unsigned char *data, size_t datasize) //4429
{
	{
		unsigned char *inchunk; //4431
		{
			unsigned int error; //4434
		}
	}
}

unsigned int addChunk_tEXt(ucvector *out, const char *keyword, const char *textstring) //3879
{
	{
		unsigned int error; //3881
		size_t i; //3882
		ucvector text; //3883
	}
}

unsigned int addChunk_tRNS(ucvector *out, const LodePNG_InfoColor *info) //3819
{
	{
		unsigned int error; //3821
		size_t i; //3822
		ucvector tRNS; //3823
	}
}

void LodePNG_add32bitInt(ucvector *buffer, unsigned int value) //2049
;

unsigned int encodeLZ77(uivector *out, const unsigned char *in, size_t insize, unsigned int windowSize) //1266
{
	{
		vector table; //1269
		uivector tablepos1, uivector tablepos2; //1270
		uivector initialZerosTable; //1271
		unsigned int pos, unsigned int i, unsigned int error; //1272
		{
			uivector *v; //1278
		}
		{
			unsigned int length, unsigned int offset, unsigned int tablepos, unsigned int max_offset; //1292
			unsigned int hash, unsigned int initialZeros; //1293
			unsigned int backpos, unsigned int current_offset, unsigned int t1, unsigned int t2, unsigned int skip, unsigned int current_length; //1294
			const unsigned char *lastptr, const unsigned char *foreptr, const unsigned char *backptr; //1295
			{
				unsigned int j, unsigned int local_hash; //1373
			}
		}
		{
			uivector *v; //1397
		}
	}
}

unsigned int getTreeInflateDynamic( //805
				HuffmanTree *codetree, HuffmanTree *codetreeD, HuffmanTree *codelengthcodetree, //805
				const unsigned char *in, size_t *bp, size_t inlength) //806
{
	{
		unsigned int error; //810
		unsigned int n, unsigned int HLIT, unsigned int HDIST, unsigned int HCLEN, unsigned int i; //811
		uivector bitlen; //812
		uivector bitlenD; //813
		uivector codelengthcode; //814
		size_t inbitlength; //815
		{
			unsigned int code; //853
			{
				unsigned int replength; //862
				unsigned int value; //863
			}
			{
				unsigned int replength; //890
			}
			{
				unsigned int replength; //914
			}
		}
	}
}

unsigned int generateDistanceTree(HuffmanTree *tree) //745
{
	{
		unsigned int i, unsigned int error; //747
		uivector bitlen; //748
	}
}

unsigned int addChunk(ucvector *out, const char *chunkName, const unsigned char *data, size_t length) //3762
{
	{
		unsigned int error; //3764
	}
}

unsigned int generateFixedTree(HuffmanTree *tree) //723
{
	{
		unsigned int i, unsigned int error; //725
		uivector bitlen; //726
	}
}

unsigned int uivector_resizev(uivector *p, size_t size, unsigned int value) //170
{
	{
		size_t oldsize, size_t i; //172
	}
}

unsigned int Crc32_crc(const unsigned char *buf, size_t len) //1991
{
	Crc32_update_crc(); //1993
}

void setBitOfReversedStream(size_t *bitpointer, unsigned char *bitstream, unsigned char bit) //2027
;

unsigned int checkColorValidity(unsigned int colorType, unsigned int bd) //2186
;

void Adam7_getpassvalues(unsigned int *passw, unsigned int *passh, size_t *filter_passstart, size_t *padded_passstart, size_t *passstart, unsigned int w, unsigned int h, unsigned int bpp) //2851
{
	{
		unsigned int i; //2854
	}
}

unsigned int unfilterScanline(unsigned char *recon, const unsigned char *scanline, const unsigned char *precon, size_t bytewidth, unsigned char filterType, size_t length) //2937
{
	{
		size_t i; //2947
	}
}

unsigned int unfilter(unsigned char *out, const unsigned char *in, unsigned int w, unsigned int h, unsigned int bpp) //3008
{
	{
		unsigned int y; //3018
		unsigned char *prevline; //3019
		size_t bytewidth; //3021
		size_t linebytes; //3022
		{
			size_t outindex; //3026
			size_t inindex; //3027
			unsigned char filterType; //3028
			unsigned int error; //3030
		}
	}
}

void removePaddingBits(unsigned char *out, const unsigned char *in, size_t olinebits, size_t ilinebits, unsigned int h) //3090
{
	{
		unsigned int y; //3098
		size_t diff; //3099
		size_t ibp, size_t obp; //3100
		{
			size_t x; //3103
			{
				unsigned char bit; //3106
			}
		}
	}
}

void addPaddingBits(unsigned char *out, const unsigned char *in, size_t olinebits, size_t ilinebits, unsigned int h) //4207
{
	{
		unsigned int y; //4211
		size_t diff; //4212
		size_t obp, size_t ibp; //4213
		{
			size_t x; //4216
			{
				unsigned char bit; //4219
			}
		}
	}
}

void ucvector_cleanup(void *p) //220
;

void uivector_cleanup(void *p) //145
;

void HuffmanTree_cleanup(HuffmanTree *tree) //489
;

void Coin_cleanup(void *c) //410
;

void string_init(char **out) //294
{
	string_resize(); //297
}

void string_set(char **out, const char *in) //306
{
	{
		size_t insize, size_t i; //308
	}
}

void addBitToStream(size_t *bitpointer, ucvector *bitstream, unsigned char bit) //327
;

void addBitsToStream(size_t *bitpointer, ucvector *bitstream, unsigned int value, size_t nbits) //334
{
	{
		size_t i; //336
	}
}

void addHuffmanSymbol(size_t *bp, ucvector *compressed, unsigned int code, unsigned int bitlen) //1136
{
	addBitsToStreamReversed(); //1138
}

unsigned int HuffmanTree_makeFromFrequencies(HuffmanTree *tree, const unsigned int *frequencies, size_t numcodes, unsigned int maxbitlen) //617
{
	{
		unsigned int i, unsigned int j; //619
		size_t sum, size_t numpresent; //620
		unsigned int error; //621
		vector prev_row; //623
		vector coins; //624
		{
			Coin *coin; //698
		}
	}
}

unsigned int inflateHuffmanBlock(ucvector *out, const unsigned char *in, size_t *bp, size_t *pos, size_t inlength, unsigned int btype) //959
{
	{
		unsigned int error; //961
		HuffmanTree codetree; //962
		HuffmanTree codetreeD; //963
		size_t inbitlength; //964
		{
			HuffmanTree codelengthcodetree; //972
		}
		{
			unsigned int code; //980
			{
				size_t length; //997
				unsigned int codeD, unsigned int distance, unsigned int numextrabitsD; //998
				size_t start, size_t forward, size_t backward, size_t numextrabits; //999
			}
		}
	}
}

unsigned int LodeFlate_inflate(ucvector *out, const unsigned char *in, size_t insize, size_t inpos) //1097
{
	{
		size_t bp; //1099
		unsigned int BFINAL; //1100
		size_t pos; //1101
		unsigned int error; //1103
		{
			unsigned int BTYPE; //1107
		}
	}
}

unsigned int LodeFlate_deflate(ucvector *out, const unsigned char *data, size_t datasize, const LodeZlib_CompressSettings *settings) //1732
{
	{
		unsigned int error; //1734
	}
}

void LodeZlib_add32bitInt(ucvector *buffer, unsigned int value) //1782
;

unsigned int LodeZlib_read32bitInt(const unsigned char *buffer) //1791
;

unsigned int LodeZlib_decompress(unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize, const LodeZlib_DecompressSettings *settings) //1802
{
	{
		unsigned int error; //1804
		unsigned int CM, unsigned int CINFO, unsigned int FDICT; //1805
		ucvector outv; //1806
		{
			unsigned int ADLER32; //1829
			unsigned int checksum; //1830
		}
	}
}

unsigned int LodeZlib_compress(unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize, const LodeZlib_CompressSettings *settings) //1841
{
	{
		ucvector deflatedata, ucvector outv; //1844
		size_t i; //1845
		unsigned int error; //1846
		unsigned int ADLER32; //1848
		unsigned int CMF; //1850
		unsigned int FLEVEL; //1851
		unsigned int FDICT; //1852
		unsigned int CMFFLG; //1853
		unsigned int FCHECK; //1854
	}
}

void LodeZlib_CompressSettings_init(LodeZlib_CompressSettings *settings) //1887
;

void LodeZlib_DecompressSettings_init(LodeZlib_DecompressSettings *settings) //1900
;

unsigned int LodePNG_chunk_length(const unsigned char *chunk) //2060
{
	LodePNG_read32bitInt(); //2062
}

void LodePNG_chunk_type(char *type, const unsigned char *chunk) //2065
{
	{
		unsigned int i; //2067
	}
}

unsigned char LodePNG_chunk_critical(const unsigned char *chunk) //2079
;

unsigned char LodePNG_chunk_private(const unsigned char *chunk) //2084
;

unsigned char LodePNG_chunk_safetocopy(const unsigned char *chunk) //2089
;

unsigned char* LodePNG_chunk_data(unsigned char *chunk) //2094
;

const unsigned char* LodePNG_chunk_data_const(const unsigned char *chunk) //2099
;

unsigned int LodePNG_chunk_check_crc(const unsigned char *chunk) //2104
{
	{
		unsigned int length; //2106
		unsigned int CRC; //2107
		unsigned int checksum; //2108
	}
}

void LodePNG_chunk_generate_crc(unsigned char *chunk) //2113
{
	{
		unsigned int length; //2115
		unsigned int CRC; //2116
	}
}

unsigned char* LodePNG_chunk_next(unsigned char *chunk) //2120
{
	{
		unsigned int total_chunk_length; //2122
	}
}

const unsigned char* LodePNG_chunk_next_const(const unsigned char *chunk) //2126
{
	{
		unsigned int total_chunk_length; //2128
	}
}

unsigned int LodePNG_append_chunk(unsigned char **out, size_t *outlength, const unsigned char *chunk) //2132
{
	{
		unsigned int i; //2134
		unsigned int total_chunk_length; //2135
		unsigned char *chunk_start, unsigned char *new_buffer; //2136
		size_t new_length; //2137
	}
}

unsigned int LodePNG_create_chunk(unsigned char **out, size_t *outlength, unsigned int length, const char *type, const unsigned char *data) //2151
{
	{
		unsigned int i; //2153
		unsigned char *chunk, unsigned char *new_buffer; //2154
		size_t new_length; //2155
	}
}

void LodePNG_InfoColor_init(LodePNG_InfoColor *info) //2220
;

void LodePNG_InfoColor_clearPalette(LodePNG_InfoColor *info) //2235
;

void LodePNG_InfoColor_cleanup(LodePNG_InfoColor *info) //2230
;

unsigned int LodePNG_InfoColor_addPalette(LodePNG_InfoColor *info, unsigned char r, unsigned char g, unsigned char b, unsigned char a) //2241
{
	{
		unsigned char *data; //2243
		{
			size_t alloc_size; //2248
		}
	}
}

unsigned int LodePNG_InfoColor_getBpp(const LodePNG_InfoColor *info) //2261
{
	getBpp(); //2263
}

unsigned int filter(unsigned char *out, const unsigned char *in, unsigned int w, unsigned int h, const LodePNG_InfoColor *info) //4078
{
	{
		unsigned int bpp; //4091
		size_t linebytes; //4092
		size_t bytewidth; //4093
		const unsigned char *prevline; //4094
		unsigned int x, unsigned int y; //4095
		unsigned int heuristic; //4096
		unsigned int error; //4097
		{
			size_t outindex; //4109
			size_t inindex; //4110
			const unsigned int TYPE; //4111
		}
		{
			unsigned int sum[5]; //4119
			struct ucvector attempt[5]; //4120
			size_t smallest; //4121
			unsigned int type, unsigned int bestType; //4122
		}
	}
}

unsigned int preProcessScanlines(unsigned char **out, size_t *outsize, const unsigned char *in, const LodePNG_InfoPng *infoPng) //4278
{
	{
		unsigned int bpp; //4285
		unsigned int w; //4286
		unsigned int h; //4287
		unsigned int error; //4288
		{
			ucvector padded; //4300
		}
		{
			unsigned char *adam7; //4315
			{
				unsigned int passw[7], unsigned int passh[7]; //4320
				unsigned int filter_passstart[8], unsigned int padded_passstart[8], unsigned int passstart[8]; //4321
				unsigned int i; //4322
				{
					ucvector padded; //4340
				}
			}
		}
	}
}

unsigned int LodePNG_InfoColor_getChannels(const LodePNG_InfoColor *info) //2266
{
	getNumColorChannels(); //2268
}

unsigned int LodePNG_InfoColor_isGreyscaleType(const LodePNG_InfoColor *info) //2271
;

unsigned int LodePNG_InfoColor_isAlphaType(const LodePNG_InfoColor *info) //2276
;

unsigned int LodePNG_InfoColor_isPaletteType(const LodePNG_InfoColor *info) //2281
;

unsigned int LodePNG_InfoColor_hasPaletteAlpha(const LodePNG_InfoColor *info) //2286
{
	{
		size_t i; //2288
	}
}

unsigned int LodePNG_InfoColor_canHaveAlpha(const LodePNG_InfoColor *info) //2296
;

unsigned int LodePNG_InfoColor_equal(const LodePNG_InfoColor *info1, const LodePNG_InfoColor *info2) //2303
;

void LodePNG_UnknownChunks_init(LodePNG_UnknownChunks *chunks) //2311
{
	{
		unsigned int i; //2313
	}
}

void LodePNG_UnknownChunks_cleanup(LodePNG_UnknownChunks *chunks) //2318
{
	{
		unsigned int i; //2320
	}
}

unsigned int LodePNG_UnknownChunks_copy(LodePNG_UnknownChunks *dest, const LodePNG_UnknownChunks *src) //2324
{
	{
		unsigned int i; //2326
		{
			size_t j; //2332
		}
	}
}

void LodePNG_Text_init(LodePNG_Text *text) //2346
;

void LodePNG_Text_clear(LodePNG_Text *text) //2372
{
	{
		size_t i; //2374
	}
}

void LodePNG_Text_cleanup(LodePNG_Text *text) //2353
;

unsigned int LodePNG_Text_add(LodePNG_Text *text, const char *key, const char *str) //2384
{
	{
		char **new_keys; //2386
		char **new_strings; //2387
	}
}

unsigned int LodePNG_Text_copy(LodePNG_Text *dest, const LodePNG_Text *source) //2358
{
	{
		size_t i; //2360
		{
			unsigned int error; //2366
		}
	}
}

void LodePNG_IText_init(LodePNG_IText *text) //2410
;

void LodePNG_IText_clear(LodePNG_IText *text) //2440
{
	{
		size_t i; //2442
	}
}

void LodePNG_IText_cleanup(LodePNG_IText *text) //2419
;

unsigned int LodePNG_IText_add(LodePNG_IText *text, const char *key, const char *langtag, const char *transkey, const char *str) //2456
{
	{
		char **new_keys; //2458
		char **new_langtags; //2459
		char **new_transkeys; //2460
		char **new_strings; //2461
	}
}

unsigned int LodePNG_IText_copy(LodePNG_IText *dest, const LodePNG_IText *source) //2424
{
	{
		size_t i; //2426
		{
			unsigned int error; //2434
		}
	}
}

void LodePNG_InfoPng_init(LodePNG_InfoPng *info) //2494
;

void LodePNG_InfoPng_cleanup(LodePNG_InfoPng *info) //2516
;

void LodePNG_InfoPng_swap(LodePNG_InfoPng *a, LodePNG_InfoPng *b) //2548
{
	{
		LodePNG_InfoPng temp; //2550
	}
}

unsigned int LodePNG_InfoColor_copy(LodePNG_InfoColor *dest, const LodePNG_InfoColor *source) //2555
{
	{
		size_t i; //2557
	}
}

void LodePNG_InfoRaw_init(LodePNG_InfoRaw *info) //2566
;

void LodePNG_InfoRaw_cleanup(LodePNG_InfoRaw *info) //2571
;

unsigned int LodePNG_InfoRaw_copy(LodePNG_InfoRaw *dest, const LodePNG_InfoRaw *source) //2576
{
	{
		unsigned int error; //2578
	}
}

unsigned int LodePNG_convert(unsigned char *out, const unsigned char *in, LodePNG_InfoColor *infoOut, LodePNG_InfoColor *infoIn, unsigned int w, unsigned int h) //2593
{
	{
		const size_t numpixels; //2595
		const unsigned int OUT_BYTES; //2596
		const unsigned int OUT_ALPHA; //2597
		size_t i, size_t c, size_t bp; //2598
		{
			size_t i; //2603
			size_t size; //2604
		}
		{
			unsigned int value; //2805
		}
		{
			unsigned int value; //2720
		}
		{
			unsigned int value; //2737
		}
	}
}

void LodePNG_Decoder_inspect(LodePNG_Decoder *decoder, const unsigned char *in, size_t inlength) //2881
{
	LodePNG_read32bitInt(); //2911
	LodePNG_read32bitInt(); //2912
	{
		unsigned int CRC; //2921
		unsigned int checksum; //2922
	}
}

unsigned int postProcessScanlines(unsigned char *out, unsigned char *in, const LodePNG_InfoPng *infoPng) //3114
{
	{
		unsigned int bpp; //3122
		unsigned int w; //3123
		unsigned int h; //3124
		unsigned int error; //3125
		{
			unsigned int passw[7], unsigned int passh[7], unsigned int filter_passstart[8], unsigned int padded_passstart[8], unsigned int passstart[8]; //3140
			unsigned int i; //3141
		}
	}
}

void decodeGeneric(LodePNG_Decoder *decoder, unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize) //3163
{
	{
		unsigned char IEND; //3165
		const unsigned char *chunk; //3166
		size_t i; //3167
		ucvector idat; //3168
		unsigned int unknown; //3171
		unsigned int critical_pos; //3172
		{
			unsigned int chunkLength; //3187
			const unsigned char *data; //3188
			{
				size_t oldsize; //3211
			}
			{
				unsigned int pos; //3228
			}
			{
				char *key, char *str; //3334
				{
					unsigned int length, unsigned int string2_begin; //3338
				}
			}
			{
				unsigned int length, unsigned int string2_begin; //3386
				char *key; //3387
				ucvector decoded; //3388
			}
			{
				unsigned int length, unsigned int begin, unsigned int compressed; //3441
				char *key, char *langtag, char *transkey; //3442
				ucvector decoded; //3443
			}
			{
				LodePNG_UnknownChunks *unknown; //3592
			}
		}
		{
			ucvector scanlines; //3613
			{
				ucvector outv; //3623
			}
		}
	}
}

void LodePNG_Decoder_decode(LodePNG_Decoder *decoder, unsigned char **out, size_t *outsize, const unsigned char *in, size_t insize) //3636
{
	{
		unsigned char *data; //3656
	}
}

void LodePNG_DecodeSettings_init(LodePNG_DecodeSettings *settings) //3716
;

void LodePNG_Decoder_init(LodePNG_Decoder *decoder) //3729
;

void LodePNG_Decoder_cleanup(LodePNG_Decoder *decoder) //3737
;

unsigned int LodePNG_decode(unsigned char **out, unsigned int *w, unsigned int *h, const unsigned char *in, size_t insize, unsigned int colorType, unsigned int bitDepth) //3677
{
	{
		unsigned int error; //3679
		size_t dummy_size; //3680
		LodePNG_Decoder decoder; //3681
	}
}

unsigned int LodePNG_decode32(unsigned char **out, unsigned int *w, unsigned int *h, const unsigned char *in, size_t insize) //3693
;

void LodePNG_Decoder_copy(LodePNG_Decoder *dest, const LodePNG_Decoder *source) //3743
;

void LodePNG_Encoder_encode(LodePNG_Encoder *encoder, unsigned char **out, size_t *outsize, const unsigned char *image, unsigned int w, unsigned int h) //4443
{
	{
		LodePNG_InfoPng info; //4445
		ucvector outv; //4446
		unsigned char *data; //4447
		size_t datasize; //4448
		{
			unsigned char *converted; //4486
			size_t size; //4487
		}
		{
			size_t i; //4506
			{
				unsigned int alread_added_id_text; //4589
			}
		}
	}
}

void LodePNG_EncodeSettings_init(LodePNG_EncodeSettings *settings) //4681
;

void LodePNG_Encoder_init(LodePNG_Encoder *encoder) //4695
;

void LodePNG_Encoder_cleanup(LodePNG_Encoder *encoder) //4703
;

unsigned int LodePNG_encode(unsigned char **out, size_t *outsize, const unsigned char *image, unsigned int w, unsigned int h, unsigned int colorType, unsigned int bitDepth) //4639
{
	{
		unsigned int error; //4641
		LodePNG_Encoder encoder; //4642
	}
}

unsigned int LodePNG_encode32(unsigned char **out, size_t *outsize, const unsigned char *image, unsigned int w, unsigned int h) //4659
;

void LodePNG_Encoder_copy(LodePNG_Encoder *dest, const LodePNG_Encoder *source) //4709
;

unsigned int LodePNG_loadFile(unsigned char **out, size_t *outsize, const char *filename) //4731
{
	{
		FILE *file; //4733
		long int size; //4734
	}
}

unsigned int LodePNG_decode_file(unsigned char **out, unsigned int *w, unsigned int *h, const char *filename, unsigned int colorType, unsigned int bitDepth) //3699
{
	{
		unsigned char *buffer; //3701
		size_t buffersize; //3702
		unsigned int error; //3703
	}
}

unsigned int LodePNG_decode32_file(unsigned char **out, unsigned int *w, unsigned int *h, const char *filename) //3710
;

unsigned int LodePNG_saveFile(const unsigned char *buffer, size_t buffersize, const char *filename) //4759
{
	{
		FILE *file; //4761
	}
}

unsigned int LodePNG_encode_file(const char *filename, const unsigned char *image, unsigned int w, unsigned int h, unsigned int colorType, unsigned int bitDepth) //4665
{
	{
		unsigned char *buffer; //4667
		size_t buffersize; //4668
		unsigned int error; //4669
	}
}

unsigned int LodePNG_encode32_file(const char *filename, const unsigned char *image, unsigned int w, unsigned int h) //4675
;

const char* LodePNG_error_text(unsigned int code) //4777
;

extern const LodeZlib_DecompressSettings LodeZlib_defaultDecompressSettings; //1905

extern const LodeZlib_CompressSettings LodeZlib_defaultCompressSettings; //1894

const unsigned int LENGTHBASE[29]; //380

const unsigned int LENGTHEXTRA[29]; //382

const unsigned int DISTANCEBASE[30]; //384

const unsigned int DISTANCEEXTRA[30]; //386

const unsigned int CLCL[19]; //388

const size_t MAX_SUPPORTED_DEFLATE_LENGTH; //1133

const unsigned int HASH_NUM_VALUES; //1232

const unsigned int HASH_NUM_CHARACTERS; //1233

const unsigned int HASH_SHIFT; //1234

unsigned int Crc32_crc_table_computed; //1954

unsigned int Crc32_crc_table[256]; //1955

const unsigned int ADAM7_IX[7]; //2846

const unsigned int ADAM7_IY[7]; //2847

const unsigned int ADAM7_DX[7]; //2848

const unsigned int ADAM7_DY[7]; //2849

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/lodepng.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/mpeIO.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

struct stat; { //44
	long long unsigned int st_dev; //45
	unsigned char __pad0[4]; //46
	long unsigned int __st_ino; //48
	unsigned int st_mode; //49
	unsigned int st_nlink; //50
	long unsigned int st_uid; //52
	long unsigned int st_gid; //53
	long long unsigned int st_rdev; //55
	unsigned char __pad3[4]; //56
	long long int st_size; //58
	long unsigned int st_blksize; //59
	long long unsigned int st_blocks; //60
	long unsigned int st_atime; //62
	long unsigned int st_atime_nsec; //63
	long unsigned int st_mtime; //65
	long unsigned int st_mtime_nsec; //66
	long unsigned int st_ctime; //68
	long unsigned int st_ctime_nsec; //69
	long long unsigned int st_ino; //71
};

struct TGA_FACE_TEXTURE_TABLE; { //73
	const char *fname; //74
};

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //95
	MP_TEX_GLASSES_LENS = 0,
	MP_TEX_GLASSES_FRAME = 1,
	MP_TEX_GLASSES_SHADOW = 2,
	MP_TEX_GLASSES_REFRACT = 3,
	MP_TEX_GLASSES_LAST = 4,
} mpTexIDGlasses; //101

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //141
	MP_EXPR_MOUTH_UPPER = 0,
	MP_EXPR_MOUTH_SMALLER = 1,
	MP_EXPR_EYE_LARGER = 2,
	MP_EXPR_EYE_SMALLER = 3,
	MP_EXPR_BREATH = 4,
	MP_EXPR_VOICE_IE = 5,
	MP_EXPR_VOICE_UO = 6,
	MP_EXPR_VOICE_A = 7,
	MP_EXPR_EYE_CLOSING = 8,
	MP_EXPR_LAST = 9,
} mpExprIndex; //152

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //215
	MP_MODEL_TYPE = 0,
	MP_NUM_EXPR = 1,
} mpFaceParam; //218

typedef enum { //220
	MP_MODE_ANIME = 0,
	MP_MODE_PHOTO = 1,
	MP_MODE_LAST = 2,
} mpModeType; //224

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef enum { //255
	INTERPOLATE_OFF = 0,
	INTERPOLATE_NORMAL = 1,
} Interpolation; //258

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef enum { //9
	UNKNOWN = 0,
	OPEN = 1,
	CREATE = 2,
	EDIT = 3,
} openStatus; //14

typedef struct { //16
	unsigned int offset; //17
	unsigned int size; //18
} textureInfo; //19

typedef struct { //21
	int magic; //22
	unsigned int mpb_offset; //23
	unsigned int mpb_size; //24
	textureInfo tex[18]; //25
	textureInfo map[3]; //26
} faceHeader; //27

class faceBin { //29
private:
	faceHeader header; //49
	unsigned char *pmpb; //50
	unsigned char *pimg[18]; //51
	unsigned char *pmap[3]; //52
	FILE *fd; //53
	openStatus status; //54
	int isInfoSet; //55
	unsigned int info[32]; //56
	private: void faceBin() //32
	;
	private: void ~faceBin() //33
	;
	private: int createFile() //35
	;
	private: int openFile() //36
	;
	private: int editFile() //37
	;
	private: int closeFile() //38
	;
	private: int addImage() //39
	;
	private: int addMap() //40
	;
	private: int setMPB() //41
	;
	private: int getImage() //42
	;
	private: int getMap() //43
	;
	private: int getMPB() //44
	;
	private: int setInfo() //45
	;
	private: int getInfo() //46
	;
};

class mpeIO { //34
	private: void mpeIO() //37
	;
	private: void SetCommonPartsDir() //40
	;
	private: char* GetCommonPartsDir() //41
	;
	private: int CreateFaceMPB() //43
	;
	private: void CloseFace() //46
	;
	private: mpGlasses* CreateGlasses() //49
	;
	private: int CreateGlassesBinary() //50
	;
	private: void CloseGlasses() //51
	;
	private: int LoadFaceTextureDLL() //53
	;
	private: int LoadFaceMapDLL() //54
	;
	private: int LoadFaceTextureTGA() //56
	;
	private: int LoadFaceMapTGA() //57
	;
	private: int LoadFaceTextureBin() //58
	;
	private: int LoadFaceMapBin() //59
	;
	private: int ReadExprText() //70
	;
	2: int CreateFaceBinary() //74
	;
	2: int ReadGlassesCharaText() //78
	;
	2: int ReadFaceCharaText() //80
	;
	2: int LoadCharaPoints() //82
	;
	2: int LoadCharaEyeFine() //84
	;
	2: int LoadCharaSegs() //86
	;
	2: void CloseFaceTexture() //99
	;
	2: int LoadGlassesTexture() //100
	;
	2: void CloseGlassesTexture() //101
	;
	2: int LoadFaceImage() //103
	;
	2: int LoadLayerImage() //104
	;
	2: int LoadGlassImage() //105
	;
	2: FILE* OpenFile() //107
	;
	2: int Write() //108
	;
	3: int ReadExprBuff() //111
	;
	3: int SaveFaceAlphaTex() //112
	;
};

typedef struct mpTexture { //55
	GLuint textureName; //57
} mpTexture; //58

class mpeTexture { //5
	private: void mpeTexture() //8
	;
	private: int CreateTexture() //10
	;
	private: void CloseTexture() //11
	;
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

typedef struct { //19
	unsigned char Header[12]; //20
} TGAHeader; //21

typedef struct { //24
	unsigned char header[6]; //25
	GLuint bytesPerPixel; //26
	GLuint imageSize; //27
	GLuint temp; //28
	GLuint type; //29
	GLuint Height; //30
	GLuint Width; //31
	GLuint Bpp; //32
} TGA; //33

class TgaLoader { //37
private:
	TGAHeader tgaheader; //46
	TGA tga; //47
	unsigned char uTGAcompare[12]; //49
	unsigned char ugTGAcompare[12]; //50
	unsigned char cTGAcompare[12]; //51
	unsigned char gTGAcompare[12]; //52
	private: void TgaLoader() //40
	;
	private: int LoadImage() //43
	;
	2: int LoadUncompressedTGA() //55
	;
	2: int LoadGrayscaleTGA() //56
	;
	2: int LoadCompressedTGA() //57
	;
};

extern mpErrorCode errCode; //171

mpTexID texIDAnime[13]; //36

mpImageID imgIDAnime[3]; //52

mpTexID texIDPhoto[11]; //58

mpImageID imgIDPhoto[2]; //72

const struct TGA_FACE_TEXTURE_TABLE gTGATextureTableFace[18]; //77

const struct TGA_FACE_TEXTURE_TABLE gTGAMapTable[3]; //98

char commonParts[1024]; //187

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/mpeIO.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/mpeTexture.cpp

typedef struct mpTexture { //55
	GLuint textureName; //57
} mpTexture; //58

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

class mpeTexture { //5
	private: void mpeTexture() //8
	;
	private: int CreateTexture() //10
	;
	private: void CloseTexture() //11
	;
};

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/mpeTexture.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/util.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

struct stat; { //44
	long long unsigned int st_dev; //45
	unsigned char __pad0[4]; //46
	long unsigned int __st_ino; //48
	unsigned int st_mode; //49
	unsigned int st_nlink; //50
	long unsigned int st_uid; //52
	long unsigned int st_gid; //53
	long long unsigned int st_rdev; //55
	unsigned char __pad3[4]; //56
	long long int st_size; //58
	long unsigned int st_blksize; //59
	long long unsigned int st_blocks; //60
	long unsigned int st_atime; //62
	long unsigned int st_atime_nsec; //63
	long unsigned int st_mtime; //65
	long unsigned int st_mtime_nsec; //66
	long unsigned int st_ctime; //68
	long unsigned int st_ctime_nsec; //69
	long long unsigned int st_ino; //71
};

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //220
	MP_MODE_ANIME = 0,
	MP_MODE_PHOTO = 1,
	MP_MODE_LAST = 2,
} mpModeType; //224

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct { //271
	float xrot; //272
	float yrot; //273
	float zrot; //274
	float xtrans; //275
	float ytrans; //276
	float ztrans; //277
	float xreye; //278
	float yreye; //279
	float xleye; //280
	float yleye; //281
	float rblink; //282
	float lblink; //283
	float expr[32]; //284
	float camXtrans, float camYtrans; //285
	float camScale; //286
} mpAnimDataGain; //287

typedef struct { //295
	float fps; //296
	int nframe; //297
	int nexpr; //298
	mpAnimDataGain *buf; //299
} mpAnimData; //300

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

struct mpTexture; { //55
	GLuint textureName; //57
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

enum MPAnimControls; { //9
	NECK_XROT = 0,
	NECK_YROT = 1,
	NECK_ZROT = 2,
	XTRANS = 3,
	YTRANS = 4,
	ZTRANS = 5,
	REYEPOS_X = 6,
	REYEPOS_Y = 7,
	LEYEPOS_X = 8,
	LEYEPOS_Y = 9,
	RBLINK = 10,
	LBLINK = 11,
	EXPR00 = 12,
	EXPR01 = 13,
	EXPR02 = 14,
	EXPR03 = 15,
	EXPR04 = 16,
	EXPR05 = 17,
	EXPR06 = 18,
	EXPR07 = 19,
	EXPR08 = 20,
	EXPR09 = 21,
	EXPR10 = 22,
	EXPR11 = 23,
	EXPR12 = 24,
	EXPR13 = 25,
	EXPR14 = 26,
	EXPR15 = 27,
	EXPR16 = 28,
	EXPR17 = 29,
	EXPR18 = 30,
	EXPR19 = 31,
	EXPR20 = 32,
	EXPR21 = 33,
	EXPR22 = 34,
	EXPR23 = 35,
	EXPR24 = 36,
	EXPR25 = 37,
	EXPR26 = 38,
	EXPR27 = 39,
	EXPR28 = 40,
	EXPR29 = 41,
	EXPR30 = 42,
	EXPR_LAST = 43,
	LOOKAT_X_AP = 44,
	LOOKAT_Y_AP = 45,
	LOOKAT_Z_AP = 46,
	NUM_OF_ANIM_CONTROL = 47,
};

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //9
	UNKNOWN = 0,
	OPEN = 1,
	CREATE = 2,
	EDIT = 3,
} openStatus; //14

typedef struct { //16
	unsigned int offset; //17
	unsigned int size; //18
} textureInfo; //19

typedef struct { //21
	int magic; //22
	unsigned int mpb_offset; //23
	unsigned int mpb_size; //24
	textureInfo tex[18]; //25
	textureInfo map[3]; //26
} faceHeader; //27

class faceBin { //29
private:
	faceHeader header; //49
	unsigned char *pmpb; //50
	unsigned char *pimg[18]; //51
	unsigned char *pmap[3]; //52
	FILE *fd; //53
	openStatus status; //54
	int isInfoSet; //55
	unsigned int info[32]; //56
	private: void faceBin() //32
	;
	private: void ~faceBin() //33
	;
	private: int createFile() //35
	;
	private: int openFile() //36
	;
	private: int editFile() //37
	;
	private: int closeFile() //38
	;
	private: int addImage() //39
	;
	private: int addMap() //40
	;
	private: int setMPB() //41
	;
	private: int getImage() //42
	;
	private: int getMap() //43
	;
	private: int getMPB() //44
	;
	private: int setInfo() //45
	;
	private: int getInfo() //46
	;
};

class mpeIO { //34
	private: void mpeIO() //37
	;
	private: void SetCommonPartsDir() //40
	;
	private: char* GetCommonPartsDir() //41
	;
	private: int CreateFaceMPB() //43
	;
	private: void CloseFace() //46
	;
	private: mpGlasses* CreateGlasses() //49
	;
	private: int CreateGlassesBinary() //50
	;
	private: void CloseGlasses() //51
	;
	private: int LoadFaceTextureDLL() //53
	;
	private: int LoadFaceMapDLL() //54
	;
	private: int LoadFaceTextureTGA() //56
	;
	private: int LoadFaceMapTGA() //57
	;
	private: int LoadFaceTextureBin() //58
	;
	private: int LoadFaceMapBin() //59
	;
	private: int ReadExprText() //70
	;
	2: int CreateFaceBinary() //74
	;
	2: int ReadGlassesCharaText() //78
	;
	2: int ReadFaceCharaText() //80
	;
	2: int LoadCharaPoints() //82
	;
	2: int LoadCharaEyeFine() //84
	;
	2: int LoadCharaSegs() //86
	;
	2: void CloseFaceTexture() //99
	;
	2: int LoadGlassesTexture() //100
	;
	2: void CloseGlassesTexture() //101
	;
	2: int LoadFaceImage() //103
	;
	2: int LoadLayerImage() //104
	;
	2: int LoadGlassImage() //105
	;
	2: FILE* OpenFile() //107
	;
	2: int Write() //108
	;
	3: int ReadExprBuff() //111
	;
	3: int SaveFaceAlphaTex() //112
	;
};

class mpeTexture { //5
	private: void mpeTexture() //8
	;
	private: int CreateTexture() //10
	;
	private: void CloseTexture() //11
	;
};

mpFace* createFacefromBin( //54
				const char *name, //55
				struct mpTexture **tex, //56
				unsigned char **img, //57
				struct mpRenderingContext *rc) //58
{
	{
		class faceBin file; //60
		mpFace *face; //61
		unsigned char *data; //62
		unsigned int size; //63
		class mpeIO io; //64
		unsigned int d[32]; //90
	}
}

const char* FindExt(const char *path) //199
{
	{
		const char *ptr; //201
		const char *ext; //202
	}
}

int flipScanlineOrder(unsigned char *img, int w, int h, int channel) //338
{
	{
		unsigned char *buf; //340
		unsigned char *src, unsigned char *dst; //341
		{
			int i; //350
		}
	}
}

unsigned char* MergeRGBandAlpha( //322
				int width, int height, //322
				unsigned char *rgb, unsigned char *a) //323
{
	{
		unsigned char *rgba, unsigned char *p; //325
		{
			int i; //331
		}
	}
}

mpFace* mpuCreateFace( //105
				const char *name, //106
				struct mpTexture **tex, //107
				unsigned char **img, //108
				struct mpRenderingContext *rc) //109
{
	{
		mpFace *face; //111
		char *data; //112
		unsigned int size; //113
		class mpeIO io; //114
		struct stat buf; //115
		char fname[1024]; //116
		int dll_mode; //117
	}
}

void mpuCloseFace(mpFace *face, struct mpTexture **tex, unsigned char **img) //191
{
	{
		class mpeIO io; //193
	}
}

int mpuSetExprData(mpFace *face, const char *fname, char *expr_name) //210
{
	{
		const char *ext; //212
		int numExpr; //213
		ExprData *expr; //214
		CharaRect animRect; //215
		class mpeIO io; //216
		{
			int i; //237
		}
	}
}

int mpuInitAnim(mpAnimData *animData, char *filename) //245
{
	{
		FILE *fp; //247
		int curveCnt; //248
		float **fbuf; //249
		int i, int j; //250
		{
			mpAnimDataGain *gain; //284
		}
	}
}

void mpuCloseAnim(mpAnimData *animData) //317
;

int mpuChangeFaceTexture( //359
				mpFace *face, struct mpTexture **tex, //359
				int w, int h, unsigned char *img) //360
{
	{
		mpeImage newTex; //362
		unsigned char *rgba; //363
		class mpeTexture mpeTex; //364
	}
}

int mpuChangeTexture( //411
				mpFace *face, struct mpTexture **tex, mpTexID texid, //411
				int w, int h, unsigned char *img) //412
{
	{
		mpeImage newTex; //414
		class mpeTexture mpeTex; //415
	}
}

int mpuGetFaceTexture( //434
				const char *fname, //434
				int *width, int *height, unsigned char **rgb) //435
{
	{
		class faceBin file; //437
		unsigned int w, unsigned int h; //438
		unsigned char *img; //439
		unsigned char *rgba; //440
		{
			unsigned int i; //456
		}
	}
}

void mpuSetCommonPartsDir(const char *dir) //473
{
	{
		class mpeIO io; //475
	}
}

unsigned char* mpuReadRGBA(const char *file, int *width, int *height) //479
{
	{
		mpeImage image; //481
	}
}

unsigned char* mpuReadRGB(const char *file, int *width, int *height) //497
{
	{
		mpeImage image; //499
	}
}

unsigned char* mpuReadGray(const char *file, int *width, int *height) //515
{
	{
		mpeImage image; //517
	}
}

int mpuWritePNG32( //534
				const char *file, const unsigned char *image, //534
				int width, int height) //535
;

int mpuWritePNG8( //543
				const char *file, const unsigned char *image, //543
				int width, int height) //544
;

extern mpErrorCode errCode; //171

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/util.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilG.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //95
	MP_TEX_GLASSES_LENS = 0,
	MP_TEX_GLASSES_FRAME = 1,
	MP_TEX_GLASSES_SHADOW = 2,
	MP_TEX_GLASSES_REFRACT = 3,
	MP_TEX_GLASSES_LAST = 4,
} mpTexIDGlasses; //101

typedef enum { //103
	MP_TEX_GLASSESOPT_MIRROR = 0,
	MP_TEX_GLASSESOPT_COLOR = 1,
	MP_TEX_GLASSESOPT_LAST = 2,
} mpTexIDGlassesOpt; //107

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses { //53
	int oneLens; //54
	struct mpTexture *tex[4]; //55
	struct mpTexture *texopt[2]; //56
	CharaSegment seg[3]; //57
	struct mpMesh *shadowMesh; //58
	struct mpMesh *frameMesh; //59
	struct mpMesh *rLensMesh; //60
	struct mpMesh *lLensMesh; //61
	struct mpMesh *rLensMeshFine; //62
	struct mpMesh *lLensMeshFine; //63
	mpVector2 glassvertex[2][32]; //64
	struct _GlassFine glassfine[2][33]; //65
	int glassfinemax; //66
	mpVector2 eyeCenter; //68
	mpVector2 frameCenter; //69
	mpVector2 lensCenter[2]; //70
	mpRotation rot; //71
	mpColor lensColor; //72
	float mirrAlpha; //73
	float refractXScale, float refractYScale; //74
	mpVector3 vertBuf[153]; //78
} mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

struct mpTexture; { //55
	GLuint textureName; //57
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

struct _GlassFine; { //43
	float x; //44
	float ys; //45
	float ye; //46
};

typedef enum { //9
	UNKNOWN = 0,
	OPEN = 1,
	CREATE = 2,
	EDIT = 3,
} openStatus; //14

typedef struct { //16
	unsigned int offset; //17
	unsigned int size; //18
} textureInfo; //19

typedef struct { //21
	int magic; //22
	unsigned int mpb_offset; //23
	unsigned int mpb_size; //24
	textureInfo tex[18]; //25
	textureInfo map[3]; //26
} faceHeader; //27

class faceBin { //29
private:
	faceHeader header; //49
	unsigned char *pmpb; //50
	unsigned char *pimg[18]; //51
	unsigned char *pmap[3]; //52
	FILE *fd; //53
	openStatus status; //54
	int isInfoSet; //55
	unsigned int info[32]; //56
	private: void faceBin() //32
	;
	private: void ~faceBin() //33
	;
	private: int createFile() //35
	;
	private: int openFile() //36
	;
	private: int editFile() //37
	;
	private: int closeFile() //38
	;
	private: int addImage() //39
	;
	private: int addMap() //40
	;
	private: int setMPB() //41
	;
	private: int getImage() //42
	;
	private: int getMap() //43
	;
	private: int getMPB() //44
	;
	private: int setInfo() //45
	;
	private: int getInfo() //46
	;
};

class mpeIO { //34
	private: void mpeIO() //37
	;
	private: void SetCommonPartsDir() //40
	;
	private: char* GetCommonPartsDir() //41
	;
	private: int CreateFaceMPB() //43
	;
	private: void CloseFace() //46
	;
	private: mpGlasses* CreateGlasses() //49
	;
	private: int CreateGlassesBinary() //50
	;
	private: void CloseGlasses() //51
	;
	private: int LoadFaceTextureDLL() //53
	;
	private: int LoadFaceMapDLL() //54
	;
	private: int LoadFaceTextureTGA() //56
	;
	private: int LoadFaceMapTGA() //57
	;
	private: int LoadFaceTextureBin() //58
	;
	private: int LoadFaceMapBin() //59
	;
	private: int ReadExprText() //70
	;
	2: int CreateFaceBinary() //74
	;
	2: int ReadGlassesCharaText() //78
	;
	2: int ReadFaceCharaText() //80
	;
	2: int LoadCharaPoints() //82
	;
	2: int LoadCharaEyeFine() //84
	;
	2: int LoadCharaSegs() //86
	;
	2: void CloseFaceTexture() //99
	;
	2: int LoadGlassesTexture() //100
	;
	2: void CloseGlassesTexture() //101
	;
	2: int LoadFaceImage() //103
	;
	2: int LoadLayerImage() //104
	;
	2: int LoadGlassImage() //105
	;
	2: FILE* OpenFile() //107
	;
	2: int Write() //108
	;
	3: int ReadExprBuff() //111
	;
	3: int SaveFaceAlphaTex() //112
	;
};

class mpeTexture { //5
	private: void mpeTexture() //8
	;
	private: int CreateTexture() //10
	;
	private: void CloseTexture() //11
	;
};

int loadLensTexture( //55
				const char *dir, const char *name, mpGlasses *gls, //55
				struct mpTexture **tex, mpColor *col, mpTexIDGlasses id) //56
{
	{
		char fname[1024]; //58
		unsigned char *alpha, unsigned char *rgba; //59
		unsigned char *src, unsigned char *dst; //60
		int width, int height; //61
		mpeImage newTex; //62
		class mpeTexture mpeTex; //63
		{
			int i; //78
		}
	}
}

void mpuSetLensReflectColor(mpColor *col) //97
;

void mpuSetLensShadowColor(mpColor *col) //102
;

mpGlasses* mpuCreateGlasses( //116
				const char *name, //117
				struct mpTexture **tex, //118
				struct mpRenderingContext *rc) //119
{
	{
		FILE *fp; //121
		char fname[1024]; //122
		mpGlasses *gls; //123
		char *data; //124
		unsigned int size; //125
		CharaSegment seg[3]; //126
		struct _GlassFine fine[34]; //127
		int png_glasses; //128
		int one_lens; //129
		const char **tbl; //191
		class mpeIO io; //204
		{
			class mpeIO io; //135
			{
				int i; //144
				{
					int j; //146
				}
			}
		}
		{
			int fine_num, int n; //180
			{
				int i; //184
			}
		}
	}
}

void mpuSetGlassesOpt(mpGlasses *gls, const char *name, struct mpTexture **tex, struct mpRenderingContext *rc) //227
{
	{
		const char **tbl; //229
		{
			int i; //231
		}
	}
}

void mpuSetGlassesCol( //239
				mpGlasses *gls, const char *name, //239
				struct mpTexture **tex, struct mpRenderingContext *rc) //240
;

void mpuSetGlassesMirror( //247
				mpGlasses *gls, const char *name, //247
				struct mpTexture **tex, struct mpRenderingContext *rc) //248
;

void mpuSetGlassesRefract( //255
				mpGlasses *gls, struct mpTexture **tex, //255
				int w, int h, unsigned char *img, struct mpRenderingContext *rc) //256
{
	{
		mpeImage newTex; //258
		class mpeTexture mpeTex; //259
	}
}

void mpuCloseGlasses(mpGlasses *glasses, struct mpTexture **tex) //281
{
	{
		int i; //283
	}
}

void mpuCloseGlassesLens(mpGlasses *glasses, struct mpTexture **tex) //307
;

void mpuSetGlassesLens( //299
				mpGlasses *gls, const char *name, //299
				struct mpTexture **tex, struct mpRenderingContext *rc) //300
;

void mpuCloseGlassesOpt(mpGlasses *gls, struct mpTexture **tex) //322
{
	{
		int i; //324
	}
}

void mpuCloseGlassesCol(mpGlasses *gls, struct mpTexture **tex) //335
;

void mpuCloseGlassesMirror(mpGlasses *gls, struct mpTexture **tex) //345
;

void mpuCloseGlassesRefract(mpGlasses *gls, struct mpTexture **tex) //354
;

extern mpErrorCode errCode; //171

const char *gTextureTableGlasses[3]; //30

const char *gTextureTableGlassesPNG[3]; //34

const const char *gTextureTableGlassesOpt[2]; //44

mpColor lensReflect; //48

mpColor lensShadow; //49

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilG.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilHH.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

struct stat; { //44
	long long unsigned int st_dev; //45
	unsigned char __pad0[4]; //46
	long unsigned int __st_ino; //48
	unsigned int st_mode; //49
	unsigned int st_nlink; //50
	long unsigned int st_uid; //52
	long unsigned int st_gid; //53
	long long unsigned int st_rdev; //55
	unsigned char __pad3[4]; //56
	long long int st_size; //58
	long unsigned int st_blksize; //59
	long long unsigned int st_blocks; //60
	long unsigned int st_atime; //62
	long unsigned int st_atime_nsec; //63
	long unsigned int st_mtime; //65
	long unsigned int st_mtime_nsec; //66
	long unsigned int st_ctime; //68
	long unsigned int st_ctime_nsec; //69
	long long unsigned int st_ino; //71
};

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //53
	MP_IMG_HIGE_ZMAP = 0,
	MP_IMG_HIGE_TRSFACT = 1,
	MP_IMG_HIGE_EXPRMAP = 2,
	MP_IMG_HIGE_LAST = 3,
} mpImageHigeID; //58

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //109
	MP_TEX_HAIR_RGB = 0,
	MP_TEX_HAIR_BGK = 1,
	MP_TEX_HAIR_FR = 2,
	MP_TEX_HAIR_LAST = 3,
} mpTexIDHair; //114

typedef enum { //116
	MP_TEX_HIGE_RGB = 0,
	MP_TEX_HIGE_LAST = 1,
} mpTexIDHige; //119

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct mpeImage { //77
	unsigned char *imageData; //79
	int bpp; //80
	int width; //81
	int height; //82
	GLuint type; //83
} mpeImage; //84

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

struct mpTexture; 

struct mpMesh; 

class mpeTexture { //5
	private: void mpeTexture() //8
	;
	private: int CreateTexture() //10
	;
	private: void CloseTexture() //11
	;
};

typedef enum { //9
	UNKNOWN = 0,
	OPEN = 1,
	CREATE = 2,
	EDIT = 3,
} openStatus; //14

typedef struct { //16
	unsigned int offset; //17
	unsigned int size; //18
} textureInfo; //19

typedef struct { //21
	int magic; //22
	unsigned int mpb_offset; //23
	unsigned int mpb_size; //24
	textureInfo tex[18]; //25
	textureInfo map[3]; //26
} faceHeader; //27

class faceBin { //29
private:
	faceHeader header; //49
	unsigned char *pmpb; //50
	unsigned char *pimg[18]; //51
	unsigned char *pmap[3]; //52
	FILE *fd; //53
	openStatus status; //54
	int isInfoSet; //55
	unsigned int info[32]; //56
	private: void faceBin() //32
	;
	private: void ~faceBin() //33
	;
	private: int createFile() //35
	;
	private: int openFile() //36
	;
	private: int editFile() //37
	;
	private: int closeFile() //38
	;
	private: int addImage() //39
	;
	private: int addMap() //40
	;
	private: int setMPB() //41
	;
	private: int getImage() //42
	;
	private: int getMap() //43
	;
	private: int getMPB() //44
	;
	private: int setInfo() //45
	;
	private: int getInfo() //46
	;
};

void searchHairEdge( //81
				unsigned char *rgba, int width, int height, //81
				int len, int *lheight, int *rheight, int *left, int *right) //82
{
	{
		int x, int y, int i; //84
		int hair_top; //85
		{
			unsigned char a; //90
		}
		{
			unsigned char a; //101
		}
	}
}

mpHair* CreateHairPNG( //261
				const char *name, //262
				struct mpTexture **tex, //263
				struct mpRenderingContext *rc) //264
{
	{
		char fname[1024]; //266
		FILE *fp; //267
		float *mesh_p, float *tfp; //268
		float edpr[12]; //269
		int zwidth, int zheight; //270
		unsigned char *zdata; //271
		mpHair *hair; //272
		unsigned char *rgba; //337
		int width, int height; //338
		mpeImage newTex; //339
		class mpeTexture mpeTex; //340
		{
			int l; //295
			{
				float *dst_top; //296
				float *src_top; //297
				{
					int i, int j; //299
					{
						float *dst; //300
						float *src; //301
					}
				}
			}
		}
		{
			int i; //334
		}
	}
}

struct mpRenderingContext; 

mpHair* CreateHairTGA( //196
				const char *name, //197
				struct mpTexture **tex, //198
				struct mpRenderingContext *rc) //199
{
	{
		int i, int zwidth, int zheight; //201
		mpHair *hair; //202
		char *data; //203
		unsigned char *zdata; //204
		unsigned int size; //205
		char fname[1024]; //206
		unsigned char *rgba; //229
		int width, int height; //230
		mpeImage newTex; //231
		class mpeTexture mpeTex; //232
	}
}

int addHairBG(unsigned char *rgba, int width, int height) //110
{
	{
		int *left, int *right; //112
		int lheight, int rheight; //113
		int i, int j, int y; //114
		int len; //115
		{
			int x; //129
			{
				unsigned char *r, unsigned char *g, unsigned char *b, unsigned char a; //130
			}
		}
		{
			unsigned char a_val; //144
			{
				int x; //145
				{
					unsigned char *r, unsigned char *g, unsigned char *b, unsigned char a; //146
				}
			}
		}
		{
			int x; //162
			{
				unsigned char *r, unsigned char *g, unsigned char *b, unsigned char a; //163
			}
		}
		{
			unsigned char a_val; //177
			{
				int x; //179
				{
					unsigned char *r, unsigned char *g, unsigned char *b, unsigned char a; //180
				}
			}
		}
	}
}

int read_integer_mesh(float *buf, int len, FILE *fp) //62
{
	{
		short int s; //64
		{
			int i; //66
		}
	}
}

void mpuSetHairMode(int background, mpColor *col) //378
;

void mpuSetHairBGArea(float bgarea) //384
;

void mpuCloseHair(mpHair *hair, struct mpTexture **tex) //463
{
	{
		int i; //465
	}
}

mpHair* mpuCreateHair( //363
				const char *name, //364
				struct mpTexture **tex, //365
				struct mpRenderingContext *rc) //366
{
	{
		struct stat buf; //368
		char fname[1024]; //369
	}
}

void mpuCloseHige(mpHige *hige, struct mpTexture **tex) //478
{
	{
		int i; //480
	}
}

mpHige* mpuCreateHige( //394
				const char *name, //395
				struct mpTexture **tex, //396
				struct mpRenderingContext *rc) //397
{
	{
		float data[12]; //399
		FILE *fd; //400
		int i, int zwidth, int zheight; //401
		mpHige *hige; //402
		unsigned char *zdata; //403
		char fname[1024]; //404
		char szmapFilePath[1024]; //405
		int png_hige; //406
		const char **tbl; //441
		const char **map; //452
		{
			int num; //413
			{
				int i; //416
			}
		}
	}
}

int mpuChangeHairTexture( //494
				mpHair *hair, struct mpTexture **tex, //494
				int w, int h, unsigned char *rgba) //495
{
	{
		mpeImage newTex; //497
		class mpeTexture mpeTex; //498
		unsigned char *tmp; //499
	}
}

int mpuSetAnimHair(mpFace *face, struct mpTexture **tex, const char *name) //528
{
	{
		class faceBin file; //530
		unsigned char *img, unsigned char *map; //531
		unsigned int imgw, unsigned int imgh, unsigned int mapw, unsigned int maph; //532
		mpeImage image; //533
		class mpeTexture mpeTex; //534
	}
}

void mpuUnsetAnimHair(mpFace *face, struct mpTexture **tex) //561
;

extern mpErrorCode errCode; //171

const mpTexIDHige texIDHige[1]; //40

const char *gTextureTableHige[1]; //44

const char *gTextureTableHigePNG[1]; //48

const char *gMapTableHige[3]; //52

const char *gMapTableHigePNG[3]; //56

int hairBGMode; //73

mpColor hairBGColor; //74

float hairBGArea; //75

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilHH.cpp// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilV.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct { //26
	short unsigned int wChannels; //27
	unsigned int dwSamplesPerSec; //28
	unsigned int dwAvgBytesPerSec; //29
	short unsigned int wBlockAlign; //30
	short unsigned int wBitsPerSample; //31
	unsigned int data_size; //32
} WavHeader; //33

void putenv3(WavHeader *wh, int *sample, int *size) //174
{
	{
		int srate, int len; //176
		unsigned int pos; //177
	}
}

int calfilt(unsigned char *buf, int size, int n) //302
{
	{
		int i, int k, int s, int sum, int f; //304
	}
}

int calpuls(int *buf, int size, int n) //341
{
	{
		int i, int sum, int f; //343
	}
}

int rand() //117
;

void mkenv(WavHeader *wh, unsigned char *wav) //362
{
	{
		int i, int j, int k, int n, int m, int d, int p, int size; //364
		float g1, float g2, float g3, float def0, float def1, float def2, float f; //365
		int sum, int len; //366
		int st; //380
		{
			float gm; //404
		}
		{
			int j; //415
		}
	}
}

void mkwav8bit(unsigned char *wav, int size) //209
{
	{
		int i, int d; //211
	}
}

void mkwav8m(unsigned char *wav, int size) //226
{
	{
		int i, int d; //228
	}
}

void mkwavls(unsigned char *wav, int size) //241
{
	{
		int d; //243
		{
			int i; //245
		}
	}
}

int get_env_buff(int org_size) //256
{
	{
		int i, int size; //258
	}
}

void free_all_buf() //293
;

unsigned char* ReadWav(char *fn, WavHeader *wh) //95
{
	{
		FILE *fp; //97
		int res32, int chunck_bytes; //98
		short int res16; //99
		unsigned char *wav; //100
	}
}

unsigned char* createENV(WavHeader *wh, unsigned char *wav, int *sample, int *size) //517
{
	mkwavls(); //542
	get_env_buff(); //549
	mkenv(); //551
	putenv3(); //553
	free_all_buf(); //555
	mkwav8m(); //529
	mkwav8bit(); //521
}

int mpuCreateVoice(mpVoice *voice, char *filename) //31
{
	{
		WavHeader wav_header; //33
		unsigned char *wav, unsigned char *envd; //34
		int sample, int size; //35
	}
}

int mpuCreateVoiceOnMemory(mpVoice *voice, WavHeader *wav_header, unsigned char *wav) //55
{
	{
		unsigned char *envd; //57
		int sample, int size; //58
	}
}

void mpuCloseVoice(mpVoice *voice) //73
;

const int lpfsel; //81

const int gamsel; //82

const int evsmpl; //83

int *env; //85

unsigned char *env2; //86

unsigned char *env3; //87

int *env4; //88

// /Users/iwasaki/src/common/lib.android/MPUtil\jni/../../../lib/MPUtil/utilV.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/animeng.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //141
	MP_EXPR_MOUTH_UPPER = 0,
	MP_EXPR_MOUTH_SMALLER = 1,
	MP_EXPR_EYE_LARGER = 2,
	MP_EXPR_EYE_SMALLER = 3,
	MP_EXPR_BREATH = 4,
	MP_EXPR_VOICE_IE = 5,
	MP_EXPR_VOICE_UO = 6,
	MP_EXPR_VOICE_A = 7,
	MP_EXPR_EYE_CLOSING = 8,
	MP_EXPR_LAST = 9,
} mpExprIndex; //152

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //172
	MP_SIDE_RIGHT = 0,
	MP_SIDE_LEFT = 1,
} mpSide; //175

typedef enum { //183
	MP_NECK_X_ENABLE = 0,
	MP_NECK_Y_ENABLE = 1,
	MP_NECK_Z_ENABLE = 2,
	MP_NECK_X_DURATION_FACTOR = 3,
	MP_NECK_Y_DURATION_FACTOR = 4,
	MP_NECK_Z_DURATION_FACTOR = 5,
	MP_NECK_X_MAX_ROT = 6,
	MP_NECK_Y_MAX_ROT = 7,
	MP_NECK_Z_MAX_ROT = 8,
	MP_BLINK_ENABLE = 9,
	MP_BLINK_DURATION_FACTOR = 10,
	MP_BLINK_FREQS = 11,
	MP_BLINK_GAIN_FACTOR = 12,
	MP_PUPIL_ENABLE = 13,
	MP_PUPIL_DURATION_FACTOR = 14,
	MP_PUPIL_X_MAX = 15,
	MP_PUPIL_Y_MAX = 16,
	MP_EXPR_ENABLE = 17,
	MP_EXPR_DURATION_FACTOR = 18,
	MP_EXPR_BREATH_DURATION_FACTOR = 19,
	MP_EXPR_GAIN_FACTORS = 20,
	MP_BREATH_ENABLE = 21,
} mpUAnimParam; //206

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct { //271
	float xrot; //272
	float yrot; //273
	float zrot; //274
	float xtrans; //275
	float ytrans; //276
	float ztrans; //277
	float xreye; //278
	float yreye; //279
	float xleye; //280
	float yleye; //281
	float rblink; //282
	float lblink; //283
	float expr[32]; //284
	float camXtrans, float camYtrans; //285
	float camScale; //286
} mpAnimDataGain; //287

typedef struct { //295
	float fps; //296
	int nframe; //297
	int nexpr; //298
	mpAnimDataGain *buf; //299
} mpAnimData; //300

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

struct mpTexture; 

struct mpMesh; 

int calcBlinkTime(mpBlinkType type, int time) //1918
;

void setCtrlPupil( //2113
				PupilUAnim *uPupil, LookAtAnim *lookAt, //2113
				CtrlData *ctrl) //2114
;

void setCtrlNeckRot( //2199
				NeckUAnim *uNeckX, NeckUAnim *uNeckY, //2199
				NeckUAnim *uNeckZ, LeanAnim *lean, //2200
				LookAtAnim *lookAt, CtrlData *ctrl) //2201
{
	{
		float w; //2203
	}
}

void setCtrlExpress( //2247
				ExprUAnim *uExpr, ExpressAnim *express, //2247
				LookAtAnim *lookAt, SpeakAnim *speak, //2248
				CloseEyeAnim *closeEye, int numExpr, CtrlData *ctrl) //2249
{
	{
		int i; //2251
		float w; //2252
	}
}

void setCtrlEyeClose(CloseEyeAnim *closeEye, CtrlData *ctrl) //2305
;

int calcLipSync(LipSync *lipSync, int time) //1741
{
	{
		int i; //1743
		int start, int pos; //1744
		float gain, float g, float fact; //1745
		mpVoice *voice; //1746
	}
}

int rand() //117
;

int updateShift(ShiftAnim *anim, long int currTime) //1466
{
	{
		float fact; //1468
	}
}

int updateLookAt( //1511
				LookAtAnim *anim, long int currTime, float *minBlink, //1511
				int *looking) //1512
{
	{
		float gain, float fact; //1514
	}
}

int updateBlinkU( //1154
				BlinkUAnim *anim, long int currTime, long int prevTime, //1154
				float minBlink, int *started, int *syncPupil, //1155
				float *halfBlinkY, float gain) //1156
{
	{
		int dur; //1158
		int denom, int numer1, int numer2; //1159
		int type; //1160
	}
}

int updateBlink( //1862
				BlinkAnim *anim, long int currTime, long int prevTime, //1862
				float minBlink, int *started, int *syncPupil, //1863
				float *halfBlinkY) //1864
;

int updateExprU(ExprUAnim *anim, long int currTime, long int prevTime, float ugain) //1358
{
	{
		int i; //1360
		float fact; //1361
		float *gain; //1362
		float *timer; //1363
		float *gainFact; //1364
		float baseFact[7]; //1365
	}
}

int updateLean(LeanAnim *anim, long int currTime) //1421
{
	{
		float fact; //1423
	}
}

int updateExpress(ExpressAnim *anim, int numExpr, long int currTime) //1594
{
	{
		int i; //1596
		float fact; //1597
		float *gain; //1598
		float *startGain; //1599
		float *endGain; //1600
	}
}

int updateCloseEye(CloseEyeAnim *anim, long int currTime) //1817
{
	{
		float fact; //1820
	}
}

void setCtrlBlink(BlinkUAnim *uBlink, BlinkAnim *cBlink, CtrlData *ctrl) //2145
{
	{
		int i; //2147
		float *gain; //2148
		float *alpha; //2149
	}
}

void setCtrlNeckTrans(ShiftAnim *anim, CtrlData *ctrl) //2168
;

void makeBlinkTable() //1990
{
	{
		int i; //1992
		mpBlinkType type; //1993
		int a, int b, int c, int d, int e, int f; //1994
		float w, float g1, float g2; //1995
	}
}

void calcBlinkGain( //1954
				BlinkData *blink, float minGain, float gainFactor, //1954
				long int currTime, long int prevTime, int startTime) //1955
{
	{
		int i; //1957
		float gain, float alpha; //1958
		mpBlinkType type; //1959
		int ctime; //1960
		int ptime; //1961
	}
}

int updateSpeak(SpeakAnim *anim, long int currTime, int *area) //1644
{
	{
		int p1, int p2, int p3; //1646
		int ct; //1647
		int result; //1648
		float fact; //1649
		{
			float lip_gain; //1680
		}
	}
}

int updateNeckU(NeckUAnim *anim, long int currTime, float gain) //1094
{
	{
		float tmp; //1096
		int dur; //1097
	}
}

int updatePupilU( //1246
				PupilUAnim *anim, long int currTime, int blinkStarted, //1246
				int syncBlink, int looking, float halfBlinkY, //1247
				int *started, float gain) //1248
{
	{
		float maxMove; //1250
	}
}

int mpAnimate(mpFace *face, long unsigned int currentTime) //245
{
	{
		int looking; //247
		float minBlink; //248
		int ubStart; //249
		int ubSync; //250
		float ubBlinkY; //251
		int cbStart; //252
		int cbSync; //253
		float cbBlinkY; //254
		int pStart; //255
		int area; //256
		float gain; //257
		AnimData *anim; //259
		ShiftAnim *shift; //260
		CtrlData *ctrl; //261
		NeckUAnim *uNeckX; //262
		NeckUAnim *uNeckY; //263
		NeckUAnim *uNeckZ; //264
		BlinkUAnim *uBlink; //265
		PupilUAnim *uPupil; //266
		ExprUAnim *uExpr; //267
		LookAtAnim *lookAt; //268
		SpeakAnim *speak; //269
		CloseEyeAnim *closeEye; //270
		LeanAnim *lean; //271
		ExpressAnim *express; //272
		BlinkAnim *cBlink; //273
	}
}

void mpSetUnconciousGain( //387
				mpFace *face, long unsigned int currentTime, //387
				long unsigned int duration, float gain) //388
;

void mpLean( //415
				mpFace *face, int duration, const mpRotation *rotation, //415
				float weight) //416
{
	{
		LeanAnim *anim; //418
	}
}

void mpShift( //460
				mpFace *face, int duration, const mpTranslation *translation, //460
				float weight) //461
{
	{
		ShiftAnim *anim; //463
	}
}

void mpLookAt( //512
				mpFace *face, int duration, const mpVector2 *position, //512
				float weight) //513
{
	{
		float pupilX, float pupilY, float rotX, float rotY; //515
		LookAtAnim *anim; //516
	}
}

void mpExpress(mpFace *face, int duration, const float *gain, float weight) //623
{
	{
		int i; //625
		float g[128]; //626
		ExpressAnim *anim; //627
	}
}

void mpSpeak(mpFace *face, mpVoice *voice, int startMargin, int endMargin) //707
{
	{
		SpeakAnim *anim; //709
	}
}

void mpSpeakStop(mpFace *face) //759
{
	{
		SpeakAnim *anim; //761
		mpVoice *voice; //762
	}
}

void mpSpeakResume(mpFace *face) //779
{
	{
		SpeakAnim *anim; //781
		mpVoice *voice; //782
	}
}

void mpCloseEye(mpFace *face, int duration, float close) //811
{
	{
		CloseEyeAnim *anim; //813
	}
}

void mpBlink(mpFace *face, mpBlinkType type, float gain) //858
{
	{
		BlinkAnim *anim; //860
	}
}

int mpAnimateData( //899
				mpFace *face, long unsigned int startTime, long unsigned int currentTime, mpAnimData *animData, //899
				float *camXtrans, float *camYtrans, float *camScale) //900
{
	{
		float fps; //902
		float ftime; //903
		int ind0, int ind1; //904
		float frac0, float frac1; //905
		long unsigned int elapsed; //906
		mpRotation rot; //907
		mpTranslation trans; //908
		mpAnimDataGain *gain; //909
		mpVector2 reye, mpVector2 leye; //910
		float rblink[4], float lblink[4]; //911
		float *expr; //912
		int i; //913
	}
}

int initAnim(AnimData *anim) //999
{
	{
		int i; //1001
	}
}

void initAnimEng() //1077
{
	makeBlinkTable(); //1079
}

void mpGetSpeakGain(mpFace *face, const long int time, float **gain) //1720
{
	{
		int area; //1722
		float *lipgain; //1724
	}
}

void mpSetAnimParamf(mpFace *face, mpUAnimParam type, float value) //2604
;

float mpGetAnimParamf(mpFace *face, mpUAnimParam type) //2680
{
	{
		float value; //2682
	}
}

void mpSetAnimParamfv(mpFace *face, mpUAnimParam type, const float *value) //2740
{
	{
		int i; //2742
		float val; //2743
	}
}

void mpGetAnimParamfv(mpFace *face, mpUAnimParam type, float *value) //2774
{
	{
		int i; //2776
	}
}

void mpSetAnimParami(mpFace *face, mpUAnimParam type, int value) //2798
;

int mpGetAnimParami(mpFace *face, mpUAnimParam type) //2836
{
	{
		int value; //2838
	}
}

void mpSetAnimParamiv(mpFace *face, mpUAnimParam type, const int *value) //2878
{
	{
		int i, int sum; //2883
	}
}

void mpGetAnimParamiv(mpFace *face, mpUAnimParam type, int *value) //2918
{
	{
		int i; //2920
	}
}

extern mpErrorCode errCode; //171

float blinkTable[3][1024]; //204

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/animeng.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/common.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //172
	MP_SIDE_RIGHT = 0,
	MP_SIDE_LEFT = 1,
} mpSide; //175

typedef enum { //215
	MP_MODEL_TYPE = 0,
	MP_NUM_EXPR = 1,
} mpFaceParam; //218

typedef enum { //232
	MP_LAYER_DRAW = 0,
	MP_LAYER_BLEND = 1,
} mpLayerParam; //235

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

struct mpTexture; 

struct mpMesh; 

void mpSetMemoryFunction( //60
				void(*)() *alloc_func, //61
				void(*)() *free_func) //62
;

void* hook_malloc(unsigned int size) //70
{
	{
		void *mem; //72
	}
}

void* hook_calloc(unsigned int num, unsigned int size) //89
{
	{
		void *mem; //91
	}
}

void hook_free(void *memblock) //115
;

mpErrorCode mpGetError() //138
{
	{
		mpErrorCode err; //140
	}
}

int mpGetFaceParami(mpFace *face, mpFaceParam type) //176
{
	{
		int value; //178
	}
}

int mpGetLayerParami(mpFace *face, int id, mpLayerParam type, int *value) //210
{
	{
		LayerData *layer; //212
	}
}

int mpSetLayerParami(mpFace *face, int id, mpLayerParam type, int value) //241
{
	{
		LayerData *layer; //243
	}
}

int mpGetLayerParamf(mpFace *face, int id, mpLayerParam type, float *value) //272
{
	{
		LayerData *layer; //274
	}
}

int mpSetLayerParamf(mpFace *face, int id, mpLayerParam type, float value) //303
{
	{
		LayerData *layer; //305
	}
}

float calTime(float time) //334
;

float calTime2(float time) //358
;

void _mpGetEyeFeaturePoints(mpFace *face, mpSide side, CharaSegment *cs) //367
;

void _mpGetFeaturePoints(mpFace *face, int idx, CharaSegment *cs) //375
;

int _mpGetMouthFeaturePoints(mpFace *face, CharaSegment *cs) //380
;

void _mpGetNoseFeaturePoints(mpFace *face, CharaSegment *cs) //386
;

void _mpGetExprData(mpFace *face, int idx, ExprData *exp) //391
;

void _mpSetExprData(mpFace *face, int idx, ExprData *exp) //396
;

void _mpGetAnimRect(mpFace *face, CharaRect *animRect) //403
;

void _mpSetAnimRect(mpFace *face, CharaRect *animRect) //408
{
	{
		int i; //411
	}
}

extern struct mpTexture *g_mpCurrentTexture; //28

extern mpErrorCode errCode; //30

struct mpRenderingContext; 

extern struct mpRenderingContext *_mprc; //22

void(*)() *hook_alloc_func; //40

void(*)() *hook_free_func; //46

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/common.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/direct.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //172
	MP_SIDE_RIGHT = 0,
	MP_SIDE_LEFT = 1,
} mpSide; //175

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

struct mpTexture; 

struct mpMesh; 

void mpGetEyePos(mpFace *face, mpSide side, mpVector2 *position) //26
{
	{
		CharaData *chara; //27
	}
}

void mpGetPupilPos(mpFace *face, mpSide side, mpVector2 *position) //50
{
	{
		CtrlData *ctrl; //52
	}
}

void mpSetPupilPos(mpFace *face, mpSide side, const mpVector2 *position) //76
{
	{
		CtrlData *ctrl; //78
	}
}

void mpGetNeckRot(mpFace *face, mpRotation *rotation) //99
{
	{
		CtrlData *ctrl; //101
	}
}

void mpSetNeckRot(mpFace *face, const mpRotation *rotation) //116
{
	{
		CtrlData *ctrl; //118
	}
}

void mpGetNeckTransl(mpFace *face, mpTranslation *translation) //133
{
	{
		CtrlData *ctrl; //135
	}
}

void mpSetNeckTransl(mpFace *face, const mpTranslation *translation) //150
{
	{
		CtrlData *ctrl; //152
	}
}

void mpGetBlink(mpFace *face, mpSide side, float *gain, float *alpha) //177
{
	{
		int i; //179
		float *blinkg, float *blinka; //180
		CtrlData *ctrl; //181
	}
}

void mpSetBlink( //222
				mpFace *face, mpSide side, const float *gain, //222
				const float *alpha) //223
{
	{
		int i; //225
		float *blinkg, float *blinka; //226
		CtrlData *ctrl; //227
	}
}

float mpGetEyeClose(mpFace *face, mpSide side) //276
{
	{
		CtrlData *ctrl; //278
	}
}

void mpSetEyeClose(mpFace *face, mpSide side, float close) //303
{
	{
		CtrlData *ctrl; //305
	}
}

void mpGetExprGain(mpFace *face, float *gain) //331
{
	{
		int i; //333
		CtrlData *ctrl; //334
	}
}

void mpSetExprGain(mpFace *face, const float *gain) //351
{
	{
		int i; //353
		CtrlData *ctrl; //354
	}
}

int mpGetMapSize(mpFace *face) //369
;

int mpGetZMapValue(mpFace *face, int p, int q) //373
;

struct mpTexture* mpGetTexture(mpFace *face, int id) //382
;

extern mpErrorCode errCode; //171

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/direct.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/gl_draw_layer.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

struct mpTexture; { //55
	GLuint textureName; //57
};

typedef struct mpMesh { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
} mpMesh; //76

void updateDrawColorBuffer(struct mpMesh *mesh) //237
{
	{
		int numVertices; //243
		int bSetMeshColorExceptForWhite; //244
		{
			int i; //256
		}
	}
}

void mpDeleteMesh(struct mpRenderingContext *rc, struct mpMesh *mesh) //193
;

struct mpMesh* mpCreateMeshDiam( //56
				struct mpRenderingContext *rc, //57
				int nWidthDivision, int nHeightDivision, //58
				int nWidthCenter, int nHeightCenter, int bVertexColor) //59
{
	{
		struct mpMesh *mesh; //61
		int numVertices; //62
		int numTriangles; //63
		mpVector3 *pos; //64
		mpVector2 *tex; //65
		short unsigned int *tidx; //66
		int x, int y, int i; //67
		{
			float yy; //94
			{
				float xx; //96
			}
		}
		{
			float yy; //113
			{
				float xx; //115
			}
		}
		{
			short unsigned int yidx0, short unsigned int yidx1; //151
		}
	}
}

struct mpMesh* mpCreateMesh( //49
				struct mpRenderingContext *rc, //50
				int nWidthDivision, int nHeightDivision, int bVertexColor) //51
;

void mpSetMeshPosition( //209
				struct mpRenderingContext *rc, //210
				struct mpMesh *mesh, //211
				const mpVector3 *position) //212
{
	{
		int numVertices; //214
	}
}

void mpSetMeshTexAddress( //223
				struct mpRenderingContext *rc, //224
				struct mpMesh *mesh, //225
				const mpVector2 *texaddress) //226
{
	{
		int numVertices; //228
	}
}

void mpSetMeshVertexColor( //275
				struct mpRenderingContext *rc, //276
				struct mpMesh *mesh, //277
				const mpColor *color) //278
{
	{
		int numVertices; //280
	}
}

void mpSetMeshColor( //304
				struct mpRenderingContext *rc, //305
				struct mpMesh *mesh, //306
				const mpColor *color) //307
;

void mpDrawMesh( //325
				struct mpRenderingContext *rc, //326
				struct mpMesh *mesh) //327
;

void mpDrawColorFan( //378
				struct mpRenderingContext *rc, //378
				int nv, mpVector3 *pos, mpColor *col) //379
;

void mpDrawMeshWithDepth( //405
				struct mpRenderingContext *rc, //406
				struct mpMesh *mesh) //407
;

void mpGetMeshBorder(struct mpMesh *mesh, int *widthMinMax, int *heightMinMax) //461
;

void mpSetMeshBorder(struct mpMesh *mesh, int *widthMinMax, int *heightMinMax) //469
;

void mpSetMatrix( //480
				struct mpRenderingContext *rc, //481
				const float *mtx) //482
;

void mpRotate(struct mpRenderingContext *rc, float theta, float axisx, float axisy, float axisz) //489
;

void mpTranslate(struct mpRenderingContext *rc, float x, float y, float z) //498
;

void mpScale(struct mpRenderingContext *rc, float x, float y, float z) //505
;

void mpSetViewport( //516
				struct mpRenderingContext *rc, //517
				int left, //518
				int top, //519
				int width, //520
				int height) //521
;

void mpGetViewport( //541
				struct mpRenderingContext *rc, //541
				int *left, int *top, int *width, int *height) //542
{
	{
		int view[4]; //544
	}
}

void mpSetTexture( //556
				struct mpRenderingContext *rc, //557
				struct mpTexture *texture) //558
;

void mpSetBlendingSwitch( //573
				struct mpRenderingContext *rc, //574
				int bEnableBlend, //575
				float alphaRef) //576
;

void mpSetMaskFunction( //595
				struct mpRenderingContext *rc, //596
				int nMaskFunc) //597
;

void mpSetColorMask(int rMask, int gMask, int bMask, int aMask) //636
;

extern struct mpTexture *g_mpCurrentTexture; //236

extern int meshColorOn; //44

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/gl_draw_layer.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/glasses.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //95
	MP_TEX_GLASSES_LENS = 0,
	MP_TEX_GLASSES_FRAME = 1,
	MP_TEX_GLASSES_SHADOW = 2,
	MP_TEX_GLASSES_REFRACT = 3,
	MP_TEX_GLASSES_LAST = 4,
} mpTexIDGlasses; //101

typedef enum { //103
	MP_TEX_GLASSESOPT_MIRROR = 0,
	MP_TEX_GLASSESOPT_COLOR = 1,
	MP_TEX_GLASSESOPT_LAST = 2,
} mpTexIDGlassesOpt; //107

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //172
	MP_SIDE_RIGHT = 0,
	MP_SIDE_LEFT = 1,
} mpSide; //175

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct { //309
	float scale; //310
	mpRotation rot; //311
	mpVector2 offset; //312
} mpGlassesAdjust; //313

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses { //53
	int oneLens; //54
	struct mpTexture *tex[4]; //55
	struct mpTexture *texopt[2]; //56
	CharaSegment seg[3]; //57
	struct mpMesh *shadowMesh; //58
	struct mpMesh *frameMesh; //59
	struct mpMesh *rLensMesh; //60
	struct mpMesh *lLensMesh; //61
	struct mpMesh *rLensMeshFine; //62
	struct mpMesh *lLensMeshFine; //63
	mpVector2 glassvertex[2][32]; //64
	struct _GlassFine glassfine[2][33]; //65
	int glassfinemax; //66
	mpVector2 eyeCenter; //68
	mpVector2 frameCenter; //69
	mpVector2 lensCenter[2]; //70
	mpRotation rot; //71
	mpColor lensColor; //72
	float mirrAlpha; //73
	float refractXScale, float refractYScale; //74
	mpVector3 vertBuf[153]; //78
} mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef enum { //142
	HAS_VCOLOR = 1,
	NO_VCOLOR = 0,
} VertexColorArg; //145

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

struct mpTexture; { //55
	GLuint textureName; //57
};

typedef struct mpMesh { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
} mpMesh; //76

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

typedef struct _GlassFine { //43
	float x; //44
	float ys; //45
	float ye; //46
} GlassFine; //47

void drawShadowMesh(mpGlasses *glasses) //1037
;

void drawFrameMesh(mpGlasses *glasses) //1022
;

void drawNormalLensforOneLens( //1101
				mpGlasses *glasses, mpMesh *meshfine, GlassFine *fine, //1101
				mpVector2 *center, float sinz, float cosz, float xo, float yo) //1102
{
	{
		mpVector2 texCoordFine[66]; //1104
		mpColor color; //1105
		int w; //1106
		{
			int i; //1108
			{
				float sx, float sy, float qx, float qy; //1109
			}
		}
	}
}

void drawNormalLens( //1080
				mpGlasses *glasses, mpMesh *mesh, mpVector2 *pos, //1080
				mpVector2 *center, float sinz, float cosz, float xo, float yo) //1081
{
	{
		mpVector2 texCoord[12]; //1083
		int w; //1084
		{
			int i; //1086
			{
				float sx, float sy, float qx, float qy; //1087
			}
		}
	}
}

void getEyeCenter(CharaSegment *seg, float *x, float *y) //1380
;

void getLensVertPos(CharaSegment *seg, mpSide side, mpVector2 *pos, int onelens) //1343
{
	{
		int i, int w; //1345
	}
}

void calcTransParams( //909
				mpGlasses *glasses, mpFace *face, float scale, //909
				mpVector2 *eyeCenter, mpVector2 *frameCenter, //910
				float *size, float *rotZ) //911
{
	{
		float lFrmX, float lFrmY; //913
		float rFrmX, float rFrmY; //914
		float dxFrm, float dyFrm; //915
		float lenFrm; //916
		float lEyeX, float lEyeY; //918
		float rEyeX, float rEyeY; //919
		float dxEye, float dyEye; //920
		float lenEye; //921
	}
}

void reshapeShadowMesh( //701
				mpGlasses *glasses, mpFace *face, float *mat, //701
				float sizeW, float sizeH, float z) //702
{
	{
		int i, int j, int n; //704
		int p, int q; //705
		int w, int h; //706
		float px, float py, float pz; //707
		mpVector3 *verts; //710
		const unsigned char *map; //714
	}
}

void reshapeFrameMesh( //850
				mpGlasses *glasses, float sizeW, float sizeH, //850
				float z, float xofst, float yofst) //851
{
	{
		int i, int w; //853
		mpVector3 *verts; //856
		float dz[8]; //862
		{
			float ofstx, float ofsty; //868
		}
	}
}

int initShadowMesh(mpGlasses *glasses) //577
{
	{
		int i, int j, int n; //579
		int w, int h; //580
		mpVector2 *texCoord; //581
		int divW, int divH; //582
	}
}

int loadGlassesModel(const char *data, CharaSegment *seg) //1291
{
	{
		int i; //1293
		int pos; //1294
		int magicNum; //1295
		unsigned int magicNumLen; //1301
		unsigned int segLen; //1302
	}
}

int initFrameMesh(mpGlasses *glasses) //524
{
	{
		int i, int w; //526
		mpVector2 texCoord[16]; //527
	}
}

void drawRefractEye(mpGlasses *glasses, mpMesh *meshfine) //1050
{
	{
		mpVector2 texCoordFine[66]; //1052
		mpColor color; //1053
		float xcenter, float ycenter; //1054
		int div; //1055
		{
			int i; //1059
		}
		{
			int i; //1068
		}
	}
}

void drawMirrorLens( //1130
				mpGlasses *glasses, mpMesh *meshfine, GlassFine *fine, //1130
				mpVector2 *center, float sinz, float cosz, float xo, float yo) //1131
{
	{
		mpVector2 texCoordFine[66]; //1133
		mpColor color; //1134
		float div; //1135
		int w; //1136
		{
			int i; //1146
			{
				float sx, float sy, float qx, float qy; //1147
				float d; //1148
			}
		}
	}
}

void reshapeLensMesh( //764
				mpGlasses *glasses, mpSide side, float sizeW, //764
				float sizeH, float z) //765
{
	{
		int i, int w; //767
		CharaSegment *seg; //768
		mpMesh *mesh, mpMesh *meshfine; //769
		GlassFine *fine; //770
		mpVector3 vertsfine[66]; //771
		float cx, float cy; //772
		mpVector3 *verts; //775
		mpVector2 *pos; //803
	}
}

void reshapeGlasses( //625
				mpGlasses *glasses, mpFace *face, //625
				mpGlassesAdjust *adjust, int doScale) //626
{
	{
		float mat[16]; //628
		float size; //629
		float z; //630
		float scale; //631
		float xofst, float yofst; //632
		mpVector2 *eyec; //634
		mpVector2 *frmc; //635
		mpRotation *rot; //636
	}
}

int initLensMesh(mpGlasses *glasses, mpSide side, int onelens) //473
{
	{
		struct mpMesh *mesh, struct mpMesh *meshfine; //475
	}
}

void drawColorLens( //1173
				mpGlasses *glasses, mpMesh *meshfine, GlassFine *fine, //1173
				mpVector2 *center) //1174
{
	{
		mpVector2 texCoordFine[66]; //1176
		int w; //1177
		{
			int i; //1179
			{
				float sx, float sy; //1180
			}
		}
	}
}

void drawLensMesh(mpGlasses *glasses, mpSide side, mpRotation *rot) //1212
{
	{
		float sinz, float cosz, float xo, float yo; //1214
		mpMesh *mesh, mpMesh *meshfine; //1215
	}
}

mpGlasses* mpCreateGlasses(const char *fileData) //200
{
	{
		mpGlasses *gls; //202
	}
}

void mpSetGlassesTexture( //251
				mpGlasses *glasses, mpTexIDGlasses id, //251
				struct mpTexture *texture) //252
;

void mpSetGlassesOptTexture( //264
				mpGlasses *glasses, mpTexIDGlassesOpt id, //264
				struct mpTexture *texture) //265
;

void mpSetGlassesfine(mpGlasses *glasses, int glassfinemax, float *data) //275
{
	{
		GlassFine *fine; //277
		{
			int i; //286
		}
		{
			int i; //280
		}
		{
			int i; //291
		}
	}
}

void mpSetGlassesLensColor(mpGlasses *glasses, const mpColor *color) //307
;

void mpSetGlassesMirrorAlpha(mpGlasses *glasses, float alpha) //322
;

void mpSetGlassesRefractValue(mpGlasses *glasses, float xscale, float yscale) //327
;

void mpReleaseGlasses(mpGlasses *glasses) //339
;

void mpSetGlasses(mpFace *face, mpGlasses *glasses, mpGlassesAdjust *adjust) //412
;

void mpSetAnimGlasses(mpFace *face, mpGlasses *glasses, mpGlassesAdjust *adjust) //425
;

mpGlasses* mpGetGlasses(mpFace *face) //443
;

void drawGlasses(mpGlasses *glasses, mpFace *face) //979
{
	{
		mpVector2 *eyec; //981
		mpVector2 *frmc; //982
		mpRotation *rot; //983
	}
}

extern mpErrorCode errCode; //171

extern struct mpRenderingContext *_mprc; //172

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/glasses.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/hair.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //109
	MP_TEX_HAIR_RGB = 0,
	MP_TEX_HAIR_BGK = 1,
	MP_TEX_HAIR_FR = 2,
	MP_TEX_HAIR_LAST = 3,
} mpTexIDHair; //114

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair { //8
	struct mpMesh *Mesh; //9
	float *hair_X; //15
	float *hair_Y; //16
	float *hair_Z; //17
	float *hair_F; //18
	float hair_qa1, float hair_qa2, float hair_qa3, float hair_qa4; //21
	float hair_pc, float hair_ps1, float hair_ps2, float hair_pe1, float hair_pe2; //22
	float f_edprx[6]; //24
	float f_edpry[6]; //25
	int zmapWidth, int zmapHeight; //27
	unsigned char *zmap; //28
	float *mesh_x, float *mesh_y; //30
	CharaSegment paradata; //32
	struct mpTexture *tex[3]; //33
	CharaHair charahair; //35
	struct tag_mpHair *next; //36
} mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef enum { //142
	HAS_VCOLOR = 1,
	NO_VCOLOR = 0,
} VertexColorArg; //145

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //198
	float face_ya1, float face_ya2, float face_ya3, float face_ya4; //203
	float face_xs1, float face_xs2, float face_xe1, float face_xe2, float face_xc; //204
	float hair_hfact; //209
	float hair_vfact; //210
	float hair_hmove; //211
	float hair_vmove; //212
	float front_hmove, float front_vmove; //213
	int bgcolor[54]; //214
	mpColor *hairMeshColor; //215
} CharaHair; //217

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

struct mpTexture; { //55
	GLuint textureName; //57
};

typedef struct mpMesh { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
} mpMesh; //76

typedef struct { //32
	int xDiv; //33
	int yDiv; //34
	float *xPosTable; //35
	float *yPosTable; //36
	int *xIdxTable; //37
	int *yIdxTable; //38
} BaseMesh; //39

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

typedef struct _HairMeshFlag { //30
	unsigned char hrma_flag[20][20]; //31
	unsigned char hrbg_flag[20][20]; //32
	unsigned char hrfr_flag[20][20]; //33
	unsigned char dgma_flag[20][20]; //34
	unsigned char dgfr_flag[20][20]; //35
} HairMeshFlag; //36

void calcHairMeshColor(mpFace *face, mpHair *hair) //354
{
	{
		int i, int j, int n, int w, int h, int *bgcolor; //356
		float x, float y; //357
		float *hair_X; //358
		float *hair_Y; //359
		{
			mpColor rgb; //371
			float y2; //377
			int yidx, int yidx2; //378
			float s; //379
		}
	}
}

void draw_hair(mpHair *hair, mpFace *face) //417
{
	{
		int i, int j, int n; //419
		float x, float y, float z; //420
		float px, float py, float pz; //421
		float fx, float fy; //422
		int w, int h; //423
		float m[16]; //424
		mpVector3 *buf; //425
		mpColor color; //426
		float zOffset, float trsFact, float alpha; //427
		mpMesh *pMesh; //428
	}
}

void hair_copyedpr(mpHair *hair, CharaSegment *paradata) //701
{
	{
		int i; //703
	}
}

void geteyeposi3(CharaData *chara, float *rx, float *ry, float *lx, float *ly) //736
;

void spline_hair(CharaSegment *paradata, float ya2, float ya3, float *xs1, float *xe1, float *xs2, float *xe2) //755
{
	{
		int i, int k, int max; //757
		int p0, int p1, int p2, int p3; //758
		float px0, float px1, float px2, float px3; //759
		float py0, float py1, float py2, float py3; //760
		float s, float s2, float s3; //761
		float dx1, float dy1, float dx2, float dy2; //762
		float f1, float f2, float f3, float f4; //763
		float px, float py; //764
		int num; //765
		int stage; //766
	}
}

void reset_hair_fact(CharaHair *chair) //838
;

void hair_set_zmap(mpHair *hair, float zmapFact) //847
{
	{
		int i, int j, int p, int q; //849
		float fx, float fy; //850
		float a, float b, float c, float d; //851
		unsigned char *zmap; //852
	}
}

void hair_setedpr(mpHair *hair) //710
{
	{
		float *f_edprx; //712
		float *f_edpry; //713
	}
}

void mpDrawMeshWithFlag2(struct mpRenderingContext *rc, struct mpMesh *mesh, unsigned char *flag) //42
;

void mpSetHairTexture( //227
				mpHair *hair, mpTexIDHair id, //227
				struct mpTexture *texture) //228
;

void mpReleaseHair(mpHair *hair) //237
;

mpHair* mpCreateHairEdprBin(float *edpr, const char *zmapData) //174
{
	{
		mpHair *hair; //176
		CharaSegment *paradata; //177
		int w, int h; //178
		int bufSize; //213
		mpColor color; //219
		{
			int i; //203
		}
		{
			int i; //220
		}
	}
}

mpHair* mpCreateHairBin(float *pbuf, float *edpr, int div_x, int div_y) //104
{
	{
		mpHair *hair; //106
		CharaSegment *paradata; //107
		mpVector2 *texCoord; //108
		int w, int h; //109
		mpColor color; //167
		{
			int i; //143
		}
		{
			int i; //159
		}
		{
			int i; //168
		}
	}
}

mpHair* mpCreateHair(const char *fileData, const char *zmapData) //45
{
	{
		mpHair *hair; //47
		CharaSegment *paradata; //48
		char *linehead; //49
		int w, int h; //50
		int bufSize; //89
		mpColor color; //95
		{
			int i; //77
		}
		{
			int i; //96
		}
	}
}

void hair_face_ana(mpFace *face, mpHair *hair) //255
{
	{
		CharaHair *chair; //257
		CharaData *chara; //258
		CharaSegment *paradata; //259
		float rx2, float ry2, float lx2, float ly2; //261
	}
}

void mpUnsetHair(mpFace *face, mpHair *hair) //293
{
	{
		mpHair *p, mpHair *pbefore; //295
	}
}

void mpSetHairAdjustFront(mpFace *face, mpHair *hair, float front_hmove, float front_vmove) //324
{
	{
		CharaHair *chair; //326
	}
}

void mpSetHairBGColor(mpFace *face, mpHair *hair, int *bgcolor) //332
;

void drawHair(mpHair *hair, mpFace *face) //408
{
	{
		mpHair *p; //410
	}
}

void hair_face_fit(mpFace *face, mpHair *hair) //534
{
	{
		CharaHair *chair; //536
		int i, int j, int p, int q; //537
		float x, float y, float px, float py, float fx, float fy; //538
		float a, float b, float c, float d; //539
		float ya1, float ya2, float ya3, float ya4; //540
		float xc, float xs1, float xs2, float xe1, float xe2, float xs, float xe; //541
		float ps, float pe; //542
		float face_xc; //544
		float face_xe1; //545
		float face_xe2; //546
		float face_xs1; //547
		float face_xs2; //548
		float face_ya1; //549
		float face_ya2; //550
		float face_ya3; //551
		float hair_hfact; //552
		float hair_vfact; //553
		float hair_hmove; //554
		float hair_vmove; //555
		float hair_pc; //557
		float hair_pe1; //558
		float hair_pe2; //559
		float hair_ps1; //560
		float hair_ps2; //561
		float hair_qa1; //562
		float hair_qa2; //563
		float hair_qa3; //564
		float hair_qa4; //565
		const unsigned char *trsmap; //567
		float TRSIZE; //586
		float *mesh_x, float *mesh_y; //589
		int div_x, int div_y; //590
	}
}

void mpSetHairEdpr(mpFace *face, mpHair *hair, float *edpr) //337
{
	{
		CharaSegment *paradata; //339
		{
			int i; //344
		}
	}
}

void mpSetHairAdjust(mpFace *face, mpHair *hair, float hfact, float vfact, float hmove, float vmove) //312
{
	{
		CharaHair *chair; //314
	}
}

void mpSetHair(mpFace *face, mpHair *hair) //283
{
	hair_set_zmap(); //285
	calcHairMeshColor(); //288
}

void mpSetAnimHair( //890
				mpFace *face, struct mpTexture *texture, //890
				int width, int height, const unsigned char *hairz) //891
{
	{
		float x, float y; //897
		float fx, float fy, float a, float b, float c, float d; //898
		int p, int q, int w, int h, int n; //899
		{
			int i; //906
			{
				int j; //908
			}
		}
	}
}

void mpUnsetAnimHair(mpFace *face) //934
;

extern mpErrorCode errCode; //171

extern struct mpRenderingContext *_mprc; //172

extern BaseMesh bMesh; //47

extern HairMeshFlag mp_hair_mesh_flag; //38

extern int mp_hair_mesh_flag_exist; //40

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/hair.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/hige.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //53
	MP_IMG_HIGE_ZMAP = 0,
	MP_IMG_HIGE_TRSFACT = 1,
	MP_IMG_HIGE_EXPRMAP = 2,
	MP_IMG_HIGE_LAST = 3,
} mpImageHigeID; //58

typedef enum { //116
	MP_TEX_HIGE_RGB = 0,
	MP_TEX_HIGE_LAST = 1,
} mpTexIDHige; //119

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige { //4
	float *hige_S; //10
	float *hige_T; //11
	int mapWidth[3], int mapHeight[3]; //13
	float *mapF[3], float mapFact[3]; //14
	char *map[3]; //15
	float hige_qa1, float hige_qa2, float hige_qa3, float hige_qa4; //19
	float hige_pc, float hige_ps1, float hige_ps2, float hige_pe1, float hige_pe2; //20
	float f_edprx[6]; //22
	float f_edpry[6]; //23
	CharaSegment paradata; //25
	struct mpTexture *tex[1]; //27
	struct mpMesh *mesh; //28
	CharaHige charahige; //30
	struct tag_mpHige *next; //31
} mpHige; //339

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

struct mpTexture; { //55
	GLuint textureName; //57
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

typedef enum { //142
	HAS_VCOLOR = 1,
	NO_VCOLOR = 0,
} VertexColorArg; //145

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //219
	float face_ya1, float face_ya2, float face_ya3, float face_ya4; //224
	float face_xs1, float face_xs2, float face_xe1, float face_xe2, float face_xc; //225
	float hige_hfact; //230
	float hige_vfact; //231
	float hige_hmove; //232
	float hige_vmove; //233
} CharaHige; //235

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef struct { //32
	int xDiv; //33
	int yDiv; //34
	float *xPosTable; //35
	float *yPosTable; //36
	int *xIdxTable; //37
	int *yIdxTable; //38
} BaseMesh; //39

void geteyeposi3(CharaData *chara, float *rx, float *ry, float *lx, float *ly) //452
;

void spline_hige(CharaSegment *paradata, float ya2, float ya3, float *xs1, float *xe1, float *xs2, float *xe2) //471
{
	{
		int i, int k, int max; //473
		int p0, int p1, int p2, int p3; //474
		float px0, float px1, float px2, float px3; //475
		float py0, float py1, float py2, float py3; //476
		float s, float s2, float s3; //477
		float dx1, float dy1, float dx2, float dy2; //478
		float f1, float f2, float f3, float f4; //479
		float px, float py; //480
		int num; //481
		int stage; //482
	}
}

void reset_hige_fact(CharaHige *chige) //553
;

void hige_copyedpr(mpHige *hige, CharaSegment *paradata) //418
{
	{
		int i; //420
	}
}

void hige_setedpr(mpHige *hige) //427
{
	{
		float *f_edprx; //429
		float *f_edpry; //430
	}
}

void hige_face_ana(mpFace *face, mpHige *hige) //35
{
	{
		CharaHige *chige; //37
		CharaData *chara; //38
		CharaSegment *paradata; //39
		float rx2, float ry2, float lx2, float ly2; //41
	}
}

mpHige* mpCreateHige(const float *data) //60
{
	{
		mpHige *hige; //62
		CharaSegment *paradata; //63
		int w; //71
		int h; //72
		{
			int i; //84
		}
	}
}

void mpSetHigeTexture( //95
				mpHige *hige, mpTexIDHige id, //95
				struct mpTexture *texture) //96
;

void mpSetHigeImage(mpHige *hige, mpImageHigeID id, int width, int height, const char *mapData, float mapFact) //105
{
	{
		int w, int h; //107
	}
}

void mpSetHigeAlpha(mpHige *hige, float alpha) //125
;

void mpReleaseHige(mpHige *hige) //130
{
	{
		int i; //132
	}
}

void mpUnsetHige(mpFace *face, mpHige *hige) //151
{
	{
		mpHige *p, mpHige *pbefore; //153
	}
}

void draw_hige(mpHige *hige, mpFace *face) //192
{
	{
		int i, int j, int n, int nzmapNo; //194
		float px, float py, float pz; //195
		float fx, float fy; //196
		int w, int h; //197
		float m[16]; //198
		mpVector3 *buf; //199
		mpVector3 *faceShape; //200
		float *faceZ; //201
		float *trsfactMap, float *exprMap; //202
		float zOffset, float trsFact; //203
	}
}

void drawHige(mpHige *hige, mpFace *face) //184
{
	{
		mpHige *p; //186
	}
}

void hige_face_fit(mpFace *face, mpHige *hige) //268
{
	{
		CharaHige *chige; //270
		int i, int j; //271
		float x, float y, float px, float py; //272
		float ya1, float ya2, float ya3, float ya4; //273
		float xc, float xs1, float xs2, float xe1, float xe2, float xs, float xe; //274
		float ps, float pe; //275
		float face_xc; //277
		float face_xe1; //278
		float face_xe2; //279
		float face_xs1; //280
		float face_xs2; //281
		float face_ya1; //282
		float face_ya2; //283
		float face_ya3; //284
		float hige_hfact; //285
		float hige_vfact; //286
		float hige_hmove; //287
		float hige_vmove; //288
		float hige_pc; //290
		float hige_pe1; //291
		float hige_pe2; //292
		float hige_ps1; //293
		float hige_ps2; //294
		float hige_qa1; //295
		float hige_qa2; //296
		float hige_qa3; //297
		float hige_qa4; //298
		mpVector2 *texaddress; //300
		{
			float *mapF, float mapFact; //389
			unsigned char *map; //390
			{
				int mapWidth, int mapHeight; //392
				{
					int mi; //395
					int mj; //396
				}
			}
		}
	}
}

void mpSetHigeAdjust( //171
				mpFace *face, mpHige *hige, //171
				float hfact, float vfact, float hmove, float vmove) //172
{
	{
		CharaHige *chige; //174
	}
}

void mpSetHige(mpFace *face, mpHige *hige) //143
;

extern mpErrorCode errCode; //171

extern struct mpRenderingContext *_mprc; //172

extern BaseMesh bMesh; //47

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/hige.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/initclose.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef struct { //243
	int faceMeshXDiv; //244
	int faceMeshYDiv; //245
	int enableBlurEyelids; //246
	int enableAlphaFill; //247
} mpContext; //248

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef enum { //243
	EXPR_AREA_OFF = 0,
	EXPR_AREA_ALL = 1,
	EXPR_AREA_EXCEPT_LIP = 2,
	EXPR_AREA_LIP = 3,
} CtrlPointArea; //248

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

struct mpTexture; { //55
	GLuint textureName; //57
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

typedef struct { //32
	int xDiv; //33
	int yDiv; //34
	float *xPosTable; //35
	float *yPosTable; //36
	int *xIdxTable; //37
	int *yIdxTable; //38
} BaseMesh; //39

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

void scaleFaceContour(mpFace *face) //612
{
	{
		float cx; //614
		float cy; //615
		CharaSegment *seg; //616
		{
			int i; //619
		}
		{
			int i; //626
		}
	}
}

int allocFace(mpFace *face) //471
{
	{
		int i; //473
		int bufSize; //474
		{
			LayerData *layer; //508
			int nzmapNo; //509
		}
	}
}

void freeFace(mpFace *face) //557
{
	{
		int i; //559
	}
}

int mpInit(struct mpRenderingContext *renderingContext, mpContext *context) //147
;

void mpClose() //166
;

void mpSetEyeMode(mpFace *face, int original) //219
;

void _mpHasEyeBore(mpFace *face, int yes) //224
;

void _mpSetMouthFeaturePoints2(mpFace *face, int isOpen, float *pnt) //229
{
	{
		int i; //233
	}
}

void _mpSetNoseFeaturePoints(mpFace *face, float *pnt) //239
{
	{
		int i; //242
	}
}

int mpInitFace(mpFace *face) //270
;

void mpSetZmapImage(mpFace *face, int id, const unsigned char *buf) //291
;

void mpSetZmapFact(mpFace *face, float zmapFact, float neckCenter_z) //309
;

int loadFaceModel(mpFace *face, const char *data) //646
{
	{
		int i, int j; //648
		int pos; //649
		int magicNum; //650
		int versionNum; //651
		unsigned int magicNumLen; //657
		unsigned int versionNumLen; //658
		unsigned int modeLen; //659
		unsigned int charaLen; //660
		unsigned int numExprLen; //661
		unsigned int animRectLen; //662
		unsigned int exprLen; //663
		unsigned int numLayerLen; //664
		unsigned int layerLen; //665
		{
			LayerData *layer_i; //720
		}
		{
			int *area; //742
		}
	}
}

mpFace* mpCreateFace(const char *fileData) //187
{
	{
		mpFace *face; //189
		float fZMapFact; //190
	}
}

void mpSetFaceImage( //341
				mpFace *face, int numImg, mpImageID *id, int width, int height, //341
				const unsigned char **buf, float zmapFact, float neckCenter_z) //342
{
	{
		int i; //344
	}
}

void mpSetFaceTexture(mpFace *face, mpTexID id, struct mpTexture *texture) //394
;

void mpSetLayerTexture(mpFace *face, int id, struct mpTexture *texture) //424
;

void mpSetEyeColor(mpFace *face, const mpColor *color) //445
;

void mpReleaseFace(mpFace *face) //458
;

extern mpErrorCode errCode; //171

extern struct mpRenderingContext *_mprc; //172

extern BaseMesh bMesh; //47

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/initclose.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/matrix.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

void mxmul(const float *a, const float *b, float *c) //206
{
	{
		float t00, float t01, float t02, float t03, float t04, float t05, float t06, float t07; //208
		float t08, float t09, float t10, float t11, float t12, float t13, float t14, float t15; //209
	}
}

void saveMtrx() //54
{
	{
		int i; //56
	}
}

void loadMtrx() //67
{
	{
		int i; //69
	}
}

void setMtrx() //81
;

void mkMtrx( //103
				float px, float py, float pz, //103
				float rx, float ry, float rz, float xo, float yo, float zo) //104
{
	{
		float sx, float sy, float sz, float cx, float cy, float cz; //106
		float r0, float r1, float r2, float r3, float r4, float r5, float r6, float r7, float r8; //107
	}
}

void mdMtrx( //162
				float px, float py, float pz, //162
				float rx, float ry, float rz, float xo, float yo, float zo) //163
{
	{
		float sx, float sy, float sz, float cx, float cy, float cz; //165
		float r0, float r1, float r2, float r3, float r4, float r5, float r6, float r7, float r8; //166
	}
}

void getMtrx(float *mtx) //255
{
	{
		int i; //257
	}
}

void transPos(mpVector3 *in, mpVector2 *out) //263
;

struct mpRenderingContext; 

extern struct mpRenderingContext *_mprc; //172

extern float MVMat[16]; //40

float MVMatSave[16]; //44

float mdMat[16]; //47

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/matrix.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/read_mesh_flag.cpp

struct __sbuf; { //87
	unsigned char *_base; //88
	int _size; //89
};

typedef struct __sFILE { //119
	unsigned char *_p; //120
	int _r; //121
	int _w; //122
	short int _flags; //123
	short int _file; //124
	struct __sbuf _bf; //125
	int _lbfsize; //126
	void *_cookie; //129
	void(*)() *_close; //130
	void(*)() *_read; //131
	void(*)() *_seek; //132
	void(*)() *_write; //133
	struct __sbuf _ext; //136
	unsigned char *_up; //138
	int _ur; //139
	unsigned char _ubuf[3]; //142
	unsigned char _nbuf[1]; //143
	struct __sbuf _lb; //146
	int _blksize; //149
	fpos_t _offset; //150
} FILE; //151

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

typedef struct _MeshFlag { //11
	unsigned char face_flag[20][20]; //12
	unsigned char hair_flag[20][20]; //13
	float r_eye_shadow[4]; //14
	float r_eye[4]; //15
	float r_eye_reflect[4]; //16
	float l_eye_shadow[4]; //17
	float l_eye[4]; //18
	float l_eye_reflect[4]; //19
	float lower_teeth[4]; //20
	float upper_teeth[4]; //21
} MeshFlag; //22

typedef struct _HairMeshFlag { //31
	unsigned char hrma_flag[20][20]; //32
	unsigned char hrbg_flag[20][20]; //33
	unsigned char hrfr_flag[20][20]; //34
	unsigned char dgma_flag[20][20]; //35
	unsigned char dgfr_flag[20][20]; //36
} HairMeshFlag; //37

void make_vtx_valid() //44
{
	{
		int i, int j; //46
	}
}

void read_mesh_flag(char *fn) //75
{
	{
		FILE *fp; //77
	}
}

void read_hair_mesh_flag(char *fn) //93
{
	{
		FILE *fp; //95
	}
}

extern MeshFlag mp_mesh_flag; //24

extern int mp_mesh_flag_exist; //26

extern unsigned char face_vtx_valid[21][21]; //28

extern unsigned char hair_vtx_valid[21][21]; //29

extern HairMeshFlag mp_hair_mesh_flag; //39

extern int mp_hair_mesh_flag_exist; //41

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/read_mesh_flag.cpp// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/rendeng.cpp

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //47
	float x; //48
	float y; //49
	float z; //50
} mpVector3; //51

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //69
	float x; //70
	float y; //71
	float z; //72
} mpTranslation; //73

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //53
	MP_IMG_HIGE_ZMAP = 0,
	MP_IMG_HIGE_TRSFACT = 1,
	MP_IMG_HIGE_EXPRMAP = 2,
	MP_IMG_HIGE_LAST = 3,
} mpImageHigeID; //58

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //95
	MP_TEX_GLASSES_LENS = 0,
	MP_TEX_GLASSES_FRAME = 1,
	MP_TEX_GLASSES_SHADOW = 2,
	MP_TEX_GLASSES_REFRACT = 3,
	MP_TEX_GLASSES_LAST = 4,
} mpTexIDGlasses; //101

typedef enum { //103
	MP_TEX_GLASSESOPT_MIRROR = 0,
	MP_TEX_GLASSESOPT_COLOR = 1,
	MP_TEX_GLASSESOPT_LAST = 2,
} mpTexIDGlassesOpt; //107

typedef enum { //109
	MP_TEX_HAIR_RGB = 0,
	MP_TEX_HAIR_BGK = 1,
	MP_TEX_HAIR_FR = 2,
	MP_TEX_HAIR_LAST = 3,
} mpTexIDHair; //114

typedef enum { //116
	MP_TEX_HIGE_RGB = 0,
	MP_TEX_HIGE_LAST = 1,
} mpTexIDHige; //119

typedef enum { //127
	MP_NO_ERROR = 0,
	MP_ERROR_INVALID_PARAM = 1,
	MP_ERROR_INVALID_MODEL = 2,
	MP_ERROR_OUT_OF_MEMORY = 3,
	MP_ERROR_IO = 4,
	MP_ERROR_OTHERS = 5,
} mpErrorCode; //134

typedef enum { //159
	MP_BLINK_TYPE_SINGLE = 0,
	MP_BLINK_TYPE_HALF = 1,
	MP_BLINK_TYPE_DOUBLE = 2,
	MP_BLINK_TYPE_LAST = 3,
} mpBlinkType; //164

typedef enum { //172
	MP_SIDE_RIGHT = 0,
	MP_SIDE_LEFT = 1,
} mpSide; //175

typedef enum { //220
	MP_MODE_ANIME = 0,
	MP_MODE_PHOTO = 1,
	MP_MODE_LAST = 2,
} mpModeType; //224

typedef struct { //243
	int faceMeshXDiv; //244
	int faceMeshYDiv; //245
	int enableBlurEyelids; //246
	int enableAlphaFill; //247
} mpContext; //248

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct tag_mpFace { //472
	int mode; //475
	AnimData anim; //478
	CtrlData ctrl; //481
	struct mpTexture *tex[18]; //484
	struct mpTexture *texLay[11]; //485
	const unsigned char *map[3]; //488
	const unsigned char *zmapLay[11]; //489
	int mapWidth; //492
	int mapHeight; //494
	CharaData chara; //497
	LayerData layer[11]; //499
	int numLayer; //500
	ExprData *expr; //523
	int numExpr; //526
	mpVector3 neckCenter; //529
	float zmapFact; //532
	float pupilZ; //535
	float teepos; //538
	mpVector3 *vertBuf; //548
	mpVector3 *faceShape; //564
	mpVector3 *layerShape[4]; //565
	float *hairZ; //582
	float *faceZ; //584
	float *layerZ[11]; //585
	float *trsFact; //603
	float faceContourZ[32]; //606
	float faceContourTrans[32]; //607
	ExprEffect faceContourExprEffect[128][32]; //608
	mpVector2 faceContour[32]; //609
	CharaSegment mouthFP; //611
	int isMouthOpen; //612
	CharaSegment noseFP; //613
	struct mpMesh *faceMesh; //616
	struct mpMesh *lipMesh; //619
	struct mpMesh *hairMesh; //622
	struct mpMesh *rRectEyeMesh[5]; //625
	struct mpMesh *lRectEyeMesh[5]; //628
	struct mpMesh *rEyelidMesh; //631
	struct mpMesh *lEyelidMesh; //634
	struct mpMesh *teethMesh; //637
	struct mpMesh *eyeShadowMesh; //640
	struct mpMesh *lipShadowMesh; //643
	struct mpMesh *rMatugeMesh; //648
	struct mpMesh *lMatugeMesh; //651
	struct mpMesh *rEyelidBlurMesh; //658
	struct mpMesh *lEyelidBlurMesh; //661
	int lipIdxLeft; //665
	int lipIdxRight; //668
	int lipIdxTop; //671
	int lipIdxBottom; //674
	mpGlasses *glasses; //677
	mpHair *hair; //679
	mpHige *hige; //680
	ExprEffect **exprEffect; //688
	float *exprGain; //695
	CharaRect stdRect; //698
	CharaRect animRect; //706
	CharaRect thisRect; //713
	FaceAlphaTex faceTex[3]; //716
	int orgeye; //721
	int haseyebore; //724
} mpFace; //320

typedef struct tag_mpGlasses { //53
	int oneLens; //54
	struct mpTexture *tex[4]; //55
	struct mpTexture *texopt[2]; //56
	CharaSegment seg[3]; //57
	struct mpMesh *shadowMesh; //58
	struct mpMesh *frameMesh; //59
	struct mpMesh *rLensMesh; //60
	struct mpMesh *lLensMesh; //61
	struct mpMesh *rLensMeshFine; //62
	struct mpMesh *lLensMeshFine; //63
	mpVector2 glassvertex[2][32]; //64
	struct _GlassFine glassfine[2][33]; //65
	int glassfinemax; //66
	mpVector2 eyeCenter; //68
	mpVector2 frameCenter; //69
	mpVector2 lensCenter[2]; //70
	mpRotation rot; //71
	mpColor lensColor; //72
	float mirrAlpha; //73
	float refractXScale, float refractYScale; //74
	mpVector3 vertBuf[153]; //78
} mpGlasses; //327

typedef struct tag_mpHair { //8
	struct mpMesh *Mesh; //9
	float *hair_X; //15
	float *hair_Y; //16
	float *hair_Z; //17
	float *hair_F; //18
	float hair_qa1, float hair_qa2, float hair_qa3, float hair_qa4; //21
	float hair_pc, float hair_ps1, float hair_ps2, float hair_pe1, float hair_pe2; //22
	float f_edprx[6]; //24
	float f_edpry[6]; //25
	int zmapWidth, int zmapHeight; //27
	unsigned char *zmap; //28
	float *mesh_x, float *mesh_y; //30
	CharaSegment paradata; //32
	struct mpTexture *tex[3]; //33
	CharaHair charahair; //35
	struct tag_mpHair *next; //36
} mpHair; //333

typedef struct tag_mpHige { //4
	float *hige_S; //10
	float *hige_T; //11
	int mapWidth[3], int mapHeight[3]; //13
	float *mapF[3], float mapFact[3]; //14
	char *map[3]; //15
	float hige_qa1, float hige_qa2, float hige_qa3, float hige_qa4; //19
	float hige_pc, float hige_ps1, float hige_ps2, float hige_pe1, float hige_pe2; //20
	float f_edprx[6]; //22
	float f_edpry[6]; //23
	CharaSegment paradata; //25
	struct mpTexture *tex[1]; //27
	struct mpMesh *mesh; //28
	CharaHige charahige; //30
	struct tag_mpHige *next; //31
} mpHige; //339

typedef enum { //142
	HAS_VCOLOR = 1,
	NO_VCOLOR = 0,
} VertexColorArg; //145

typedef struct { //102
	mpVector2 rTop; //103
	mpVector2 lTop; //104
	mpVector2 rBtm; //105
	mpVector2 lBtm; //106
} CharaRect; //107

typedef struct { //125
	int num; //126
	mpVector2 pnt[32]; //127
} CharaSegment; //128

typedef struct { //130
	int num; //131
	mpVector2 pnt[24]; //132
	float blink0, float blink1, float YRGB[4]; //133
	float empty[10]; //134
} CharaEyefine; //135

typedef struct { //163
	CharaSegment segment[16]; //164
	CharaEyefine rEyeFine; //165
	CharaEyefine lEyeFine; //166
	CharaPoint rEye; //167
	CharaPoint lEye; //168
	CharaPoint mouth; //169
	CharaPoint lipStart; //170
	CharaPoint lipEnd; //171
	float rEyeWhite; //174
	float lEyeWhite; //175
	float rEyeSize; //177
	float lEyeSize; //178
} CharaData; //195

typedef struct { //198
	float face_ya1, float face_ya2, float face_ya3, float face_ya4; //203
	float face_xs1, float face_xs2, float face_xe1, float face_xe2, float face_xc; //204
	float hair_hfact; //209
	float hair_vfact; //210
	float hair_hmove; //211
	float hair_vmove; //212
	float front_hmove, float front_vmove; //213
	int bgcolor[54]; //214
	mpColor *hairMeshColor; //215
} CharaHair; //217

typedef struct { //219
	float face_ya1, float face_ya2, float face_ya3, float face_ya4; //224
	float face_xs1, float face_xs2, float face_xe1, float face_xe2, float face_xc; //225
	float hige_hfact; //230
	float hige_vfact; //231
	float hige_hmove; //232
	float hige_vmove; //233
} CharaHige; //235

typedef enum { //243
	EXPR_AREA_OFF = 0,
	EXPR_AREA_ALL = 1,
	EXPR_AREA_EXCEPT_LIP = 2,
	EXPR_AREA_LIP = 3,
} CtrlPointArea; //248

typedef enum { //255
	INTERPOLATE_OFF = 0,
	INTERPOLATE_NORMAL = 1,
} Interpolation; //258

typedef struct { //265
	int area; //266
	float cx; //267
	float cy; //268
	float mx; //269
	float my; //270
	float fx; //271
	float fy; //272
	float rt; //273
} CtrlPoint; //274

typedef struct { //281
	float jawMoveY; //282
	int iType; //283
	CtrlPoint ctrlPoint[16]; //284
} ExprData; //285

typedef struct { //291
	unsigned char bOn; //293
	int nlayerNo; //294
	int nzmapNo; //299
	int nmeshNo; //303
	int nmorphNo; //306
	float alpha; //312
	int pad2; //313
	float fTorsoRate; //314
	mpVector2 pos; //315
	mpRotation rot; //316
	mpVector2 scale; //317
} LayerData; //319

typedef struct { //135
	float mxy[4][2]; //142
} ExprEffect; //144

typedef struct { //146
	int width, int height; //147
	unsigned char *alpha; //148
} FaceAlphaTex; //149

typedef struct { //157
	int enable; //158
	int time; //159
	int startTime; //160
	int duration; //161
	float rot; //162
	float startRot; //163
	float endRot; //164
	float maxRot; //165
	float durFact; //166
	int itype; //167
} NeckUAnim; //168

typedef struct { //175
	int enable; //176
	int time; //177
	int startTime; //178
	int duration; //179
	float x; //180
	float lastX; //181
	float y; //182
	float xMax; //183
	float yMax; //184
	float durFact; //185
} PupilUAnim; //186

typedef struct { //193
	float gainMax; //194
	mpBlinkType type; //195
	float gain[4]; //196
	float alpha[4]; //197
} BlinkData; //198

typedef struct { //205
	int enable; //206
	int time; //207
	int startTime; //208
	int duration; //209
	float durFact; //210
	int freq[3]; //211
	BlinkData blink; //212
	float gainFact; //213
} BlinkUAnim; //214

typedef struct { //223
	int enable; //224
	int enableBreath; //225
	float timer[128]; //226
	float gain[128]; //227
	float durFact; //228
	float durBreathFact; //229
	float gainFact[128]; //230
} ExprUAnim; //231

typedef struct { //240
	int time; //241
	int startTime; //242
	int duration; //243
	float rotX; //244
	float startRotX; //245
	float endRotX; //246
	float rotY; //247
	float startRotY; //248
	float endRotY; //249
	float rotZ; //250
	float startRotZ; //251
	float endRotZ; //252
	float weight; //253
	float startWeight; //254
	float endWeight; //255
} LeanAnim; //256

typedef struct { //265
	int time; //266
	int startTime; //267
	int duration; //268
	float translX; //269
	float startTranslX; //270
	float endTranslX; //271
	float translY; //272
	float startTranslY; //273
	float endTranslY; //274
	float translZ; //275
	float startTranslZ; //276
	float endTranslZ; //277
	float weight; //278
	float startWeight; //279
	float endWeight; //280
} ShiftAnim; //281

typedef struct { //288
	int time; //289
	int startTime; //290
	int duration; //291
	float pupilX; //292
	float startPupilX; //293
	float endPupilX; //294
	float pupilY; //295
	float startPupilY; //296
	float endPupilY; //297
	float rotX; //298
	float startRotX; //299
	float endRotX; //300
	float rotY; //301
	float startRotY; //302
	float endRotY; //303
	float rotZ; //304
	float weight; //305
	float startWeight; //306
	float endWeight; //307
	float eyeGain; //308
} LookAtAnim; //313

typedef struct { //320
	int time; //321
	int startTime; //322
	int duration; //323
	float gain[128]; //324
	float startGain[128]; //325
	float endGain[128]; //326
	float weight; //327
	float startWeight; //328
	float endWeight; //329
} ExpressAnim; //333

typedef struct { //340
	int time; //341
	int startTime; //342
	int duration; //343
	int prevIdx; //344
	int nextIdx; //345
	float prevGain; //346
	float nextGain; //347
	float gain[3]; //348
	mpVoice *voice; //349
} LipSync; //350

typedef struct { //357
	int time; //358
	int startTime; //359
	int duration; //360
	float weight; //361
	float startWeight; //362
	float endWeight; //363
	int startMargin; //364
	int endMargin; //365
	LipSync lipSync; //366
	int lipKeepStart; //367
	int lipKeepAfter; //368
	float lip_gain[3]; //369
} SpeakAnim; //370

typedef struct { //377
	int time; //378
	int startTime; //379
	int duration; //380
	float close; //381
	float startClose; //382
	float endClose; //383
	float weight; //384
} CloseEyeAnim; //385

typedef struct { //392
	int enable; //393
	int time; //394
	int startTime; //395
	BlinkData blink; //396
} BlinkAnim; //397

typedef struct { //406
	long unsigned int startTime; //407
	long unsigned int duration; //408
	float curGain, float endGain; //409
	long unsigned int prevTime; //410
	NeckUAnim uNeckX; //411
	NeckUAnim uNeckY; //412
	NeckUAnim uNeckZ; //413
	BlinkUAnim uBlink; //414
	PupilUAnim uPupil; //415
	ExprUAnim uExpr; //416
	LeanAnim lean; //417
	ShiftAnim shift; //418
	LookAtAnim lookAt; //419
	ExpressAnim express; //420
	SpeakAnim speak; //421
	CloseEyeAnim closeEye; //422
	BlinkAnim blink; //423
} AnimData; //424

typedef struct { //436
	mpVector3 rPupilPos; //441
	mpVector3 lPupilPos; //442
	mpRotation neckRot; //444
	mpTranslation neckTransl; //445
	float exprGain[128]; //446
	float rBlinkGain[4]; //447
	float rBlinkAlpha[4]; //448
	float lBlinkGain[4]; //449
	float lBlinkAlpha[4]; //450
	float rEyeClose; //451
	float lEyeClose; //452
} CtrlData; //453

typedef enum { //455
	RECT_EYE_BASE = 0,
	RECT_EYE_CORNEA = 1,
	RECT_EYE_IRIS = 2,
	RECT_EYE_PUPIL = 3,
	RECT_EYE_REFLECT = 4,
	RECT_EYE_LAST = 5,
} RectEyeTexID; //462

struct mpTexture; { //55
	GLuint textureName; //57
};

struct mpMesh; { //65
	int widthDivision; //67
	int heightDivision; //68
	int widthMin, int widthMax, int heightMin, int heightMax; //69
	mpVector3 *position; //70
	mpVector2 *textureAddress; //71
	short unsigned int *indices; //72
	mpColor *color; //73
	mpColor *drawColorBuffer; //74
	mpColor meshColor; //75
};

typedef struct { //32
	int xDiv; //33
	int yDiv; //34
	float *xPosTable; //35
	float *yPosTable; //36
	int *xIdxTable; //37
	int *yIdxTable; //38
} BaseMesh; //39

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

struct _GlassFine; { //43
	float x; //44
	float ys; //45
	float ye; //46
};

void getEyeCenter(CharaSegment *segment, float *x, float *y) //2508
;

void calcPupEffectParam( //2478
				CharaSegment *segment, float eyeSize, //2478
				const mpVector3 *pos, CtrlPoint *effect) //2479
;

void drawLipMesh( //3218
				mpFace *face, struct mpTexture *tex, const float *exprGain) //3219
{
	{
		int i, int j, int n; //3221
		int ii, int jj, int nn; //3222
		int divW, int divH; //3223
		int w, int h; //3224
		float x, float y, float z; //3225
		mpVector3 *buf; //3226
		mpVector3 *lipShape; //3227
	}
}

void drawContactLens( //2810
				mpFace *face, mpSide side, const mpVector3 *pupilPos, //2810
				const mpRotation *rot) //2811
{
	{
		float px, float py, float size, float brt, float x, float y, float xs, float xe, float m, float xo, float yo; //2813
		mpColor color; //2814
		struct mpMesh **pmesh; //2815
		CharaData *chara; //2816
		mpVector2 texCoord[4]; //2817
		float Er, float Eg, float Eb; //2824
		float min; //2825
	}
}

int initLipShadowMesh(mpFace *face) //1601
{
	{
		int i; //1603
		int divW, int divH; //1604
		int w, int h; //1605
		mpVector2 *texCoord; //1606
		mpColor *color; //1607
	}
}

int initEyeShadowMesh(mpFace *face) //1682
{
	{
		int i; //1684
		int divW, int divH; //1685
		int w, int h; //1686
		mpVector2 *texCoord; //1687
		mpColor *color; //1688
	}
}

int makeXTables() //257
{
	{
		int i, int j, int w; //259
		float x, float y, float c; //260
	}
}

int makeYTables() //323
{
	{
		int i, int j, int h; //325
		float x, float y, float c; //326
	}
}

void calcExprGain( //2182
				CtrlData *ctrl, const ExprData *expr, int numExpr, //2182
				float *exprGain, float *jawMoveY) //2183
{
	{
		int i; //2185
		int iType; //2186
	}
}

void drawLipShadowMesh(mpFace *face, struct mpTexture *tex) //3810
{
	{
		int i, int w, int h; //3812
		CharaSegment *segment; //3813
		mpVector3 *buf; //3814
	}
}

void initThisRectAnime(CharaSegment *segment, CharaRect *thisRect) //478
;

void initThisRectPhoto(CharaSegment *segment, CharaRect *thisRect) //501
{
	{
		int i, int n, int num; //503
		float sumx, float sumy; //504
	}
}

int initLipMesh(mpFace *face) //1018
{
	{
		int i, int j, int n; //1020
		int divW, int divH; //1021
		int w, int h; //1022
		mpVector2 *texCoord; //1023
		int diamCX; //1047
		int diamCY; //1048
	}
}

int initTeethMesh(mpFace *face) //1523
{
	{
		int i, int j, int n; //1525
		int w, int h; //1526
		mpVector2 *texCoord; //1527
		int divW, int divH; //1528
		float white; //1529
		mpColor color; //1530
	}
}

int initRectMesh(mpFace *face) //1762
{
	{
		int i; //1764
		mpVector2 texCoord[4]; //1766
	}
}

void initNeckCenter(mpFace *face) //642
{
	{
		mpVector2 stdCenter; //644
		mpVector2 center; //645
		mpVector2 dummy; //646
	}
}

void drawRectMesh( //3939
				struct mpMesh *mesh, float left, float right, float top, //3939
				float bottom, float z) //3940
{
	{
		mpVector3 buf[4]; //3942
	}
}

float calcPupEffectFactor(const CtrlPoint *ctrlPoint, float x, float y) //2434
{
	{
		float r, float dx, float dy; //2436
	}
}

void convPoint( //730
				CharaRect *from, CharaRect *to, const mpVector2 *inPos, //730
				mpVector2 *outPos, mpVector2 *outScale) //731
{
	{
		float px, float py, float xs, float xe, float us, float ue; //733
	}
}

void drawTeethMesh( //3719
				mpFace *face, struct mpTexture *tex, float moveY, //3720
				const mpVector2 *scale, float z) //3721
{
	{
		int i, int j, int p, int q, int n; //3723
		int w, int h; //3724
		float px, float py, float faceZ; //3725
		mpVector3 *buf; //3726
		const unsigned char *map; //3727
		float width; //3729
		float height; //3730
		mpColor color; //3768
	}
}

void drawFaceMesh(mpFace *face, struct mpTexture *tex, int nzmapNo, float zOffset, float trsFact, float alpha) //2969
{
	{
		int i, int j, int n; //2971
		float px, float py, float pz; //2972
		float fx, float fy; //2973
		int w, int h; //2974
		float m[16]; //2975
		mpVector3 *buf; //2976
		mpVector3 *faceShape; //2977
		float *faceZ; //2978
		mpColor color; //2979
	}
}

void drawHairMesh(mpFace *face, struct mpTexture *tex, int nzmapNo, float zOffset, float trsFact, float alpha) //4039
{
	{
		int i, int j, int n; //4041
		float x, float y, float z; //4042
		float px, float py, float pz; //4043
		float fx, float fy; //4044
		int w, int h; //4045
		float m[16]; //4046
		mpVector3 *buf; //4047
		float *hairZ; //4048
		mpColor color; //4049
		int wMinMax[2], int hMinMax[2]; //4051
	}
}

void fillFaceAlpha(mpFace *face) //3970
{
	{
		float m[16]; //3972
		mpVector3 pos[32]; //3973
		int num; //3974
		mpColor col; //3975
		{
			int i; //3979
			{
				float px; //3980
				float py; //3981
				float pz; //3982
				float fx; //3983
				{
					float fy; //3989
				}
			}
		}
	}
}

int initEyelidTexcoord(mpFace *face, mpSide side) //1074
{
	{
		int i, int j, int n; //1076
		int w, int h; //1077
		float fact1, float fact2, float fact3; //1078
		float pos; //1079
		CharaPoint *p; //1080
		struct mpMesh *mesh; //1081
		mpVector2 *texCoord; //1082
	}
}

float calcFactor( //675
				CharaRect *animRect, CharaRect *thisRect, //675
				CtrlPoint *ctrlPoint, float x, float y) //676
{
	{
		float r, float u, float v, float dx, float dy, float rt, float fx, float fy; //678
		mpVector2 origPos; //679
		mpVector2 pos; //680
		mpVector2 scale; //681
	}
}

int initEyelidMesh(mpFace *face, mpSide side) //1185
{
	{
		int i, int j, int n; //1187
		int divW, int divH; //1188
		int w, int h; //1189
		struct mpMesh *mesh; //1190
		mpColor *vColor; //1191
	}
}

int initMatugeMesh(mpFace *face, mpSide side) //1427
{
	{
		int i, int j, int n; //1429
		float x; //1430
		int divW, int divH; //1431
		int w, int h; //1432
		struct mpMesh *mesh; //1433
		mpVector2 *texCoord; //1434
		mpColor *vColor; //1435
	}
}

int initEyelidBlurMesh(mpFace *face, mpSide side) //1296
{
	{
		int i, int j, int n; //1298
		int divW, int divH; //1299
		int w, int h; //1300
		float fact2, float fact3; //1301
		float pos; //1302
		CharaPoint *p; //1303
		struct mpMesh *mesh; //1304
		mpVector2 *texCoord; //1305
		mpColor *vColor; //1306
	}
}

void calcFaceShape(mpFace *face, const float *exprGain) //2224
{
	{
		int i, int j, int k, int l, int n; //2226
		int w, int h; //2227
		float pupFact; //2228
		CtrlPoint rPupEffect; //2229
		CtrlPoint lPupEffect; //2230
		mpVector3 *faceShape; //2231
		mpVector3 *lipShape; //2232
		ExprEffect *effect; //2233
		int exprTblNum, int exprTbl[128]; //2235
		float xls[2], float yls[2]; //2236
		int numExpr; //2237
		CharaSegment *seg; //2306
		{
			float xls; //2308
			float yls; //2309
		}
	}
}

void drawEyePhoto( //2627
				mpFace *face, mpSide side, const mpVector3 *pupilPos, //2627
				const mpRotation *rot) //2628
{
	{
		float px, float py, float size, float brt, float x, float y, float xs, float xe, float m, float xo, float yo; //2630
		mpColor color; //2631
		struct mpMesh **pmesh; //2632
		CharaData *chara; //2633
		mpVector2 texCoord[4]; //2634
		float Er, float Eg, float Eb; //2641
		float min; //2642
	}
}

void drawEyeAnime(mpFace *face, mpSide side, const mpVector3 *pupilPos) //2540
{
	{
		struct mpTexture *tShadow; //2542
		struct mpTexture *tEye; //2543
		struct mpTexture *tReflect; //2544
		float fx, float fy; //2545
		int cx, int cy; //2546
		mpColor color; //2547
		struct mpMesh **pmesh; //2548
		CharaSegment *segment; //2549
	}
}

void setContactMode(int flag) //176
;

int initRendEng(mpContext *context) //189
{
	makeXTables(); //208
	makeYTables(); //208
}

void closeRendEng() //220
;

void initExprEffect( //557
				mpFace *face, ExprData *expr, CharaRect *animRect, //557
				CharaRect *thisRect, ExprEffect *effect, ExprEffect *faceContourEffect) //558
{
	{
		int i, int j, int k, int l, int n; //560
		float x, float y, float fact; //561
		int w, int h; //562
		CtrlPoint *cp; //563
		CharaSegment *seg; //613
	}
}

void mpSetEyelidLength(mpFace *face, float fact2, float fact3, mpSide side) //1259
;

int mpDrawHair(mpFace *face, mpHair *hair) //2154
;

void closeFace(mpFace *face) //4252
{
	{
		int i; //4254
	}
}

int getXIndex(float x) //4326
{
	{
		int i; //4328
	}
}

int getYIndex(float y) //4355
{
	{
		int i; //4357
	}
}

int mpGetPointPos(mpFace *face, float u, float v, int layer_id, float *x, float *y, float *z) //3068
{
	{
		int i, int j, int n, int p, int q, int w; //3070
		float px, float py, float pz; //3071
		float gx[2][2], float gy[2][2], float gz[2][2]; //3072
		float fx, float fy, float cx0, float cx1, float cy0, float cy1; //3073
		float a, float b, float c, float d; //3074
		float m[16]; //3075
		float trsFact, float zOffset, float *faceZ; //3076
		CtrlData *ctrl; //3077
		LayerData *layer; //3078
		mpVector3 *faceShape; //3079
	}
}

void applyFaceShape(mpFace *face, mpVector3 *vert, int numVert) //4167
{
	{
		int i, int p, int q; //4169
		float x, float y, float fx, float fy, float cx0, float cx1, float cy0, float cy1; //4170
		float a, float b, float c, float d; //4171
		float faceZ; //4172
		const unsigned char *map; //4173
	}
}

void drawEyeShadowMesh( //3861
				mpFace *face, mpSide side, struct mpTexture *tex, //3862
				float gain, float close) //3863
{
	{
		int i; //3865
		int w, int h; //3866
		float gain2; //3867
		float w1; //3868
		CharaPoint *p; //3869
		mpVector3 *buf; //3870
	}
}

void drawEyelidBlurMesh( //3487
				mpFace *face, mpSide side, struct mpTexture *tex, //3488
				float gain, float alpha, float close) //3489
{
	{
		int i, int j, int n; //3491
		int w, int h; //3492
		float fact2, float fact3, float gain2, float pos; //3493
		struct mpMesh *mesh; //3494
		CharaPoint *p; //3495
		mpVector3 *buf; //3496
		mpColor color; //3497
	}
}

void drawEyelidMesh( //3312
				mpFace *face, mpSide side, struct mpTexture *tex, //3313
				float gain, float alpha, float close) //3314
{
	{
		int i, int j, int n; //3316
		int w, int h; //3317
		float fact1, float fact2, float fact3, float gain1, float gain2, float pos; //3318
		struct mpMesh *mesh; //3319
		CharaPoint *p; //3320
		mpVector3 *buf; //3321
		mpColor color; //3322
	}
}

void drawMatugeMesh( //3612
				mpFace *face, mpSide side, struct mpTexture *tex, //3613
				float gain, float alpha, float close) //3614
{
	{
		int i, int j, int n; //3616
		float dy; //3617
		int w, int h; //3618
		CharaPoint *p; //3619
		struct mpMesh *mesh; //3620
		mpVector3 *buf; //3621
		mpColor color; //3622
	}
}

void calcLayerShape(mpFace *face, const float *exprGain) //2331
{
	{
		int i, int j, int k, int l, int n; //2333
		int w, int h; //2334
		float pupFact; //2335
		CtrlPoint rPupEffect; //2336
		CtrlPoint lPupEffect; //2337
		mpVector3 *faceShape; //2338
		mpVector3 **layerShape; //2339
		ExprEffect *effect; //2340
		CharaSegment *seg; //2397
		{
			float xls; //2399
			float yls; //2400
		}
	}
}

int mpDraw(mpFace *face, int x, int y, int width, int height) //1850
{
	{
		int IsMultiLayer; //1852
		int i; //1853
		float *exprGain; //1854
		mpVector2 stdCenter; //1855
		mpVector2 center; //1856
		mpVector2 scale; //1857
		CtrlData *ctrl; //1859
		struct mpTexture **tex; //1860
		{
			float zOffset; //1902
			LayerData *layer; //1903
			int l; //1905
		}
		{
			mpTexID texid; //2009
		}
		{
			mpTexID texid; //2036
		}
	}
}

int initFaceMesh(mpFace *face) //831
{
	{
		int i, int j; //833
		float x, float y; //834
		int p, int q; //835
		int divW, int divH; //836
		int w, int h; //837
		int n; //838
		int bufSize; //839
		float fx, float fy, float a, float b, float c, float d; //840
		const unsigned char *map; //841
		mpVector2 *texCoord; //842
		int maxp, int maxq; //843
		CharaSegment *seg; //939
		int diamCX; //970
		int diamCY; //971
	}
}

int initFace(mpFace *face, int mode) //396
{
	{
		int i; //398
	}
}

extern mpErrorCode errCode; //171

extern struct mpRenderingContext *_mprc; //172

extern BaseMesh bMesh; //157

int blurEyelids; //166

int fillAlpha; //166

int doDrawContactLens; //175

// /Users/iwasaki/src/common/lib.android/motiport\jni/../../../lib/motiport/rendeng.cpp// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/Base64Transcoder.c

size_t EstimateBas64EncodedDataSize(size_t inDataSize) //104
{
	size_t theEncodedDataSize; //106
}

size_t EstimateBas64DecodedDataSize(size_t inDataSize) //111
{
	size_t theDecodedDataSize; //113
}

_Bool Base64EncodeData(void *inInputData, size_t inInputDataSize, char *outOutputData, size_t *ioOutputDataSize) //118
{
	size_t theEncodedDataSize; //120
	const u_int8_t *theInPtr; //124
	u_int32_t theInIndex, u_int32_t theOutIndex; //125
	const size_t theRemainingBytes; //138
}

_Bool Base64DecodeData(void *inInputData, size_t inInputDataSize, void *ioOutputData, size_t *ioOutputDataSize) //166
{
	size_t theDecodedDataSize; //170
	const u_int8_t *theInPtr; //174
	u_int8_t *theOutPtr; //175
	size_t theInIndex, size_t theOutIndex; //176
	u_int8_t theOutputOctet; //177
	size_t theSequence; //178
	{
		int8_t theSextet; //181
		int8_t theCurrentInputOctet; //183
	}
}

extern const u_int8_t kBase64EncodeTable[64]; //33

extern const int8_t kBase64DecodeTable[128]; //60

extern const u_int8_t kBits_00000011; //95

extern const u_int8_t kBits_00001111; //96

extern const u_int8_t kBits_00110000; //97

extern const u_int8_t kBits_00111100; //98

extern const u_int8_t kBits_00111111; //99

extern const u_int8_t kBits_11000000; //100

extern const u_int8_t kBits_11110000; //101

extern const u_int8_t kBits_11111100; //102

// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/Base64Transcoder.c// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/app-android.c

struct _jfieldID; 

struct _jmethodID; 

typedef union jvalue { //123
	jboolean z; //124
	jbyte b; //125
	jchar c; //126
	jshort s; //127
	jint i; //128
	jlong j; //129
	jfloat f; //130
	jdouble d; //131
	jobject l; //132
} jvalue; //133

typedef enum jobjectRefType { //135
	JNIInvalidRefType = 0,
	JNILocalRefType = 1,
	JNIGlobalRefType = 2,
	JNIWeakGlobalRefType = 3,
} jobjectRefType; //140

typedef struct { //142
	const char *name; //143
	const char *signature; //144
	void *fnPtr; //145
} JNINativeMethod; //146

struct JNINativeInterface; { //163
	void *reserved0; //164
	void *reserved1; //165
	void *reserved2; //166
	void *reserved3; //167
	void(*)() *GetVersion; //169
	void(*)() *DefineClass; //171
	void(*)() *FindClass; //173
	void(*)() *FromReflectedMethod; //175
	void(*)() *FromReflectedField; //176
	void(*)() *ToReflectedMethod; //178
	void(*)() *GetSuperclass; //180
	void(*)() *IsAssignableFrom; //181
	void(*)() *ToReflectedField; //184
	void(*)() *Throw; //186
	void(*)() *ThrowNew; //187
	void(*)() *ExceptionOccurred; //188
	void(*)() *ExceptionDescribe; //189
	void(*)() *ExceptionClear; //190
	void(*)() *FatalError; //191
	void(*)() *PushLocalFrame; //193
	void(*)() *PopLocalFrame; //194
	void(*)() *NewGlobalRef; //196
	void(*)() *DeleteGlobalRef; //197
	void(*)() *DeleteLocalRef; //198
	void(*)() *IsSameObject; //199
	void(*)() *NewLocalRef; //201
	void(*)() *EnsureLocalCapacity; //202
	void(*)() *AllocObject; //204
	void(*)() *NewObject; //205
	void(*)() *NewObjectV; //206
	void(*)() *NewObjectA; //207
	void(*)() *GetObjectClass; //209
	void(*)() *IsInstanceOf; //210
	void(*)() *GetMethodID; //211
	void(*)() *CallObjectMethod; //213
	void(*)() *CallObjectMethodV; //214
	void(*)() *CallObjectMethodA; //215
	void(*)() *CallBooleanMethod; //216
	void(*)() *CallBooleanMethodV; //217
	void(*)() *CallBooleanMethodA; //218
	void(*)() *CallByteMethod; //219
	void(*)() *CallByteMethodV; //220
	void(*)() *CallByteMethodA; //221
	void(*)() *CallCharMethod; //222
	void(*)() *CallCharMethodV; //223
	void(*)() *CallCharMethodA; //224
	void(*)() *CallShortMethod; //225
	void(*)() *CallShortMethodV; //226
	void(*)() *CallShortMethodA; //227
	void(*)() *CallIntMethod; //228
	void(*)() *CallIntMethodV; //229
	void(*)() *CallIntMethodA; //230
	void(*)() *CallLongMethod; //231
	void(*)() *CallLongMethodV; //232
	void(*)() *CallLongMethodA; //233
	void(*)() *CallFloatMethod; //234
	void(*)() *CallFloatMethodV; //235
	void(*)() *CallFloatMethodA; //236
	void(*)() *CallDoubleMethod; //237
	void(*)() *CallDoubleMethodV; //238
	void(*)() *CallDoubleMethodA; //239
	void(*)() *CallVoidMethod; //240
	void(*)() *CallVoidMethodV; //241
	void(*)() *CallVoidMethodA; //242
	void(*)() *CallNonvirtualObjectMethod; //244
	void(*)() *CallNonvirtualObjectMethodV; //246
	void(*)() *CallNonvirtualObjectMethodA; //248
	void(*)() *CallNonvirtualBooleanMethod; //250
	void(*)() *CallNonvirtualBooleanMethodV; //252
	void(*)() *CallNonvirtualBooleanMethodA; //254
	void(*)() *CallNonvirtualByteMethod; //256
	void(*)() *CallNonvirtualByteMethodV; //258
	void(*)() *CallNonvirtualByteMethodA; //260
	void(*)() *CallNonvirtualCharMethod; //262
	void(*)() *CallNonvirtualCharMethodV; //264
	void(*)() *CallNonvirtualCharMethodA; //266
	void(*)() *CallNonvirtualShortMethod; //268
	void(*)() *CallNonvirtualShortMethodV; //270
	void(*)() *CallNonvirtualShortMethodA; //272
	void(*)() *CallNonvirtualIntMethod; //274
	void(*)() *CallNonvirtualIntMethodV; //276
	void(*)() *CallNonvirtualIntMethodA; //278
	void(*)() *CallNonvirtualLongMethod; //280
	void(*)() *CallNonvirtualLongMethodV; //282
	void(*)() *CallNonvirtualLongMethodA; //284
	void(*)() *CallNonvirtualFloatMethod; //286
	void(*)() *CallNonvirtualFloatMethodV; //288
	void(*)() *CallNonvirtualFloatMethodA; //290
	void(*)() *CallNonvirtualDoubleMethod; //292
	void(*)() *CallNonvirtualDoubleMethodV; //294
	void(*)() *CallNonvirtualDoubleMethodA; //296
	void(*)() *CallNonvirtualVoidMethod; //298
	void(*)() *CallNonvirtualVoidMethodV; //300
	void(*)() *CallNonvirtualVoidMethodA; //302
	void(*)() *GetFieldID; //305
	void(*)() *GetObjectField; //307
	void(*)() *GetBooleanField; //308
	void(*)() *GetByteField; //309
	void(*)() *GetCharField; //310
	void(*)() *GetShortField; //311
	void(*)() *GetIntField; //312
	void(*)() *GetLongField; //313
	void(*)() *GetFloatField; //314
	void(*)() *GetDoubleField; //315
	void(*)() *SetObjectField; //317
	void(*)() *SetBooleanField; //318
	void(*)() *SetByteField; //319
	void(*)() *SetCharField; //320
	void(*)() *SetShortField; //321
	void(*)() *SetIntField; //322
	void(*)() *SetLongField; //323
	void(*)() *SetFloatField; //324
	void(*)() *SetDoubleField; //325
	void(*)() *GetStaticMethodID; //327
	void(*)() *CallStaticObjectMethod; //329
	void(*)() *CallStaticObjectMethodV; //330
	void(*)() *CallStaticObjectMethodA; //331
	void(*)() *CallStaticBooleanMethod; //332
	void(*)() *CallStaticBooleanMethodV; //333
	void(*)() *CallStaticBooleanMethodA; //335
	void(*)() *CallStaticByteMethod; //337
	void(*)() *CallStaticByteMethodV; //338
	void(*)() *CallStaticByteMethodA; //339
	void(*)() *CallStaticCharMethod; //340
	void(*)() *CallStaticCharMethodV; //341
	void(*)() *CallStaticCharMethodA; //342
	void(*)() *CallStaticShortMethod; //343
	void(*)() *CallStaticShortMethodV; //344
	void(*)() *CallStaticShortMethodA; //345
	void(*)() *CallStaticIntMethod; //346
	void(*)() *CallStaticIntMethodV; //347
	void(*)() *CallStaticIntMethodA; //348
	void(*)() *CallStaticLongMethod; //349
	void(*)() *CallStaticLongMethodV; //350
	void(*)() *CallStaticLongMethodA; //351
	void(*)() *CallStaticFloatMethod; //352
	void(*)() *CallStaticFloatMethodV; //353
	void(*)() *CallStaticFloatMethodA; //354
	void(*)() *CallStaticDoubleMethod; //355
	void(*)() *CallStaticDoubleMethodV; //356
	void(*)() *CallStaticDoubleMethodA; //357
	void(*)() *CallStaticVoidMethod; //358
	void(*)() *CallStaticVoidMethodV; //359
	void(*)() *CallStaticVoidMethodA; //360
	void(*)() *GetStaticFieldID; //362
	void(*)() *GetStaticObjectField; //365
	void(*)() *GetStaticBooleanField; //366
	void(*)() *GetStaticByteField; //367
	void(*)() *GetStaticCharField; //368
	void(*)() *GetStaticShortField; //369
	void(*)() *GetStaticIntField; //370
	void(*)() *GetStaticLongField; //371
	void(*)() *GetStaticFloatField; //372
	void(*)() *GetStaticDoubleField; //373
	void(*)() *SetStaticObjectField; //375
	void(*)() *SetStaticBooleanField; //376
	void(*)() *SetStaticByteField; //377
	void(*)() *SetStaticCharField; //378
	void(*)() *SetStaticShortField; //379
	void(*)() *SetStaticIntField; //380
	void(*)() *SetStaticLongField; //381
	void(*)() *SetStaticFloatField; //382
	void(*)() *SetStaticDoubleField; //383
	void(*)() *NewString; //385
	void(*)() *GetStringLength; //386
	void(*)() *GetStringChars; //387
	void(*)() *ReleaseStringChars; //388
	void(*)() *NewStringUTF; //389
	void(*)() *GetStringUTFLength; //390
	void(*)() *GetStringUTFChars; //392
	void(*)() *ReleaseStringUTFChars; //393
	void(*)() *GetArrayLength; //394
	void(*)() *NewObjectArray; //395
	void(*)() *GetObjectArrayElement; //396
	void(*)() *SetObjectArrayElement; //397
	void(*)() *NewBooleanArray; //399
	void(*)() *NewByteArray; //400
	void(*)() *NewCharArray; //401
	void(*)() *NewShortArray; //402
	void(*)() *NewIntArray; //403
	void(*)() *NewLongArray; //404
	void(*)() *NewFloatArray; //405
	void(*)() *NewDoubleArray; //406
	void(*)() *GetBooleanArrayElements; //408
	void(*)() *GetByteArrayElements; //409
	void(*)() *GetCharArrayElements; //410
	void(*)() *GetShortArrayElements; //411
	void(*)() *GetIntArrayElements; //412
	void(*)() *GetLongArrayElements; //413
	void(*)() *GetFloatArrayElements; //414
	void(*)() *GetDoubleArrayElements; //415
	void(*)() *ReleaseBooleanArrayElements; //417
	void(*)() *ReleaseByteArrayElements; //419
	void(*)() *ReleaseCharArrayElements; //421
	void(*)() *ReleaseShortArrayElements; //423
	void(*)() *ReleaseIntArrayElements; //425
	void(*)() *ReleaseLongArrayElements; //427
	void(*)() *ReleaseFloatArrayElements; //429
	void(*)() *ReleaseDoubleArrayElements; //431
	void(*)() *GetBooleanArrayRegion; //434
	void(*)() *GetByteArrayRegion; //436
	void(*)() *GetCharArrayRegion; //438
	void(*)() *GetShortArrayRegion; //440
	void(*)() *GetIntArrayRegion; //442
	void(*)() *GetLongArrayRegion; //444
	void(*)() *GetFloatArrayRegion; //446
	void(*)() *GetDoubleArrayRegion; //448
	void(*)() *SetBooleanArrayRegion; //452
	void(*)() *SetByteArrayRegion; //454
	void(*)() *SetCharArrayRegion; //456
	void(*)() *SetShortArrayRegion; //458
	void(*)() *SetIntArrayRegion; //460
	void(*)() *SetLongArrayRegion; //462
	void(*)() *SetFloatArrayRegion; //464
	void(*)() *SetDoubleArrayRegion; //466
	void(*)() *RegisterNatives; //469
	void(*)() *UnregisterNatives; //471
	void(*)() *MonitorEnter; //472
	void(*)() *MonitorExit; //473
	void(*)() *GetJavaVM; //474
	void(*)() *GetStringRegion; //476
	void(*)() *GetStringUTFRegion; //477
	void(*)() *GetPrimitiveArrayCritical; //479
	void(*)() *ReleasePrimitiveArrayCritical; //480
	void(*)() *GetStringCritical; //482
	void(*)() *ReleaseStringCritical; //483
	void(*)() *NewWeakGlobalRef; //485
	void(*)() *DeleteWeakGlobalRef; //486
	void(*)() *ExceptionCheck; //488
	void(*)() *NewDirectByteBuffer; //490
	void(*)() *GetDirectBufferAddress; //491
	void(*)() *GetDirectBufferCapacity; //492
	void(*)() *GetObjectRefType; //495
};

struct JNIInvokeInterface; { //1051
	void *reserved0; //1052
	void *reserved1; //1053
	void *reserved2; //1054
	void(*)() *DestroyJavaVM; //1056
	void(*)() *AttachCurrentThread; //1057
	void(*)() *DetachCurrentThread; //1058
	void(*)() *GetEnv; //1059
	void(*)() *AttachCurrentThreadAsDaemon; //1060
};

struct timeval; { //25
	time_t tv_sec; //26
	suseconds_t tv_usec; //27
};

enum android_LogPriority; { //79
	ANDROID_LOG_UNKNOWN = 0,
	ANDROID_LOG_DEFAULT = 1,
	ANDROID_LOG_VERBOSE = 2,
	ANDROID_LOG_DEBUG = 3,
	ANDROID_LOG_INFO = 4,
	ANDROID_LOG_WARN = 5,
	ANDROID_LOG_ERROR = 6,
	ANDROID_LOG_FATAL = 7,
	ANDROID_LOG_SILENT = 8,
};

long int _getTime() //45
{
	struct timeval now; //47
}

void Java_de_gui_avatar_DemoRenderer_nativeInit(JNIEnv *env) //55
;

void Java_de_gui_avatar_DemoRenderer_nativeResize(JNIEnv *env, jobject thiz, jint w, jint h) //63
;

void Java_de_gui_avatar_DemoRenderer_nativeDone(JNIEnv *env) //71
;

void Java_de_gui_avatar_DemoRenderer_nativeAnimateScreenSize(JNIEnv *env, jobject thiz, jfloat jscale) //77
;

jint Java_de_gui_avatar_DemoRenderer_nativeLipSynchUri(JNIEnv *env, jobject thiz, jstring uri) //84
{
	int ret; //86
	const jbyte *utf8; //89
	char path[128]; //91
}

jint Java_de_gui_avatar_DemoRenderer_nativeLipSynchStart(JNIEnv *env, jobject thiz) //104
{
	int ret; //106
}

void Java_de_gui_avatar_AvatarView_nativeTouch(JNIEnv *env, jobject thiz, jfloat x, jfloat y) //119
;

void Java_de_gui_avatar_AvatarView_nativeTouchMove(JNIEnv *env, jobject thiz, jfloat x, jfloat y) //127
;

void Java_de_gui_avatar_AvatarView_nativeTouchFinish(JNIEnv *env) //135
;

void Java_de_gui_avatar_DemoRenderer_nativeRender(JNIEnv *env) //142
{
	long int curTime; //144
	_getTime(); //146
}

void Java_de_gui_avatar_AvatarView_nativeOnClick(JNIEnv *env, jobject thiz, jstring button) //157
{
	const char *action; //159
}

jbyteArray Java_de_gui_avatar_Avatar_decodeString( //164
				JNIEnv *env, //164
				jobject thiz, //165
				jstring inStr) //166
{
	jboolean jbool; //168
	_Bool bRet; //169
	jbyteArray result; //170
	const char *cpStr; //171
	int len; //172
	int outLen; //173
	jbyte *bpOutData; //174
	jbyte *dst; //175
	int i; //176
}

jstring Java_de_gui_avatar_Avatar_encodeData( //229
				JNIEnv *env, //229
				jobject thiz, //230
				jbyteArray inStr) //231
{
	jbyte *bpStr; //233
	jboolean b; //234
	jstring jsRet; //235
	int len; //236
	int outLen; //237
	_Bool bRet; //238
	char *cpOutStr; //239
}

void Java_de_gui_avatar_DemoRenderer_nativeLoadFaceNumber(JNIEnv *env, jobject thiz, jint x) //271
;

jint Java_de_gui_avatar_AvatarView_setFaceInfo( //277
				JNIEnv *env, //277
				jobject thiz, //278
				jstring newStr) //279
{
	int ret; //281
	const char *src; //282
}

long int sTimeOffset; //36

int sTimeOffsetInit; //37

long int sTimeStopped; //38

int sVoiceFd; //41

extern int gAppAlive; //29

extern unsigned int __page_size; //172

extern unsigned int __page_shift; //176

extern int gWindowWidth; //31

extern int gWindowHeight; //32

extern float gTouchX; //33

extern float gTouchY; //34

// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/app-android.c// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/sample.cpp

enum android_LogPriority; { //79
	ANDROID_LOG_UNKNOWN = 0,
	ANDROID_LOG_DEFAULT = 1,
	ANDROID_LOG_VERBOSE = 2,
	ANDROID_LOG_DEBUG = 3,
	ANDROID_LOG_INFO = 4,
	ANDROID_LOG_WARN = 5,
	ANDROID_LOG_ERROR = 6,
	ANDROID_LOG_FATAL = 7,
	ANDROID_LOG_SILENT = 8,
};

typedef struct { //37
	float x; //38
	float y; //39
} mpVector2; //40

typedef struct { //58
	float x; //59
	float y; //60
	float z; //61
} mpRotation; //62

typedef struct { //79
	float r; //80
	float g; //81
	float b; //82
	float a; //83
} mpColor; //84

typedef enum { //46
	MP_IMG_COMM_FACEZ = 0,
	MP_IMG_COMM_TRSFACT = 1,
	MP_IMG_ANIME_HAIRZ = 2,
	MP_IMG_LAST = 3,
} mpImageID; //51

typedef enum { //66
	MP_TEX_COMM_FACE = 0,
	MP_TEX_COMM_LIP_EYELASH = 1,
	MP_TEX_COMM_SHADOW_EYELASH = 2,
	MP_TEX_COMM_LOWER_TEETH = 3,
	MP_TEX_COMM_UPPER_TEETH = 4,
	MP_TEX_ANIME_HAIR = 5,
	MP_TEX_ANIME_REYE = 6,
	MP_TEX_ANIME_LEYE = 7,
	MP_TEX_ANIME_REYE_SHADOW = 8,
	MP_TEX_ANIME_LEYE_SHADOW = 9,
	MP_TEX_ANIME_REYE_REFLECT = 10,
	MP_TEX_ANIME_LEYE_REFLECT = 11,
	MP_TEX_ANIME_EYEBROW = 12,
	MP_TEX_PHOTO_EYE_BASE = 13,
	MP_TEX_PHOTO_PUPIL = 14,
	MP_TEX_PHOTO_IRIS = 15,
	MP_TEX_PHOTO_EYE_REFLECT = 16,
	MP_TEX_PHOTO_EYELID = 17,
	MP_TEX_LAST = 18,
} mpTexID; //86

typedef enum { //95
	MP_TEX_GLASSES_LENS = 0,
	MP_TEX_GLASSES_FRAME = 1,
	MP_TEX_GLASSES_SHADOW = 2,
	MP_TEX_GLASSES_REFRACT = 3,
	MP_TEX_GLASSES_LAST = 4,
} mpTexIDGlasses; //101

typedef enum { //103
	MP_TEX_GLASSESOPT_MIRROR = 0,
	MP_TEX_GLASSESOPT_COLOR = 1,
	MP_TEX_GLASSESOPT_LAST = 2,
} mpTexIDGlassesOpt; //107

typedef enum { //109
	MP_TEX_HAIR_RGB = 0,
	MP_TEX_HAIR_BGK = 1,
	MP_TEX_HAIR_FR = 2,
	MP_TEX_HAIR_LAST = 3,
} mpTexIDHair; //114

typedef enum { //116
	MP_TEX_HIGE_RGB = 0,
	MP_TEX_HIGE_LAST = 1,
} mpTexIDHige; //119

typedef enum { //183
	MP_NECK_X_ENABLE = 0,
	MP_NECK_Y_ENABLE = 1,
	MP_NECK_Z_ENABLE = 2,
	MP_NECK_X_DURATION_FACTOR = 3,
	MP_NECK_Y_DURATION_FACTOR = 4,
	MP_NECK_Z_DURATION_FACTOR = 5,
	MP_NECK_X_MAX_ROT = 6,
	MP_NECK_Y_MAX_ROT = 7,
	MP_NECK_Z_MAX_ROT = 8,
	MP_BLINK_ENABLE = 9,
	MP_BLINK_DURATION_FACTOR = 10,
	MP_BLINK_FREQS = 11,
	MP_BLINK_GAIN_FACTOR = 12,
	MP_PUPIL_ENABLE = 13,
	MP_PUPIL_DURATION_FACTOR = 14,
	MP_PUPIL_X_MAX = 15,
	MP_PUPIL_Y_MAX = 16,
	MP_EXPR_ENABLE = 17,
	MP_EXPR_DURATION_FACTOR = 18,
	MP_EXPR_BREATH_DURATION_FACTOR = 19,
	MP_EXPR_GAIN_FACTORS = 20,
	MP_BREATH_ENABLE = 21,
} mpUAnimParam; //206

typedef enum { //215
	MP_MODEL_TYPE = 0,
	MP_NUM_EXPR = 1,
} mpFaceParam; //218

typedef struct { //243
	int faceMeshXDiv; //244
	int faceMeshYDiv; //245
	int enableBlurEyelids; //246
	int enableAlphaFill; //247
} mpContext; //248

typedef struct { //257
	float gain; //258
	int bytesPerSec; //259
	int position; //260
	int length; //261
	unsigned char *buf; //262
} mpVoice; //263

typedef struct { //309
	float scale; //310
	mpRotation rot; //311
	mpVector2 offset; //312
} mpGlassesAdjust; //313

typedef struct tag_mpFace mpFace; //320

typedef struct tag_mpGlasses mpGlasses; //327

typedef struct tag_mpHair mpHair; //333

typedef struct tag_mpHige mpHige; //339

struct mpRenderingContext; { //24
	int dummy; //26
	int bEnableDepthTest; //28
	int texWidth; //30
	int texHeight; //31
	float theta; //34
	float axisx; //35
	float axisy; //36
	float axisz; //37
	float transx; //40
	float transy; //41
	float transz; //42
	float scalex; //45
	float scaley; //46
	float scalez; //47
};

typedef struct mpTexture { //55
	GLuint textureName; //57
} mpTexture; //58

typedef struct { //80
	float x; //81
	float y; //82
	float scale; //83
} ViewScale; //84

void Speech() //280
{
	{
		char voicestr[2048]; //282
	}
}

void CloseEye() //265
{
	{
		int flagCloseEye; //267
		float close; //269
	}
}

void Face() //426
;

void checkAction() //433
;

void loadFace(int index) //326
{
	{
		char facestr[2048]; //328
		char str[50]; //347
	}
}

void Hair() //161
{
	{
		char hair[2048]; //163
	}
}

void Glasses() //190
{
	{
		int lens; //193
		mpGlassesAdjust adjust; //194
		mpColor reflectCol; //199
		mpColor ShadowCol; //200
		mpColor color; //201
		char glasses[2048]; //203
	}
}

void Beard() //246
{
	{
		char beard[2048]; //248
	}
}

void DisplayExpression() //304
{
	{
		int i; //306
		float gains[128]; //307
	}
}

void CancelExpression() //314
{
	{
		int i; //316
		float gains[128]; //317
	}
}

void appInit() //89
{
	{
		mpContext ctxt; //97
		char comm[2048]; //104
	}
}

void appDeinit() //115
;

void appRender(long int tick, int width, int height) //128
{
	{
		int screen; //130
		int sx; //135
		int sy; //136
	}
}

void loadFaces(int x) //323
;

void appAction(const char *action) //456
;

void appTouch() //469
{
	{
		float gTouchX; //471
		float gTouchY; //472
		int gWindowWidth; //473
		int gWindowHeight; //474
		mpVector2 pos; //478
	}
}

void appTouchMove() //485
{
	{
		float gTouchX; //487
		float gTouchY; //488
		int gWindowWidth; //489
		int gWindowHeight; //490
		mpVector2 pos; //494
	}
}

void appTouchFinish() //501
{
	{
		mpVector2 pos; //505
	}
}

void appScale(float scale) //512
;

int setFaceInfo(const char *face) //521
;

void speakWav(const char *buf) //532
;

extern int gAppAlive; //40

struct mpRenderingContext rc; //40

mpFace *aPhotoFace; //42

mpHair *aHair; //43

mpGlasses *aGlasses; //44

mpHige *aBeard; //45

mpVoice voice; //46

struct mpTexture *texPhoto[18]; //48

struct mpTexture *texHair[3]; //49

struct mpTexture *texGlasses[4]; //50

mpTexture *glassesTexOpt[2]; //51

struct mpTexture *texBeard[1]; //52

unsigned char *imgPhoto[3]; //54

int numExpr; //56

int clicked; //58

char g_currentAction[64]; //59

int g_touched; //60

int idxExpr; //61

const const char *localDir; //63

int face_idx; //65

int face_max; //66

int hair_idx; //67

const int hair_max; //68

char usrFace[2048]; //69

char localVoiceFile[2048]; //71

float g_scale; //74

ViewScale sViewScale; //85

// /Users/robertoandrade/Documents/workspace.guide/GuideAvatar\jni/sample.cpp