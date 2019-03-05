#include "stdafx.h"
#include <iostream>
#include <unordered_set>

using namespace std;

typedef pair<int, int> P;

template <class T> struct MyHash;
template<> struct MyHash<P>
{
	std::size_t operator()(P const& p) const
	{
		return p.first + 4000 * p.second;
	}
};

typedef unordered_set<P, MyHash<P>> Set;

template<typename F>
void iterNeighbors(F f, P p) {
	f(make_pair(p.first - 1, p.second));
	f(make_pair(p.first + 1, p.second));
	f(make_pair(p.first, p.second - 1));
	f(make_pair(p.first, p.second + 1));
}

Set nthLoop(int n, Set s1, Set s2) {
	if (n == 0) return s1;
	Set s0;
	for (const auto& p : s1) {
		auto add = [&s0, &s1, &s2](P p) {
			if (s1.find(p) == s1.end() && s2.find(p) == s2.end())
				s0.insert(p);
		};
		iterNeighbors(add, p);
	}
	return nthLoop(n-1, s0, s1);
}

Set nth(int n, P p) {
	Set s1;
	s1.insert(p);
	return nthLoop(n, s1, Set());
}

int _tmain(int argc, _TCHAR* argv[])
{
	cout << nth(2000, make_pair(0, 0)).size();

	return 0;
}
