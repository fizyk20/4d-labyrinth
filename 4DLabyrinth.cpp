#include "objects.h"
#include "Graph4D.h"
#include "levels.h"
#include <stdio.h>
#pragma comment(lib,"Graph4D.lib")
#pragma comment(lib,"opengl32.lib")
#pragma comment(lib,"glu32.lib")

Graph4D* graph;
LQueue* queue;
LPlayer* player;

int level;
bool winner;
HWND winfps,win;
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

LRESULT CALLBACK wndproc2(HWND hwnd,UINT msg,WPARAM wparam,LPARAM lparam)
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
		winfps=NULL;
		if(win!=NULL) DestroyWindow(win);
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
		level=1;
		break;
	case WM_DESTROY:
		delete graph;
		graph=NULL;
		win=NULL;
		if(winfps!=NULL) DestroyWindow(winfps);
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

void Controls()
{
	double k=3.2;
	double r=pi/3;
	k*=(GetTickCount()-t)/1000.0;
	r*=(GetTickCount()-t)/1000.0;

//----------------------------movement keys

	if(GetAsyncKeyState(VK_W))
	{
		player->Go(vector4d(0.0,0.0,k,0.0));
		if(queue->collision()) player->Go(vector4d(0.0,0.0,-k,0.0));
	}
	if(GetAsyncKeyState(VK_S))
	{
		player->Go(vector4d(0.0,0.0,-k,0.0));
		if(queue->collision()) player->Go(vector4d(0.0,0.0,k,0.0));
	}
	if(GetAsyncKeyState(VK_A))
	{
		player->Go(vector4d(-k,0.0,0.0,0.0));
		if(queue->collision()) player->Go(vector4d(k,0.0,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_D))
	{
		player->Go(vector4d(k,0.0,0.0,0.0));
		if(queue->collision()) player->Go(vector4d(-k,0.0,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_Q))
	{
		player->Go(vector4d(0.0,k,0.0,0.0));
		if(queue->collision()) player->Go(vector4d(0.0,-k,0.0,0.0));
	}
	if(GetAsyncKeyState(VK_E))
	{
		player->Go(vector4d(0.0,-k,0.0,0.0));
		if(queue->collision()) player->Go(vector4d(0.0,k,0.0,0.0));
	}
	/*if(GetAsyncKeyState(VK_Z))
	{
		player->Go(vector4d(0.0,0.0,0.0,k));
		if(queue->collision()) player->Go(vector4d(0.0,0.0,0.0,-k));
	}
	if(GetAsyncKeyState(VK_X))
	{
		player->Go(vector4d(0.0,0.0,0.0,-k));
		if(queue->collision()) player->Go(vector4d(0.0,0.0,0.0,k));
	}*/
//------------------------movement keys end

//------------------------rotation keys
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
//------------------------rotation keys end
}

int WINAPI WinMain(HINSTANCE hInst,HINSTANCE,LPSTR,int)
{
	winner=false;

	WNDCLASSEX wnd;

	wnd.cbClsExtra=0;
	wnd.cbSize=sizeof(WNDCLASSEX);
	wnd.cbWndExtra=0;
	wnd.hbrBackground=NULL;
	wnd.hCursor=LoadCursor(NULL,IDC_ARROW);
	wnd.hIcon=wnd.hIconSm=LoadIcon(NULL,IDI_APPLICATION);
	wnd.hInstance=hInst;
	wnd.lpfnWndProc=wndproc;
	wnd.lpszClassName="klasa";
	wnd.lpszMenuName=NULL;
	wnd.style=CS_VREDRAW|CS_HREDRAW;

	RegisterClassEx(&wnd);
	win=CreateWindowEx(0,"klasa","4D Labyrinth",WS_OVERLAPPEDWINDOW,0,0,960,540,0,0,hInst,0);
	ShowWindow(win,SW_SHOW);

	//FPS Rate
	wnd.lpfnWndProc=wndproc2;
	wnd.lpszClassName="klasa2";
	wnd.hbrBackground=CreateSolidBrush(RGB(255,255,255));
	wnd.style=0;
	RegisterClassEx(&wnd);
	winfps=CreateWindowEx(0,"klasa2","Labirynt4D - FPS",WS_OVERLAPPEDWINDOW,970,0,200,80,0,0,hInst,0);
	ShowWindow(winfps,SW_SHOW);

	MSG msg;

	t=GetTickCount();

	while(1)
	{
		if(PeekMessage(&msg,0,0,0,PM_REMOVE))
		{
			if(msg.message==WM_QUIT) break;
			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}
		if(graph!=NULL)
		{
			Controls();
			t=GetTickCount();
			queue->doYourJob();
			graph->Render();
			fps++;
			if(queue->win())
			{
				delete queue;
				level++;
				if(!LoadLevel(level))
				{
					winner=true;
					DestroyWindow(win);
				}
				else 
				{
					char next[20];
					sprintf(next,"Prepare for level %d",level);
					MessageBox(win,next,"Next level",MB_ICONINFORMATION);
				}
			}
		}
	}

	if(winner) MessageBox(NULL,"You won!","Congratulations",MB_ICONINFORMATION);

	return(0);
}