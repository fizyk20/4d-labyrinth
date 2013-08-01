#include "objects.h"

LPlayer::LPlayer()
{
	type=PLAYER;
	player_up=vector4d(0.0,1.0,0.0,0.0);
	player_front=vector4d(0.0,0.0,-1.0,0.0);
	player_right=vector4d(1.0,0.0,0.0,0.0);
	player_ana=vector4d(0.0,0.0,0.0,1.0);
	orient.LoadIdentity();
	pos=vector4d(0.0,0.0,0.0,0.0);
}

LPlayer::~LPlayer()
{
}

void LPlayer::Go(vector4d vec)
{
	pos=pos+player_right*vec.x;
	pos=pos+player_up*vec.y;
	pos=pos-player_front*vec.z;
	pos=pos+player_ana*vec.w;
}

void LPlayer::RotateXY(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_right,player_up,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::RotateXZ(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_right,player_front,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::RotateXW(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_right,player_ana,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::RotateYZ(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_up,player_front,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::RotateYW(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_up,player_ana,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::RotateZW(double phi)
{
	matrix4d mat;
	mat=RotationMatrix(player_front,player_ana,phi);
	player_up=mat*player_up;
	player_right=mat*player_right;
	player_front=mat*player_front;
	player_ana=mat*player_ana;
	orient=mat*orient;
	player_up.Normalize();
	player_right.Normalize();
	player_front.Normalize();
	player_ana.Normalize();
}

void LPlayer::doYourJob()
{
	matrix4d mat;
	
	mat=TranslationMatrix(pos)*orient;
	graph->PushMatrix();
	graph->ApplyMatrix(mat);
	graph->Color(0.6,0.6,0.0);
	graph->Tesseract(2*SIZE);
	graph->PopMatrix();
	mat.LoadIdentity();
	mat=orient;
	graph->camera->ApplyMatrix(mat);
	graph->camera->Translate(pos+orient*vector4d(0.0,0.7,-3.0,0.0));
}

//---------------------------

#define Tetrahedron graph->Tetrahedron
#define CUBE(a,b,c,d,e,f,g,h) Tetrahedron(ver[d],ver[a],ver[e],ver[b]); \
	Tetrahedron(ver[b],ver[e],ver[g],ver[f]); \
	Tetrahedron(ver[d],ver[g],ver[h],ver[e]); \
	Tetrahedron(ver[b],ver[c],ver[d],ver[g]); \
	Tetrahedron(ver[b],ver[e],ver[d],ver[g]);

LWall::LWall(vector4d a,vector4d b)
{
	type=WALL;
	int i=0;
	double pom;
	if((a.x-b.x>-0.001)&&(a.x-b.x<0.001))		//we switch coordinates so that size in W direction is 0
	{
		i=1;
		pom=a.x;
		a.x=a.w;
		a.w=pom;
		pom=b.x;
		b.x=b.w;
		b.w=pom;
	}
	if((a.y-b.y>-0.001)&&(a.y-b.y<0.001))
	{
		i=2;
		pom=a.y;
		a.y=a.w;
		a.w=pom;
		pom=b.y;
		b.y=b.w;
		b.w=pom;
	}
	if((a.z-b.z>-0.001)&&(a.z-b.z<0.001))
	{
		i=3;
		pom=a.z;
		a.z=a.w;
		a.w=pom;
		pom=b.z;
		b.z=b.w;
		b.w=pom;
	}
	ver[0]=a;
	ver[1]=vector4d(a.x,a.y,b.z,a.w);
	ver[2]=vector4d(a.x,b.y,b.z,a.w);
	ver[3]=vector4d(a.x,b.y,a.z,a.w);
	ver[4]=vector4d(b.x,a.y,a.z,a.w);		//creating cube vertices
	ver[5]=vector4d(b.x,a.y,b.z,a.w);
	ver[6]=b;
	ver[7]=vector4d(b.x,b.y,a.z,a.w);
	int j;
	for(j=0;j<8;j++)
		switch(i)
		{
		case 1:
			pom=ver[j].x;
			ver[j].x=ver[j].w;
			ver[j].w=pom;
			break;
		case 2:
			pom=ver[j].y;
			ver[j].y=ver[j].w;		//switching coords back
			ver[j].w=pom;
			break;
		case 3:
			pom=ver[j].z;
			ver[j].z=ver[j].w;
			ver[j].w=pom;
			break;
		}
	middle=ver[0]+ver[6];
	middle=middle*0.5;
	width=fabs(ver[0].x-ver[6].x)*0.5;
	height=fabs(ver[0].y-ver[6].y)*0.5;
	depth=fabs(ver[0].z-ver[6].z)*0.5;
	something=fabs(ver[0].w-ver[6].w)*0.5;
}

LWall::~LWall()
{
}

bool LWall::collision()
{
	bool d1,d2,d3,d4;
	double x=player->GetX();
	double y=player->GetY();
	double z=player->GetZ();
	double w=player->GetW();
	d1=fabs(x-middle.x)<(width+SIZE);
	d2=fabs(y-middle.y)<(height+SIZE);
	d3=fabs(z-middle.z)<(depth+SIZE);
	d4=fabs(w-middle.w)<(something+SIZE);
	return d1&&d2&&d3&&d4;
}

void LWall::doYourJob()
{
	graph->ColorA(0.5,0.5,0.5,0.2);
	CUBE(0,1,2,3,4,5,6,7);
}

//-----------------------------------------

LTarget::LTarget(vector4d a,vector4d b)
{
	type=TARGET;
	pos=(a+b)/2;
	size=fabs(a.x-b.x);
}

LTarget::~LTarget()
{
}

void LTarget::doYourJob()
{
	matrix4d mat;
	
	mat=TranslationMatrix(pos);
	graph->PushMatrix();
	graph->ApplyMatrix(mat);
	graph->ColorA(0.0,0.2,1.0,0.4);
	graph->Tesseract(size);
	graph->PopMatrix();
}

bool LTarget::collision()
{
	bool d1,d2,d3,d4;
	double x=player->GetX();
	double y=player->GetY();
	double z=player->GetZ();
	double w=player->GetW();
	d1=fabs(x-pos.x)<(size/2+SIZE);
	d2=fabs(y-pos.y)<(size/2+SIZE);
	d3=fabs(z-pos.z)<(size/2+SIZE);
	d4=fabs(w-pos.w)<(size/2+SIZE);
	return d1&&d2&&d3&&d4;
}

//-----------------------------------------

LQueue::LQueue()
{
	n=0;
	queue=NULL;
}

LQueue::~LQueue()
{
	n=0;
	if(queue!=NULL) delete[] queue;
}

void LQueue::AddObject(Object* x)
{
	Object** b=new Object*[n+1];
	n++;
	for(int i=0;i<n-1;i++)
		b[i]=queue[i];
	b[n-1]=x;
	delete[] queue;
	queue=b;
	b=NULL;
}

void LQueue::doYourJob()
{
	for(int i=0;i<n;i++)
		queue[i]->doYourJob();
}

bool LQueue::collision()
{
	for(int i=0;i<n;i++)
		if(queue[i]->type==WALL)
			if(((LWall*)queue[i])->collision()) return true;
	return false;
}

bool LQueue::win()
{
	for(int i=0;i<n;i++)
		if(queue[i]->type==TARGET)
			if(((LTarget*)queue[i])->collision()) return true;
	return false;
}