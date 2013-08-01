//#include "obiekty.h"
#include "Graph4D.h"
//#include "levels.h"
#include "okno.h"
#include <QApplication>

//LKolejka* kol;
//LPlayer* player;

short fps=0;

#define VK_Q 81
#define VK_W 87
#define VK_E 69
#define VK_A 65
#define VK_S 83
#define VK_D 68
#define VK_Z 90
#define VK_X 88
#define VK_T 84
#define VK_G 71
#define VK_F 70
#define VK_H 72
#define VK_R 82
#define VK_Y 89
#define VK_C 67
#define VK_V 86
#define VK_B 66
#define VK_N 78
#define VK_U 85
#define VK_J 74

/*LRESULT CALLBACK wndproc2(HWND hwnd,UINT msg,WPARAM wparam,LPARAM lparam)
{
	switch(msg)
	{
	case WM_CREATE:
		CreateWindowEx(0,"STATIC","FPS: 0",WS_CHILD|WS_VISIBLE,10,10,180,25,hwnd,(HMENU)1,GetModuleHandle(NULL),0);
		SetTimer(hwnd,128,1000,NULL);
		break;
	case WM_TIMER:
		if(wparam==128)
		{
			char pom[100];
			sprintf(pom,"FPS: %d",fps);
			SetWindowText(GetDlgItem(hwnd,1),pom);
			fps=0;
		}
		break;
	case WM_DESTROY:
		oknofps=NULL;
		if(okno!=NULL) DestroyWindow(okno);
		PostQuitMessage(0);
		break;
	default:
		return DefWindowProc(hwnd,msg,wparam,lparam);
	}
	return 0;
}

LRESULT CALLBACK wndproc(HWND hwnd,UINT msg,WPARAM wparam,LPARAM lparam)
{
	switch(msg)
	{
	case WM_CREATE:
		graph=new Graph4D(GetDC(hwnd));
		graph->EnableTwoSide(true);
		graph->LightDir(0.0,-0.6,-0.8);
		LoadLevel(1);
		break;
	case WM_DESTROY:
		delete graph;
		graph=NULL;
		okno=NULL;
		if(oknofps!=NULL) DestroyWindow(oknofps);
		PostQuitMessage(0);
		break;
	case WM_SIZE:
		if(graph!=NULL) graph->HandleWMSize(lparam);
		break;
	default:
		return DefWindowProc(hwnd,msg,wparam,lparam);
	}
	return 0;
}

long t;

void Sterowanie()
{
	double k=3.2;
	double r=pi/3;
	k*=(GetTickCount()-t)/1000.0;
	r*=(GetTickCount()-t)/1000.0;

//----------------------------klawisze ruchu

	if(GetAsyncKeyState(VK_W))
	{
		player->Go(vector4d(0.0,0.0,k,0.0));
		if(kol->kolizja()) player->Go(vector4d(0.0,0.0,-k,0.0));
	}
	if(GetAsyncKeyState(VK_S))
	{
		player->Go(vector4d(0.0,0.0,-k,0.0));
		if(kol->kolizja()) player->Go(vector4d(0.0,0.0,k,0.0));
	}
	if(GetAsyncKeyState(VK_A))
	{
		player->Go(vector4d(-k,0.0,0.0,0.0));
		if(kol->kolizja()) player->Go(vector4d(k,0.0,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_D))
	{
		player->Go(vector4d(k,0.0,0.0,0.0));
		if(kol->kolizja()) player->Go(vector4d(-k,0.0,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_Q))
	{
		player->Go(vector4d(0.0,k,0.0,0.0));
		if(kol->kolizja()) player->Go(vector4d(0.0,-k,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_E))
	{
		player->Go(vector4d(0.0,-k,0.0,0.0));
		if(kol->kolizja()) player->Go(vector4d(0.0,k,0.0,0.0));
	}
	/if(GetAsyncKeyState(VK_Z))
	{
		player->Go(vector4d(0.0,0.0,0.0,k));
		if(kol->kolizja()) player->Go(vector4d(0.0,0.0,0.0,-k));
	}
	if(GetAsyncKeyState(VK_X))
	{
		player->Go(vector4d(0.0,0.0,0.0,-k));
		if(kol->kolizja()) player->Go(vector4d(0.0,0.0,0.0,k));
	}/
//------------------------koniec klawiszy ruchu

//------------------------klawisze obrotu
	if(GetAsyncKeyState(VK_T))
		player->RotateXW(r);
	if(GetAsyncKeyState(VK_G))
		player->RotateXW(-r);
	if(GetAsyncKeyState(VK_F))
		player->RotateZW(-r);
	if(GetAsyncKeyState(VK_H))
		player->RotateZW(r);
	if(GetAsyncKeyState(VK_R))
		player->RotateYW(-r);
	if(GetAsyncKeyState(VK_Y))
		player->RotateYW(r);
	if(GetAsyncKeyState(VK_U))
		player->RotateXY(-r);
	if(GetAsyncKeyState(VK_J))
		player->RotateXY(r);
	if(GetAsyncKeyState(VK_C))
		player->RotateXZ(r);
	if(GetAsyncKeyState(VK_V))
		player->RotateXZ(-r);
	if(GetAsyncKeyState(VK_B))
		player->RotateYZ(r);
	if(GetAsyncKeyState(VK_N))
		player->RotateYZ(-r);
//------------------------koniec klawiszy obrotu
}*/

int main(int argc,char** argv)
{
	QApplication app(argc,argv);
		/*if(graph!=NULL)
		{
			Sterowanie();
			t=GetTickCount();
			kol->wykonaj();
			graph->Render();
			fps++;
		}*/
	Okno* o = new Okno;
	o->show();
	return app.exec();
}
