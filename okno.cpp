#include "okno.h"

Rysowanie::Rysowanie(Graph4D* g):DrawThread(g)
{
	t = new QTime;
	t->start();
}

void Rysowanie::run()
{
	while(1)
	{
		int t1 = t->elapsed();
	
		matrix4d mat = RotationMatrix(vector4d(0,0,1.0,0),vector4d(0,0,0,1.0),(double)(t1%2000)/1000.0*pi);
		matrix4d mat2 = TranslationMatrix(vector4d(0,0,4.0,0));
		graph->PushMatrix();
		graph->ApplyMatrix(mat2*mat);
		graph->Color(1.0,1.0,0);
		graph->Tesseract(2.0);
		graph->PopMatrix();
	
		emit updateGL();
	}
}

Okno::Okno()
{
	resize(800,600);
	graph = new Graph4D(this);
	graph->move(0,0);
	graph->resize(800,600);
	show();
	
	th = new Rysowanie(graph);
	th->start();
}

Okno::~Okno()
{
	th->terminate();
	delete th;
}
