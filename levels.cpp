#include "levels.h"
#include "objects.h"
#include <fstream>
#include <stdio.h>

extern LQueue* queue;
extern LPlayer* player;

bool LoadLevel(int lev)
{
	char pom[30];
	char walls[15];
	double x1,y1,z1,w1,x2,y2,z2,w2,p;
	sprintf(pom,"level%d.dat",lev);
	std::ifstream fin(pom);

	if(fin.fail()) return false;
	
	queue=new LQueue;
	player=new LPlayer;
	queue->AddObject(player);
	while(!fin.eof())
	{
		fin>>pom;
		x1=atof(pom);
		fin>>pom;
		y1=atof(pom);
		fin>>pom;
		z1=atof(pom);
		fin>>pom;
		w1=atof(pom);
		fin>>pom;
		x2=atof(pom);
		fin>>pom;
		y2=atof(pom);
		fin>>pom;
		z2=atof(pom);
		fin>>pom;
		w2=atof(pom);
		if(x1>x2)
		{
			p=x1;
			x1=x2;
			x2=p;
		}
		if(y1>y2)
		{
			p=y1;
			y1=y2;
			y2=p;
		}
		if(z1>z2)
		{
			p=z1;
			z1=z2;
			z2=p;
		}
		if(w1>w2)
		{
			p=w1;
			w1=w2;
			w2=p;
		}
		fin>>walls;
		for(unsigned i=0;i<strlen(walls);i++)
		{
			switch(walls[i])
			{
			case 'x':
				queue->AddObject(new LWall(vector4d(x1,y1,z1,w1),vector4d(x1,y2,z2,w2)));
				break;
			case 'X':
				queue->AddObject(new LWall(vector4d(x2,y1,z1,w1),vector4d(x2,y2,z2,w2)));
				break;
			case 'y':
				queue->AddObject(new LWall(vector4d(x1,y1,z1,w1),vector4d(x2,y1,z2,w2)));
				break;
			case 'Y':
				queue->AddObject(new LWall(vector4d(x1,y2,z1,w1),vector4d(x2,y2,z2,w2)));
				break;
			case 'z':
				queue->AddObject(new LWall(vector4d(x1,y1,z1,w1),vector4d(x2,y2,z1,w2)));
				break;
			case 'Z':
				queue->AddObject(new LWall(vector4d(x1,y1,z2,w1),vector4d(x2,y2,z2,w2)));
				break;
			case 'w':
				queue->AddObject(new LWall(vector4d(x1,y1,z1,w1),vector4d(x2,y2,z2,w1)));
				break;
			case 'W':
				queue->AddObject(new LWall(vector4d(x1,y1,z1,w2),vector4d(x2,y2,z2,w2)));
				break;
			case 'T':
				queue->AddObject(new LTarget(vector4d(x1,y1,z1,w1),vector4d(x2,y2,z2,w2)));
				break;
			}
		}
	}
	fin.close();
	return true;
}