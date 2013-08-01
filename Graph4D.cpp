#define GRAPHDLL __declspec(dllexport)
#include "Graph4D.h"

//version 0.7.8

//---------------------------------------classes----------------------------------

//---------------------------------------vector4d-------------------------------

vector4d::vector4d()
{
	x=y=z=w=0.0;
}

vector4d::vector4d(double x1,double y1,double z1,double w1)
{
	x=x1;
	y=y1;
	z=z1;
	w=w1;
}

vector4d::~vector4d()
{
}

vector4d vector4d::operator +(const vector4d& arg)
{
	vector4d vec(x+arg.x,y+arg.y,z+arg.z,w+arg.w);
	return vec;
}

vector4d vector4d::operator -(const vector4d& arg)
{
	vector4d vec(x-arg.x,y-arg.y,z-arg.z,w-arg.w);
	return vec;
}

vector4d vector4d::operator *(const double& arg)
{
	vector4d vec(arg*x,arg*y,arg*z,arg*w);
	return vec;
}

vector4d vector4d::operator /(const double& arg)
{
	vector4d vec(x/arg,y/arg,z/arg,w/arg);
	return vec;
}

bool vector4d::operator ==(const vector4d& arg)
{
	bool xx,yy,zz,ww;
	xx=(x<arg.x+0.0001&&x>arg.x-0.0001);
	yy=(y<arg.y+0.0001&&y>arg.y-0.0001);
	zz=(z<arg.z+0.0001&&z>arg.z-0.0001);
	ww=(w<arg.w+0.0001&&w>arg.w-0.0001);
	return(xx&&yy&&zz&&ww);
}

double vector4d::operator %(const vector4d& arg)
{
	double a=x*arg.x+y*arg.y+z*arg.z+w*arg.w;
	return a;
}

void vector4d::Normalize()
{
	double l=sqrt(x*x+y*y+z*z+w*w);
	x=x/l;
	y=y/l;
	z=z/l;
	w=w/l;
}

double vector4d::Len()
{
	return sqrt(x*x+y*y+z*z+w*w);
}

//---------------------------------------vertex4d-------------------------------

bool vertex4d::operator ==(const vertex4d& arg)
{
	return (pt==arg.pt);
}

//---------------------------------------matrix4d-------------------------------

matrix4d::matrix4d()
{
	int i,j;
	for(i=0;i<5;i++)
		for(j=0;j<5;j++)
			a[i][j]=0.0;
}

matrix4d::~matrix4d()
{
}

void matrix4d::LoadIdentity()
{
	int i,j;
	for(i=0;i<5;i++)
		for(j=0;j<5;j++)
			a[i][j]=(j==i)?1.0:0.0;
}

void matrix4d::SetValue(int i,int j,double x)
{
	a[i][j]=x;
}

matrix4d matrix4d::operator *(const matrix4d& arg)
{
	matrix4d temp;
	double sum;

	for(int i=0;i<5;i++)
		for(int j=0;j<5;j++)
		{
			sum=0.0;
			for(int n=0;n<5;n++)
				sum+=a[i][n]*arg.a[n][j];
			temp.a[i][j]=sum;
		}
	return temp;
}

vector4d matrix4d::operator *(const vector4d& arg)
{
	vector4d temp;
	temp.x=a[0][0]*arg.x+a[0][1]*arg.y+a[0][2]*arg.z+a[0][3]*arg.w+a[0][4];
	temp.y=a[1][0]*arg.x+a[1][1]*arg.y+a[1][2]*arg.z+a[1][3]*arg.w+a[1][4];
	temp.z=a[2][0]*arg.x+a[2][1]*arg.y+a[2][2]*arg.z+a[2][3]*arg.w+a[2][4];
	temp.w=a[3][0]*arg.x+a[3][1]*arg.y+a[3][2]*arg.z+a[3][3]*arg.w+a[3][4];
	return temp;
}

matrix4d matrix4d::operator =(const matrix4d& arg)
{
	int i,j;
	for(i=0;i<5;i++)
		for(j=0;j<5;j++)
			a[i][j]=arg.a[i][j];
	return(*this);
}

//---------------------------------------MatrixBuffer---------------------------

MatrixBuffer::MatrixBuffer()
{
	cur_matrix.LoadIdentity();
	current=-1;
}

MatrixBuffer::~MatrixBuffer()
{
}

void MatrixBuffer::PopMatrix()
{
	if(current>-1)
	{
		cur_matrix=matrix_stack[current];
		current--;
	}
}

void MatrixBuffer::PushMatrix()
{
	current++;
	matrix_stack[current]=cur_matrix;
}

void MatrixBuffer::MultiplyMatrix(matrix4d mat)
{
	cur_matrix=mat*cur_matrix;
}

matrix4d MatrixBuffer::GetMatrix()
{
	return cur_matrix;
}

void MatrixBuffer::LoadIdentity()
{
	cur_matrix.LoadIdentity();
}

void MatrixBuffer::ZeroStack()
{
	current=-1;
}

//---------------------------------------PrimBuffer-----------------------------

#define MAX_PRIMS 1000

PrimBuffer::PrimBuffer()
{
	max_prims=MAX_PRIMS;
	num_prims=0;
	buffer=new primitive[MAX_PRIMS];
}

PrimBuffer::~PrimBuffer()
{
	if(buffer!=NULL) delete[] buffer;
}

int PrimBuffer::AddPrim(primitive prim)
{
	if(num_prims+1>max_prims)
	{
		max_prims+=MAX_PRIMS;
		ReAlloc();
	}
	buffer[num_prims]=prim;
	num_prims++;
	return (num_prims-1);
}

void PrimBuffer::ReAlloc()
{
	primitive* temp=new primitive[max_prims];
	unsigned long i;
	for(i=0;i<num_prims;i++)
		temp[i]=buffer[i];
	if(buffer!=NULL) delete[] buffer;
	buffer=temp;
	temp=NULL;
}

void PrimBuffer::DeletePrim(int ind)
{
	for(unsigned i=ind;i<num_prims-1;i++)
		buffer[i]=buffer[i+1];
	num_prims--;
	if(num_prims<=max_prims-MAX_PRIMS)
	{
		max_prims-=MAX_PRIMS;
		ReAlloc();
	}
}

primitive PrimBuffer::GetPrim(int ind)
{
	return buffer[ind];
}

unsigned long PrimBuffer::GetNumPrims()
{
	return num_prims;
}

//---------------------------------------Camera---------------------------------

Camera::Camera()
{
	lookat=vector4d(0.0,0.0,-1.0,0.0);
	up=vector4d(0.0,1.0,0.0,0.0);
	right=vector4d(1.0,0.0,0.0,0.0);
	normal=vector4d(0.0,0.0,0.0,1.0);
	location=vector4d(0.0,0.0,0.0,0.0);
}

Camera::~Camera()
{
}

void Camera::Translate(vector4d arg)
{
	location=location+arg;
}

void Camera::RotateXY(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,1,-sin(phi));
	mat.SetValue(1,0,sin(phi));
	mat.SetValue(1,1,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::RotateXZ(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,2,-sin(phi));
	mat.SetValue(2,0,sin(phi));
	mat.SetValue(2,2,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::RotateXW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,3,-sin(phi));
	mat.SetValue(3,0,sin(phi));
	mat.SetValue(3,3,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::RotateYZ(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(1,1,cos(phi));
	mat.SetValue(1,2,-sin(phi));
	mat.SetValue(2,1,sin(phi));
	mat.SetValue(2,2,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::RotateYW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(1,1,cos(phi));
	mat.SetValue(1,3,-sin(phi));
	mat.SetValue(3,1,sin(phi));
	mat.SetValue(3,3,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::RotateZW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(2,2,cos(phi));
	mat.SetValue(2,3,-sin(phi));
	mat.SetValue(3,2,sin(phi));
	mat.SetValue(3,3,cos(phi));
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}


void Camera::Rotate(vector4d n1,vector4d n2,double phi)
{
	matrix4d mat;
	mat=RotationMatrix(n1,n2,phi);
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

void Camera::LoadIdentity()
{
	lookat=vector4d(0.0,0.0,-1.0,0.0);
	up=vector4d(0.0,1.0,0.0,0.0);
	right=vector4d(1.0,0.0,0.0,0.0);
	normal=vector4d(0.0,0.0,0.0,1.0);
	location=vector4d(0.0,0.0,0.0,0.0);
}

void Camera::ApplyMatrix(matrix4d mat)
{
	lookat=mat*lookat;
	up=mat*up;
	right=mat*right;
	normal=mat*normal;
	lookat.Normalize();
	up.Normalize();
	right.Normalize();
	normal.Normalize();
}

vector4d Camera::GetVector(int ktory)
{
	vector4d blad;
	switch(ktory)
	{
	case VEC_LOOKAT:
		return lookat;
	case VEC_UP:
		return up;
	case VEC_RIGHT:
		return right;
	case VEC_NORMAL:
		return normal;
	case VEC_LOCATION:
		return location;
	default:
		return blad;
	}
}

//---------------------------------------Graph4D--------------------------------

Graph4D::Graph4D(HDC hdc1)
{
	camera=new Camera;
	buffer=new PrimBuffer;
	m_buffer=new MatrixBuffer;
	hdc=hdc1;
	SetupPixelFormat(hdc);
	hrc=wglCreateContext(hdc);
	wglMakeCurrent(hdc,hrc);
//setting default light options
	float p[4];
	p[0]=p[1]=p[2]=0.8f;
	p[3]=1.0f;
	EnableLighting(true);
	glEnable(GL_LIGHT0);
	glLightfv(GL_LIGHT0,GL_DIFFUSE,p);
	p[0]=p[1]=p[2]=0.4f;
	glLightfv(GL_LIGHT0,GL_AMBIENT,p);
	p[0]=p[3]=0.0f;
	p[1]=p[2]=-0.707f;
	glLightfv(GL_LIGHT0,GL_POSITION,p);
	glShadeModel(GL_SMOOTH);
	EnableTwoSide(false);
	a=1.0;
}

void Graph4D::SetupPixelFormat(HDC hdc)
{
	int nPixelFormat;
	static PIXELFORMATDESCRIPTOR pfd= {
		sizeof(PIXELFORMATDESCRIPTOR),
		1,
		PFD_DRAW_TO_WINDOW|PFD_SUPPORT_OPENGL|PFD_DOUBLEBUFFER,
		PFD_TYPE_RGBA,
		32,
		0,0,0,0,0,0,0,0,0,0,0,0,0,
		16,0,0,
		PFD_MAIN_PLANE,0,0,0,0};
	nPixelFormat=ChoosePixelFormat(hdc,&pfd);
	SetPixelFormat(hdc,nPixelFormat,&pfd);
}

Graph4D::~Graph4D()
{
	wglMakeCurrent(hdc,NULL);
	wglDeleteContext(hrc);
	if(camera!=NULL) delete camera;
	if(buffer!=NULL) delete buffer;
	if(local_buffer!=NULL) delete local_buffer;
	if(m_buffer!=NULL) delete m_buffer;
}

void Graph4D::PushMatrix()
{
	m_buffer->PushMatrix();
}

void Graph4D::PopMatrix()
{
	m_buffer->PopMatrix();
}

void Graph4D::AddPrimitive(primitive prim)
{
	int i;
	for(i=0;i<prim.type;i++)
		prim.vert[i].pt=m_buffer->GetMatrix()*prim.vert[i].pt;
	buffer->AddPrim(prim);
}

void Graph4D::ApplyMatrix(matrix4d mat)
{
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::Translate(vector4d arg)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,4,arg.x);
	mat.SetValue(1,4,arg.y);
	mat.SetValue(2,4,arg.z);
	mat.SetValue(3,4,arg.w);
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateXY(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,1,-sin(phi));
	mat.SetValue(1,0,sin(phi));
	mat.SetValue(1,1,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateXZ(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,2,-sin(phi));
	mat.SetValue(2,0,sin(phi));
	mat.SetValue(2,2,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateXW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,cos(phi));
	mat.SetValue(0,3,-sin(phi));
	mat.SetValue(3,0,sin(phi));
	mat.SetValue(3,3,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateYZ(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(1,1,cos(phi));
	mat.SetValue(1,2,-sin(phi));
	mat.SetValue(2,1,sin(phi));
	mat.SetValue(2,2,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateYW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(1,1,cos(phi));
	mat.SetValue(1,3,-sin(phi));
	mat.SetValue(3,1,sin(phi));
	mat.SetValue(3,3,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::RotateZW(double phi)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(2,2,cos(phi));
	mat.SetValue(2,3,-sin(phi));
	mat.SetValue(3,2,sin(phi));
	mat.SetValue(3,3,cos(phi));
	m_buffer->MultiplyMatrix(mat);
}

void Graph4D::Rotate(vector4d n1,vector4d n2,double phi)
{
	matrix4d mat;
	mat=RotationMatrix(n1,n2,phi);
	m_buffer->MultiplyMatrix(mat);
}

primitive Graph4D::Intersect(primitive prim,Space space)
{
	primitive temp,prim1,prim2,prim3,prim4,prim5,prim6;
	temp.type=PRIM_NONE;
	int i,j;
	bool b;

	double a;
	a=space.normal%prim.vert[0].pt+space.e;
	for(i=1;i<prim.type;i++)
		if((space.normal%prim.vert[i].pt+space.e)*a<0.0) break;
	if(i==prim.type) return temp;	//if all vertices are on the same side of the slice, return empty primitive

	switch(prim.type)
	{
	case PRIM_POINT:
		if(space.normal%prim.vert[0].pt+space.e==0)
			temp=prim;
		break;
	case PRIM_LINE:
		double scal1,scal2;
		scal1=space.normal%prim.vert[0].pt+space.e;
		scal2=space.normal%prim.vert[1].pt+space.e;
		if(scal1*scal2<0)
		{
			vector4d pt1,pt2,vec,inter;
			pt1=prim.vert[0].pt;
			pt2=prim.vert[1].pt;
			vec=pt2-pt1;
			double a;
			a=-(space.normal%pt1+space.e);
			a/=space.normal%vec;
			inter=pt1+(vec*a);
			temp.type=PRIM_POINT;
			temp.vert[0].pt=inter;
			double len1,len2;
			len1=(inter-pt1).Len();
			len2=(pt2-pt1).Len();
			temp.vert[0].r=(prim.vert[1].r-prim.vert[0].r)*(len1/len2)+prim.vert[0].r;
			temp.vert[0].g=(prim.vert[1].g-prim.vert[0].g)*(len1/len2)+prim.vert[0].g;
			temp.vert[0].b=(prim.vert[1].b-prim.vert[0].b)*(len1/len2)+prim.vert[0].b;
			temp.vert[0].a=(prim.vert[1].a-prim.vert[0].a)*(len1/len2)+prim.vert[0].a;
			return temp;
		}
		if(scal1*scal2==0)
		{
			if(scal1==0&&scal2==0)
				temp=prim;
			else
			{
				temp.type=PRIM_POINT;
				if(scal1==0)
					temp.vert[0]=prim.vert[0];
				else
					temp.vert[0]=prim.vert[1];
			}
		}
		break;
	case PRIM_TRIANGLE:
		prim1.type=prim2.type=prim3.type=PRIM_LINE;
		prim1.vert[0]=prim2.vert[0]=prim.vert[0];
		prim1.vert[1]=prim3.vert[0]=prim.vert[1];
		prim2.vert[1]=prim3.vert[1]=prim.vert[2];
		prim1=Intersect(prim1,space);
		prim2=Intersect(prim2,space);
		prim3=Intersect(prim3,space);

		temp=prim1;
		for(i=0;i<prim2.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim2.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim2.vert[i];
				temp.type++;
			}
		}
		for(i=0;i<prim3.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim3.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim3.vert[i];
				temp.type++;
			}
		}
		if(temp.type>PRIM_TRIANGLE)
			temp.type=PRIM_INVALID;
		break;
	case PRIM_TETRA:
		prim1.type=prim2.type=prim3.type=prim4.type=prim5.type=prim6.type=PRIM_LINE;
		prim1.vert[0]=prim2.vert[0]=prim3.vert[0]=prim.vert[0];
		prim2.vert[1]=prim4.vert[0]=prim6.vert[0]=prim.vert[1];
		prim1.vert[1]=prim5.vert[0]=prim6.vert[1]=prim.vert[2];
		prim4.vert[1]=prim5.vert[1]=prim3.vert[1]=prim.vert[3];
		prim1=Intersect(prim1,space);
		prim2=Intersect(prim2,space);
		prim3=Intersect(prim3,space);
		prim4=Intersect(prim4,space);
		prim5=Intersect(prim5,space);
		prim6=Intersect(prim6,space);
		temp=prim1;
		for(i=0;i<prim2.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim2.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim2.vert[i];
				temp.type++;
			}
		}
		for(i=0;i<prim3.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim3.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim3.vert[i];
				temp.type++;
			}
		}
		for(i=0;i<prim4.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim4.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim4.vert[i];
				temp.type++;
			}
		}
		for(i=0;i<prim5.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim5.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim5.vert[i];
				temp.type++;
			}
		}
		for(i=0;i<prim6.type;i++)
		{
			b=false;
			for(j=0;j<temp.type;j++)
				if(temp.vert[j]==prim6.vert[i]) b=true;
			if(!b)
			{
				temp.vert[temp.type]=prim6.vert[i];
				temp.type++;
			}
		}
		if(temp.type==PRIM_TETRA)
		{
			vector4d x1,y1,v1;
			double k[3],pom;
			x1=temp.vert[1].pt-temp.vert[0].pt;
			y1=temp.vert[2].pt-temp.vert[0].pt;
			v1=temp.vert[3].pt-temp.vert[0].pt;
			x1.Normalize();
			y1.Normalize();
			k[0]=acos(x1%y1);
			k[1]=acos((x1%v1)/v1.Len());
			k[2]=acos((y1%v1)/v1.Len());
			if(k[0]>k[1])
			{
				pom=k[1];
				k[1]=k[0];
				k[0]=pom;
			}
			if(k[1]>k[2])
			{
				pom=k[2];
				k[2]=k[1];
				k[1]=pom;
			}
			if(k[0]+k[1]>k[2]-0.0001&&k[0]+k[1]<k[2]+0.0001) temp.type=PRIM_QUAD;
		}
		if(temp.type>PRIM_TETRA)
			temp.type=PRIM_INVALID;
		break;
	}
	return temp;
}

vector4d Graph4D::CalculateLocal(vector4d point)
{
	vector4d temp,temp2;
	temp2.x=point.x-camera->location.x;
	temp2.y=point.y-camera->location.y;
	temp2.z=point.z-camera->location.z;
	temp2.w=point.w-camera->location.w;
	temp.x=(temp2%camera->right)/camera->right.Len();
	temp.y=(temp2%camera->up)/camera->up.Len();
	temp.z=(temp2%camera->lookat)/camera->lookat.Len();
	temp.w=(temp2%camera->normal)/camera->normal.Len();
	return temp;
}

void Graph4D::SetBlending(double a)
{
	glEnable(GL_BLEND);
	glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
	if(a<0.99) glDepthMask(GL_FALSE);
	else 
	{
		glDisable(GL_BLEND);
		glDepthMask(GL_TRUE);
	}
}

void Graph4D::Render()
{
	primitive prim;

	glDepthMask(GL_TRUE);
	glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT);
	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
	gluLookAt(0.0,0.0,0.0,0.0,0.0,-1.0,0.0,1.0,0.0);

	glEnable(GL_DEPTH_TEST);
	float diffuseMat[]={1.0f,1.0f,1.0f,1.0f};
	glMaterialfv(GL_FRONT_AND_BACK,GL_AMBIENT_AND_DIFFUSE,diffuseMat);
	glEnable(GL_COLOR_MATERIAL);
	glColorMaterial(GL_FRONT_AND_BACK,GL_AMBIENT_AND_DIFFUSE);
	
	
	Space space;
	space.normal=camera->normal;
	space.e=-(space.normal%camera->location);
	unsigned long i,a=buffer->GetNumPrims();
	int j;

	local_buffer=new PrimBuffer;
	for(i=0;i<a;i++)
	{
		prim=Intersect(buffer->GetPrim(0),space);
		if(prim.type!=PRIM_NONE&&prim.type!=PRIM_INVALID)
			local_buffer->AddPrim(prim);
		buffer->DeletePrim(0);
	}

	a=local_buffer->GetNumPrims();
	for(i=0;i<a;i++)
	{
		prim=local_buffer->GetPrim(a-i-1);
		
		for(j=0;j<((prim.type==-1)?4:prim.type);j++)
			prim.vert[j].pt=CalculateLocal(prim.vert[j].pt);
		
		SetBlending(prim.vert[0].a);

		vector4d norm;
		switch(prim.type)
		{
		case PRIM_POINT:
			glBegin(GL_POINTS);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
			glEnd();
			break;
		case PRIM_LINE:
			glBegin(GL_LINES);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
				glColor4d(prim.vert[1].r,prim.vert[1].g,prim.vert[1].b,prim.vert[1].a);
				glVertex3d(prim.vert[1].pt.x,prim.vert[1].pt.y,prim.vert[1].pt.z);
			glEnd();
			break;
		case PRIM_TRIANGLE:
			glBegin(GL_TRIANGLES);
				norm=CrossProduct3(prim.vert[2].pt-prim.vert[0].pt,prim.vert[1].pt-prim.vert[0].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
				glColor4d(prim.vert[1].r,prim.vert[1].g,prim.vert[1].b,prim.vert[1].a);
				glVertex3d(prim.vert[1].pt.x,prim.vert[1].pt.y,prim.vert[1].pt.z);
				glColor4d(prim.vert[2].r,prim.vert[2].g,prim.vert[2].b,prim.vert[2].a);
				glVertex3d(prim.vert[2].pt.x,prim.vert[2].pt.y,prim.vert[2].pt.z);
			glEnd();
			break;
		case PRIM_TETRA:
			glBegin(GL_TRIANGLES);
				norm=CrossProduct3(prim.vert[1].pt-prim.vert[2].pt,prim.vert[0].pt-prim.vert[2].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
				glColor4d(prim.vert[1].r,prim.vert[1].g,prim.vert[1].b,prim.vert[1].a);
				glVertex3d(prim.vert[1].pt.x,prim.vert[1].pt.y,prim.vert[1].pt.z);
				glColor4d(prim.vert[2].r,prim.vert[2].g,prim.vert[2].b,prim.vert[2].a);
				glVertex3d(prim.vert[2].pt.x,prim.vert[2].pt.y,prim.vert[2].pt.z);
				norm=CrossProduct3(prim.vert[2].pt-prim.vert[1].pt,prim.vert[3].pt-prim.vert[1].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim.vert[1].r,prim.vert[1].g,prim.vert[1].b,prim.vert[1].a);
				glVertex3d(prim.vert[1].pt.x,prim.vert[1].pt.y,prim.vert[1].pt.z);;
				glColor4d(prim.vert[2].r,prim.vert[2].g,prim.vert[2].b,prim.vert[2].a);
				glVertex3d(prim.vert[2].pt.x,prim.vert[2].pt.y,prim.vert[2].pt.z);
				glColor4d(prim.vert[3].r,prim.vert[3].g,prim.vert[3].b,prim.vert[3].a);
				glVertex3d(prim.vert[3].pt.x,prim.vert[3].pt.y,prim.vert[3].pt.z);
				norm=CrossProduct3(prim.vert[0].pt-prim.vert[2].pt,prim.vert[3].pt-prim.vert[2].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim.vert[2].r,prim.vert[2].g,prim.vert[2].b,prim.vert[2].a);
				glVertex3d(prim.vert[2].pt.x,prim.vert[2].pt.y,prim.vert[2].pt.z);
				glColor4d(prim.vert[3].r,prim.vert[3].g,prim.vert[3].b,prim.vert[3].a);
				glVertex3d(prim.vert[3].pt.x,prim.vert[3].pt.y,prim.vert[3].pt.z);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
				norm=CrossProduct3(prim.vert[3].pt-prim.vert[1].pt,prim.vert[0].pt-prim.vert[1].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim.vert[3].r,prim.vert[3].g,prim.vert[3].b,prim.vert[3].a);
				glVertex3d(prim.vert[3].pt.x,prim.vert[3].pt.y,prim.vert[3].pt.z);
				glColor4d(prim.vert[0].r,prim.vert[0].g,prim.vert[0].b,prim.vert[0].a);
				glVertex3d(prim.vert[0].pt.x,prim.vert[0].pt.y,prim.vert[0].pt.z);
				glColor4d(prim.vert[1].r,prim.vert[1].g,prim.vert[1].b,prim.vert[1].a);
				glVertex3d(prim.vert[1].pt.x,prim.vert[1].pt.y,prim.vert[1].pt.z);
			glEnd();
			break;
		case PRIM_QUAD:
			vector4d v[3];
			int vakt;
			double scal[3];
			primitive prim2;
			v[0]=prim.vert[1].pt-prim.vert[0].pt;
			v[1]=prim.vert[2].pt-prim.vert[0].pt;
			v[2]=prim.vert[3].pt-prim.vert[0].pt;
			scal[0]=(v[1]%v[0])/(v[1].Len()*v[0].Len());
			scal[1]=(v[2]%v[0])/(v[2].Len()*v[0].Len());
			scal[2]=(v[2]%v[1])/(v[2].Len()*v[1].Len());
			vakt=(scal[0]<scal[1])?0:1;
			vakt=(scal[2]<scal[vakt])?2:vakt;
			prim2.vert[0]=prim.vert[0];
			switch(vakt)
			{
			case 0:
				prim2.vert[1]=prim.vert[1];
				prim2.vert[2]=prim.vert[2];
				prim2.vert[3]=prim.vert[3];
				break;
			case 1:
				prim2.vert[1]=prim.vert[1];
				prim2.vert[2]=prim.vert[3];
				prim2.vert[3]=prim.vert[2];
				break;
			case 2:
				prim2.vert[1]=prim.vert[2];
				prim2.vert[2]=prim.vert[3];
				prim2.vert[3]=prim.vert[1];
				break;
			}
			glBegin(GL_TRIANGLE_STRIP);
				norm=CrossProduct3(prim2.vert[2].pt-prim2.vert[0].pt,prim2.vert[1].pt-prim2.vert[0].pt);
				norm.Normalize();
				glNormal3d(norm.x,norm.y,norm.z);
				glColor4d(prim2.vert[0].r,prim2.vert[0].g,prim2.vert[0].b,prim.vert[0].a);
				glVertex3d(prim2.vert[0].pt.x,prim2.vert[0].pt.y,prim2.vert[0].pt.z);
				glColor4d(prim2.vert[1].r,prim2.vert[1].g,prim2.vert[1].b,prim.vert[1].a);
				glVertex3d(prim2.vert[1].pt.x,prim2.vert[1].pt.y,prim2.vert[1].pt.z);
				glColor4d(prim2.vert[2].r,prim2.vert[2].g,prim2.vert[2].b,prim.vert[2].a);
				glVertex3d(prim2.vert[2].pt.x,prim2.vert[2].pt.y,prim2.vert[2].pt.z);
				glColor4d(prim2.vert[3].r,prim2.vert[3].g,prim2.vert[3].b,prim.vert[3].a);
				glVertex3d(prim2.vert[3].pt.x,prim2.vert[3].pt.y,prim2.vert[3].pt.z);
			glEnd();
			break;
		}
	}
	delete local_buffer;
	local_buffer=NULL;
	m_buffer->LoadIdentity();
	camera->LoadIdentity();
	SwapBuffers(hdc);
}

void Graph4D::HandleWMSize(LPARAM lparam)
{
	int width,height;
	width=LOWORD(lparam);
	height=HIWORD(lparam);
	if(height==0) height=1;
	glViewport(0,0,width,height);
	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	gluPerspective(45.0f,(GLfloat)width/(GLfloat)height,1.0f,1000.0f);
	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
}

void Graph4D::Color(double r1,double g1,double b1)
{
	r=r1;
	g=g1;
	b=b1;
	a=1.0;
}

void Graph4D::ColorA(double r1,double g1,double b1,double a1)
{
	r=r1;
	g=g1;
	b=b1;
	a=a1;
}

void Graph4D::Point(point4d pt1)
{
	primitive prim;
	prim.type=PRIM_POINT;
	prim.vert[0].r=r;
	prim.vert[0].g=g;
	prim.vert[0].b=b;
	prim.vert[0].a=a;
	prim.vert[0].pt=pt1;
	AddPrimitive(prim);
}

void Graph4D::Line(point4d pt1,point4d pt2)
{
	primitive prim;
	prim.type=PRIM_LINE;
	prim.vert[0].r=r;
	prim.vert[0].g=g;
	prim.vert[0].b=b;
	prim.vert[0].a=a;
	prim.vert[1].r=r;
	prim.vert[1].g=g;
	prim.vert[1].b=b;
	prim.vert[1].a=a;
	prim.vert[0].pt=pt1;
	prim.vert[1].pt=pt2;
	AddPrimitive(prim);
}

void Graph4D::Triangle(point4d pt1,point4d pt2,point4d pt3)
{
	primitive prim;
	prim.type=PRIM_TRIANGLE;
	prim.vert[0].r=r;
	prim.vert[0].g=g;
	prim.vert[0].b=b;
	prim.vert[0].a=a;
	prim.vert[1].r=r;
	prim.vert[1].g=g;
	prim.vert[1].b=b;
	prim.vert[1].a=a;
	prim.vert[2].r=r;
	prim.vert[2].g=g;
	prim.vert[2].b=b;
	prim.vert[2].a=a;
	prim.vert[0].pt=pt1;
	prim.vert[1].pt=pt2;
	prim.vert[2].pt=pt3;
	AddPrimitive(prim);
}

void Graph4D::Tetrahedron(point4d pt1,point4d pt2,point4d pt3,point4d pt4)
{
	primitive prim;
	prim.type=PRIM_TETRA;
	prim.vert[0].r=r;
	prim.vert[0].g=g;
	prim.vert[0].b=b;
	prim.vert[0].a=a;
	prim.vert[1].r=r;
	prim.vert[1].g=g;
	prim.vert[1].b=b;
	prim.vert[1].a=a;
	prim.vert[2].r=r;
	prim.vert[2].g=g;
	prim.vert[2].b=b;
	prim.vert[2].a=a;
	prim.vert[3].r=r;
	prim.vert[3].g=g;
	prim.vert[3].b=b;
	prim.vert[3].a=a;
	prim.vert[0].pt=pt1;
	prim.vert[1].pt=pt2;
	prim.vert[2].pt=pt3;
	prim.vert[3].pt=pt4;
	AddPrimitive(prim);
}

void Graph4D::PointVertex(vertex4d ver1)
{
	primitive prim;
	prim.type=PRIM_POINT;
	prim.vert[0]=ver1;
	AddPrimitive(prim);
}

void Graph4D::LineVertex(vertex4d ver1,vertex4d ver2)
{
	primitive prim;
	prim.type=PRIM_LINE;
	prim.vert[0]=ver1;
	prim.vert[1]=ver2;
	AddPrimitive(prim);
}

void Graph4D::TriangleVertex(vertex4d ver1,vertex4d ver2,vertex4d ver3)
{
	primitive prim;
	prim.type=PRIM_TRIANGLE;
	prim.vert[0]=ver1;
	prim.vert[1]=ver2;
	prim.vert[2]=ver3;
	AddPrimitive(prim);
}

void Graph4D::TetrahedronVertex(vertex4d ver1,vertex4d ver2,vertex4d ver3,vertex4d ver4)
{
	primitive prim;
	prim.type=PRIM_TETRA;
	prim.vert[0]=ver1;
	prim.vert[1]=ver2;
	prim.vert[2]=ver3;
	prim.vert[3]=ver4;
	AddPrimitive(prim);
}

void Graph4D::EnableLighting(bool b)
{
	if(b)
		glEnable(GL_LIGHTING);
	else
		glDisable(GL_LIGHTING);
}

void Graph4D::EnableTwoSide(bool b)
{
	glLightModeli(GL_LIGHT_MODEL_TWO_SIDE,b?GL_TRUE:GL_FALSE);
}

void Graph4D::LightDir(double x,double y,double z)
{
	float p[4];
	p[3]=0.0f;
	p[0]=(float)x;
	p[1]=(float)y;
	p[2]=(float)z;
	glLightfv(GL_LIGHT0,GL_POSITION,p);
}

//drawing functions

void Graph4D::Cube(double bok)
{
	vector4d ver[8];
	ver[0]=vector4d(-0.5*bok,-0.5*bok,-0.5*bok,0.0);
	ver[1]=vector4d(0.5*bok,-0.5*bok,-0.5*bok,0.0);
	ver[2]=vector4d(0.5*bok,-0.5*bok,0.5*bok,0.0);
	ver[3]=vector4d(-0.5*bok,-0.5*bok,0.5*bok,0.0);
	ver[4]=vector4d(-0.5*bok,0.5*bok,-0.5*bok,0.0);
	ver[5]=vector4d(0.5*bok,0.5*bok,-0.5*bok,0.0);
	ver[6]=vector4d(0.5*bok,0.5*bok,0.5*bok,0.0);
	ver[7]=vector4d(-0.5*bok,0.5*bok,0.5*bok,0.0);
	Tetrahedron(ver[0],ver[4],ver[3],ver[1]);
	Tetrahedron(ver[2],ver[6],ver[1],ver[3]);
	Tetrahedron(ver[5],ver[1],ver[6],ver[4]);
	Tetrahedron(ver[7],ver[3],ver[4],ver[6]);
	Tetrahedron(ver[4],ver[6],ver[3],ver[1]);
}

#define CUBE(a,b,c,d,e,f,g,h) Tetrahedron(ver[a],ver[c],ver[b],ver[e]); \
	Tetrahedron(ver[f],ver[h],ver[e],ver[b]); \
	Tetrahedron(ver[d],ver[b],ver[c],ver[h]); \
	Tetrahedron(ver[g],ver[e],ver[h],ver[c]); \
	Tetrahedron(ver[c],ver[h],ver[b],ver[e]);

void Graph4D::Tesseract(double bok)
{
	vector4d ver[16];
	ver[0]=vector4d(-0.5*bok,-0.5*bok,-0.5*bok,-0.5*bok);
	ver[1]=vector4d(-0.5*bok,-0.5*bok,-0.5*bok,0.5*bok);
	ver[2]=vector4d(-0.5*bok,-0.5*bok,0.5*bok,-0.5*bok);
	ver[3]=vector4d(-0.5*bok,-0.5*bok,0.5*bok,0.5*bok);
	ver[4]=vector4d(-0.5*bok,0.5*bok,-0.5*bok,-0.5*bok);
	ver[5]=vector4d(-0.5*bok,0.5*bok,-0.5*bok,0.5*bok);
	ver[6]=vector4d(-0.5*bok,0.5*bok,0.5*bok,-0.5*bok);
	ver[7]=vector4d(-0.5*bok,0.5*bok,0.5*bok,0.5*bok);
	ver[8]=vector4d(0.5*bok,-0.5*bok,-0.5*bok,-0.5*bok);
	ver[9]=vector4d(0.5*bok,-0.5*bok,-0.5*bok,0.5*bok);
	ver[10]=vector4d(0.5*bok,-0.5*bok,0.5*bok,-0.5*bok);
	ver[11]=vector4d(0.5*bok,-0.5*bok,0.5*bok,0.5*bok);
	ver[12]=vector4d(0.5*bok,0.5*bok,-0.5*bok,-0.5*bok);
	ver[13]=vector4d(0.5*bok,0.5*bok,-0.5*bok,0.5*bok);
	ver[14]=vector4d(0.5*bok,0.5*bok,0.5*bok,-0.5*bok);
	ver[15]=vector4d(0.5*bok,0.5*bok,0.5*bok,0.5*bok);
	CUBE(0,1,2,3,4,5,6,7)
	CUBE(8,9,10,11,12,13,14,15)
	CUBE(0,1,2,3,8,9,10,11)
	CUBE(4,5,6,7,12,13,14,15)
	CUBE(0,1,4,5,8,9,12,13)
	CUBE(2,3,6,7,10,11,14,15)
	CUBE(0,2,4,6,8,10,12,14)
	CUBE(1,3,5,7,9,11,13,15)
}

//------------------------------------DllMain-------------------------------------

//actually, I have no idea why this is here - I probably forgot to remove it
BOOL WINAPI DllMain(HANDLE,DWORD,LPVOID)
{
	return 1;
}

//------------------------------------helpers----------------------------------

vector4d CrossProduct3(vector4d arg1,vector4d arg2)
{
	vector4d temp;
	temp.x=(arg1.y*arg2.z)-(arg1.z*arg2.y);
	temp.y=(arg1.z*arg2.x)-(arg1.x*arg2.z);
	temp.z=(arg1.x*arg2.y)-(arg1.y*arg2.x);
	return temp;
}

vector4d CrossProduct4(vector4d arg1,vector4d arg2,vector4d arg3)
{
	vector4d temp;
	temp.x=(arg1.y*arg2.z*arg3.w)+(arg1.z*arg2.w*arg3.y)+(arg1.w*arg2.y*arg3.z)-(arg1.y*arg2.w*arg3.z)-(arg1.z*arg2.y*arg3.w)-(arg1.w*arg2.z*arg3.y);
	temp.y=(arg1.z*arg2.w*arg3.x)+(arg1.w*arg2.x*arg3.z)+(arg1.x*arg2.z*arg3.w)-(arg1.z*arg2.x*arg3.w)-(arg1.w*arg2.z*arg3.x)-(arg1.x*arg2.w*arg3.z);
	temp.z=(arg1.w*arg2.x*arg3.y)+(arg1.x*arg2.y*arg3.w)+(arg1.y*arg2.w*arg3.x)-(arg1.w*arg2.y*arg3.x)-(arg1.x*arg2.w*arg3.y)-(arg1.y*arg2.x*arg3.w);
	temp.w=(arg1.x*arg2.y*arg3.z)+(arg1.y*arg2.z*arg3.x)+(arg1.z*arg2.x*arg3.y)-(arg1.x*arg2.z*arg3.y)-(arg1.y*arg2.x*arg3.z)-(arg1.z*arg2.y*arg3.x);
	return temp;
}

vertex4d Vertex(double x,double y,double z,double w,double r,double g,double b)
{
	vertex4d temp;
	temp.pt=vector4d(x,y,z,w);
	temp.r=r;
	temp.g=g;
	temp.b=b;
	temp.a=1.0;
	return temp;
}

vertex4d VertexA(double x,double y,double z,double w,double r,double g,double b,double a)
{
	vertex4d temp;
	temp.pt=vector4d(x,y,z,w);
	temp.r=r;
	temp.g=g;
	temp.b=b;
	temp.a=a;
	return temp;
}

matrix4d RotationMatrix(vector4d n1,vector4d n2,double phi)
{
	n1.Normalize();
	n2.Normalize();
	n2=n2-(n1*(n1%n2));
	n2.Normalize();
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,(n1.x*n1.x+n2.x*n2.x)*(1.0-cos(phi))+cos(phi));
	mat.SetValue(1,0,(n1.x*n1.y+n2.x*n2.y)*(1.0-cos(phi))+(n1.z*n2.w-n1.w*n2.z)*sin(phi));
	mat.SetValue(2,0,(n1.x*n1.z+n2.x*n2.z)*(1.0-cos(phi))-(n1.y*n2.w-n1.w*n2.y)*sin(phi));
	mat.SetValue(3,0,(n1.x*n1.w+n2.x*n2.w)*(1.0-cos(phi))+(n1.y*n2.z-n1.z*n2.y)*sin(phi));
	mat.SetValue(0,1,(n1.y*n1.x+n2.y*n2.x)*(1.0-cos(phi))-(n1.z*n2.w-n1.w*n2.z)*sin(phi));
	mat.SetValue(1,1,(n1.y*n1.y+n2.y*n2.y)*(1.0-cos(phi))+cos(phi));
	mat.SetValue(2,1,(n1.y*n1.z+n2.y*n2.z)*(1.0-cos(phi))+(n1.x*n2.w-n1.w*n2.x)*sin(phi));
	mat.SetValue(3,1,(n1.y*n1.w+n2.y*n2.w)*(1.0-cos(phi))-(n1.x*n2.z-n1.z*n2.x)*sin(phi));
	mat.SetValue(0,2,(n1.z*n1.x+n2.z*n2.x)*(1.0-cos(phi))+(n1.y*n2.w-n1.w*n2.y)*sin(phi));
	mat.SetValue(1,2,(n1.z*n1.y+n2.z*n2.y)*(1.0-cos(phi))-(n1.x*n2.w-n1.w*n2.x)*sin(phi));
	mat.SetValue(2,2,(n1.z*n1.z+n2.z*n2.z)*(1.0-cos(phi))+cos(phi));
	mat.SetValue(3,2,(n1.z*n1.w+n2.z*n2.w)*(1.0-cos(phi))+(n1.x*n2.y-n1.y*n2.x)*sin(phi));
	mat.SetValue(0,3,(n1.w*n1.x+n2.w*n2.x)*(1.0-cos(phi))-(n1.y*n2.z-n1.z*n2.y)*sin(phi));
	mat.SetValue(1,3,(n1.w*n1.y+n2.w*n2.y)*(1.0-cos(phi))+(n1.x*n2.z-n1.z*n2.x)*sin(phi));
	mat.SetValue(2,3,(n1.w*n1.z+n2.w*n2.z)*(1.0-cos(phi))-(n1.x*n2.y-n1.y*n2.x)*sin(phi));
	mat.SetValue(3,3,(n1.w*n1.w+n2.w*n2.w)*(1.0-cos(phi))+cos(phi));
	return mat;
}

matrix4d TranslationMatrix(vector4d arg)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,4,arg.x);
	mat.SetValue(1,4,arg.y);
	mat.SetValue(2,4,arg.z);
	mat.SetValue(3,4,arg.w);
	return mat;
}

matrix4d ScaleMatrix(double x,double y,double z,double w)
{
	matrix4d mat;
	mat.LoadIdentity();
	mat.SetValue(0,0,x);
	mat.SetValue(1,1,y);
	mat.SetValue(2,2,z);
	mat.SetValue(3,3,w);
	return mat;
}