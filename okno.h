#ifndef __LAB4DOKNO__
#define __LAB4DOKNO__

#include <QtGui>
#include "Graph4D.h"

class Rysowanie : public DrawThread
{
Q_OBJECT
	QTime* t;
public:
	Rysowanie(Graph4D*);
	void run();
};

class Okno : public QWidget
{
Q_OBJECT
	Graph4D* graph;
	Rysowanie* th;
public:
	Okno();
	~Okno();
};

#endif
