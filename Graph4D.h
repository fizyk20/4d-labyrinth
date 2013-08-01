#ifndef __GRAPH_4D__
#define __GRAPH_4D__

#include <windows.h>
#include <math.h>
#include <gl/gl.h>
#include <gl/glu.h>
#pragma comment(lib,"opengl32.lib")
#pragma comment(lib,"glu32.lib")

#ifndef GRAPHDLL
#define GRAPHDLL __declspec(dllimport)
#endif

#define pi 3.1415926535

//primitives
#define PRIM_INVALID -2
#define PRIM_QUAD -1
#define PRIM_NONE 0
#define PRIM_POINT 1
#define PRIM_LINE 2
#define PRIM_TRIANGLE 3
#define PRIM_TETRA 4

class GRAPHDLL vector4d
{
public:
	double x,y,z,w;

	vector4d();
	vector4d(double,double,double,double);
	~vector4d();
	vector4d operator+ (const vector4d&);
	vector4d operator- (const vector4d&);
	double operator% (const vector4d&);
	vector4d operator* (const double&);
	vector4d operator/ (const double&);
	bool operator== (const vector4d&);
	void Normalize();
	double Len();
};

class GRAPHDLL matrix4d
{
	double a[5][5];
public:
	matrix4d();
	~matrix4d();
	void LoadIdentity();
	void SetValue(int,int,double);
	matrix4d operator* (const matrix4d&);
	vector4d operator* (const vector4d&);
	matrix4d operator= (const matrix4d&);
};

typedef vector4d point4d;

struct vertex4d
{
	point4d pt;
	double r,g,b,a;
	bool operator==(const vertex4d&);
};

struct primitive
{
	short type;
	vertex4d vert[4];
};

class GRAPHDLL PrimBuffer
{
	unsigned long max_prims;
	unsigned long num_prims;
	primitive* buffer;
public:
	PrimBuffer();
	~PrimBuffer();
	void ReAlloc();
	int AddPrim(primitive);
	void DeletePrim(int);
	primitive GetPrim(int);
	unsigned long GetNumPrims();
};

#define VEC_LOOKAT 1
#define VEC_UP 2
#define VEC_RIGHT 3
#define VEC_NORMAL 4
#define VEC_LOCATION 5

class GRAPHDLL Camera
{
	vector4d lookat;
	vector4d up;
	vector4d right;
	vector4d normal;
	point4d location;
public:
	Camera();
	~Camera();
	void LoadIdentity();
	void ApplyMatrix(matrix4d);
	void Translate(vector4d);
	void RotateXY(double);
	void RotateXZ(double);
	void RotateXW(double);
	void RotateYZ(double);
	void RotateYW(double);
	void RotateZW(double);
	void Rotate(vector4d,vector4d,double);
	vector4d GetVector(int);
	friend class Graph4D;
};

#define MAX_MATRIX 21000

class MatrixBuffer
{
	matrix4d cur_matrix;
	matrix4d matrix_stack[MAX_MATRIX];
	int current;
public:
	MatrixBuffer();
	~MatrixBuffer();
	void PushMatrix();
	void PopMatrix();
	void MultiplyMatrix(matrix4d);
	matrix4d GetMatrix();
	void LoadIdentity();
	void ZeroStack();
};

struct Space
{
	vector4d normal;
	double e;
};

#define MODE_SLICE 1
#define MODE_PROJ 2		//projection mode not yet implemented

class GRAPHDLL Graph4D
{
	PrimBuffer* buffer;
	PrimBuffer* local_buffer;
	MatrixBuffer* m_buffer;
	HGLRC hrc;
	HDC hdc;
	double r,g,b,a;

	int mode;

	void SetBlending(double);
	primitive Intersect(primitive,Space);
	void SetupPixelFormat(HDC);
public:
	Camera* camera;

	Graph4D(HDC);
	~Graph4D();
	vector4d CalculateLocal(vector4d);
	
	void Render();
	
	void EnableLighting(bool);
	void EnableTwoSide(bool);
	void LightDir(double,double,double);

	void AddPrimitive(primitive);
	void ApplyMatrix(matrix4d);
	void Translate(vector4d);
	void RotateXY(double);
	void RotateXZ(double);
	void RotateXW(double);
	void RotateYZ(double);
	void RotateYW(double);
	void RotateZW(double);
	void Rotate(vector4d,vector4d,double);
	void PushMatrix();
	void PopMatrix();

	void HandleWMSize(LPARAM);
	
	void Color(double,double,double);
	void ColorA(double,double,double,double);
	void Point(point4d);
	void Line(point4d,point4d);
	void Triangle(point4d,point4d,point4d);
	void Tetrahedron(point4d,point4d,point4d,point4d);
	void PointVertex(vertex4d);
	void LineVertex(vertex4d,vertex4d);
	void TriangleVertex(vertex4d,vertex4d,vertex4d);
	void TetrahedronVertex(vertex4d,vertex4d,vertex4d,vertex4d);
	void Cube(double);
	void Tesseract(double);
};

vertex4d GRAPHDLL Vertex(double,double,double,double,double,double,double);		//create colored vertex
vertex4d GRAPHDLL VertexA(double x,double y,double z,double w,double r,double g,double b,double a);		//create RGBA vertex
vector4d CrossProduct3(vector4d,vector4d);
vector4d CrossProduct4(vector4d,vector4d,vector4d);
matrix4d GRAPHDLL RotationMatrix(vector4d,vector4d,double);
matrix4d GRAPHDLL TranslationMatrix(vector4d);
matrix4d GRAPHDLL ScaleMatrix(double,double,double,double);

#endif