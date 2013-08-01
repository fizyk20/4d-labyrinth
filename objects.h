#ifndef __OBIEKTY_LABIRYNT4D__
#define __OBIEKTY_LABIRYNT4D__

#include "Graph4D.h"
#pragma comment(lib,"Graph4D.lib")

extern Graph4D* graph;

//an abstract object class
class Object
{
public:
	int type;

	Object() {}
	virtual ~Object() {}
	virtual void doYourJob() {}
};

#define PLAYER 1
#define WALL 2
#define TARGET 3

#define SIZE 0.2

class LPlayer:public Object
{
	vector4d player_up,player_front,player_right,player_ana;
	matrix4d orient;
	vector4d pos;
public:
	LPlayer();
	~LPlayer();
	void doYourJob();
	void Go(vector4d);
	void RotateXY(double);  //rotates AROUND XY plane
	void RotateXZ(double);	//etc.
	void RotateXW(double);
	void RotateYZ(double);
	void RotateYW(double);
	void RotateZW(double);
	double GetX() {return pos.x;}
	double GetY() {return pos.y;}
	double GetZ() {return pos.z;}
	double GetW() {return pos.w;}
};

extern LPlayer* player;

class LWall:public Object
{
	vector4d ver[8];
	vector4d middle;
	double width,height,depth,something;
public:
	LWall(vector4d a,vector4d b);
	~LWall();
	void doYourJob();
	bool collision();
};

class LTarget:public Object
{
	vector4d pos;
	double size;
public:
	LTarget(vector4d a,vector4d b);
	~LTarget();
	void doYourJob();
	bool collision();
};

class LQueue
{
	Object** queue;
	int n;
public:
	LQueue();
	~LQueue();
	void AddObject(Object*);
	void doYourJob();
	bool collision();
	bool win();
};

#endif